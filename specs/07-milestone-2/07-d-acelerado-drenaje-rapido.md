# Spec 07-d — Throttle: drenaje de cola siempre activo + hard-stop ENZ

> Milestone 2 · Referencia: Protocolo v4.3 §7 + ADR-015 punto (4) + ADR-016 · Prioridad: **media** · Quick win
> Dependencias upstream: 07-b (cap diario).
> Habilita: nada bloqueante — cierra el subsistema Throttle.

---

## 1. Problema (qué hay hoy)

`contracts/playground/lib.rs:343` — la línea `throttle_burn_bps(mode).min(BPS_DENOMINATOR)`
capa el 125% del modo ACELERADO a 100%. Como el tramo LP del Motor B en una tx no
puede quemar **más de lo que existe en esa tx** (no hay un buffer del que sacar el
25% extra), el modo ACELERADO ejecuta **exactamente lo mismo que NORMAL**:

- Quema del tramo LP: 100% en ambos modos.
- Drenaje de la cola diferida: 10%/semana en ambos modos.

Es decir, la palanca ACELERADO existe en las constantes y en la máquina de estados
pero **no produce ningún efecto observable en la contabilidad ni en la deflación**.

**Origen de la inconsistencia (arqueología):** el 125% original de v4.2 §7 asumía
que el 25% extra salía de la R_op ("Consumo R_op: al 100% uso máximo"). La corrección
v4.2 §4.3 dejó la R_op **idle** ("no interviene en el flujo normal") — eso dejó al
125% sin fuente y quedó como palanca huérfana. Registrado en el análisis de hallazgos
sesión 5.

## 2. Diseño

### 2.1 Semántica aprobada por Sebastián (2026-08-21)

**ACELERADO = "quemar más rápido"** = **acelerar el drenaje de la cola diferida**,
no inventar tokens extra por tx. Interpretación económica coherente con la R_op idle
y con la intuición del arquitecto ("en euforia, quemo el backlog más rápido").

### 2.2 Cambio concreto — la quema NUNCA se detiene (hasta ENZ)

| Modo Throttle | Quema tramo LP tx | Drenaje cola/semana | Antes |
| --- | --- | --- | --- |
| ACELERADO (P > 1.2× EMA30) | 100% | **25%/sem** | 10%/sem |
| NORMAL (0.8× ≤ P ≤ 1.2×) | 100% | **10%/sem** | 10%/sem |
| CONSERVADOR (0.5× ≤ P < 0.8×) | 60% + 40% a cola | **5%/sem** ⬆ | *no drenaba* |
| DEFENSIVO (P < 0.5×) | 25% + 75% a cola | **2%/sem** ⬆ | *no drenaba* |
| **POST-ENZ** (supply ≤ 3.3B) | 0% (todo a Vault) | **0%** (cola congelada) | n/a |

**Escala geométrica memorable:** 25 / 10 / 5 / 2 (aprox. ÷2.5 por nivel).

**Orden monótono limpio** de intensidad de deflación:
`POST-ENZ < DEF < CONS < NORMAL < ACELERADO` ✅ (antes NORMAL ≡ ACEL, y CONS/DEF ≡ 0).

**Promesa a la comunidad (ADR-016):** "La quema nunca se detiene hasta que el protocolo
cumple su misión deflacionaria (supply → 3.3B). Es verificable on-chain en cualquier
momento."

### 2.3 Hard-stop ENZ (supply ≤ 3.3B)

Cuando el supply total alcanza o cruza el umbral ENZ (3,300,000,000 tokens):

1. **`process_fee`**: el tramo LP ya NO se quema ni se encola — va 100% al Vault.
2. **`execute_deferred_burn`**: rechaza la ejecución (`BurnComplete`). La cola se
   congela con el saldo que tenga — **el supply nunca baja de 3.3B**.
3. El Throttle sigue calculando el modo (para otros usos post-ENZ en Milestone 3),
   pero no actúa sobre quema.
4. La transición es **irreversible**: una vez `supply ≤ SUPPLY_ENZ`, el protocolo
   nunca vuelve a quemar.

### 2.4 Por qué drenar SIEMPRE es seguro (y óptimo)

- **Nunca quema más tokens de los que existen en la cola.** `deferred_burn_queue` es
  una cuenta finita — el drenaje es una fracción de ella.
- **El cap 1%/día (07-b) sigue operando:** si el drenaje semanal excede el cap diario
  remanente, el exceso **se queda en cola** (compone con 07-b limpiamente).
- **En DEFENSIVO, el 2% es económicamente óptimo:** el protocolo compra $LUKA barato
  para quemar → reduce más supply por USD → cuando el mercado recupera, precio sube
  más. Es "protocol-level buy-the-dip".
- **Anti-acumulación patológica:** sin drenaje en CONS/DEF, un bear extendido de 2
  años haría que la cola creciera indefinidamente. Con 5%/2%, siempre drena algo.
- **El hard-stop ENZ protege contra quema excesiva** en el extremo opuesto: por
  agresivo que sea el drenaje, nunca lleva el supply debajo de 3.3B.
- **No requiere buffer de tokens ni fuente externa** — solo cambia el ritmo.

---

## 3. Contrato — interfaces

### 3.1 Constantes nuevas (`constants.rs`)

```rust
// Drenaje de la cola diferida en bps (escala geométrica ÷2.5):
pub const QUEUE_DRAIN_ACCEL_BPS: u64 = 2_500;  // 25%/sem — euforia
pub const QUEUE_DRAIN_NORMAL_BPS: u64 = 1_000; // 10%/sem — estable
pub const QUEUE_DRAIN_CONS_BPS: u64   =   500; //  5%/sem — bajista
pub const QUEUE_DRAIN_DEF_BPS: u64    =   200; //  2%/sem — depresión

// Hard-stop de quema (supply mínimo absoluto)
pub const SUPPLY_ENZ: u64 = 3_300_000_000_000_000; // 3.3B × 10^6 decimals
```

`QUEUE_DRAIN_BPS` (el nombre viejo) se **deprecia**: sustituir todas las referencias
por `QUEUE_DRAIN_NORMAL_BPS` (mismo valor, mejor nombre) para que no haya ambigüedad.

### 3.2 Instrucción `execute_deferred_burn` — modificación

```rust
pub fn execute_deferred_burn(ctx: Context<ExecuteDeferredBurn>) -> Result<()> {
    require!(!ctx.accounts.config.paused, LukashError::ProtocolPaused);
    let state = &mut ctx.accounts.state;

    // HARD-STOP ENZ: si el supply ya llegó al mínimo, la quema se apaga definitivamente
    require!(
        state.current_supply > SUPPLY_ENZ,
        LukashError::BurnComplete
    );

    require!(state.deferred_burn_queue > 0, LukashError::EmptyQueue);
    let now = Clock::get()?.unix_timestamp;
    require!(
        now.checked_sub(state.last_queue_exec_ts).unwrap_or(0) >= WEEK_SECONDS,
        LukashError::QueueCooldown
    );

    // Drenaje según modo Throttle — TODOS los modos drenan (ADR-016)
    let drain_bps = match state.throttle_mode {
        THROTTLE_ACCELERATED  => QUEUE_DRAIN_ACCEL_BPS,  // 25%
        THROTTLE_NORMAL       => QUEUE_DRAIN_NORMAL_BPS, // 10%
        THROTTLE_CONSERVATIVE => QUEUE_DRAIN_CONS_BPS,   //  5%
        THROTTLE_DEFENSIVE    => QUEUE_DRAIN_DEF_BPS,    //  2%
        _                     => QUEUE_DRAIN_DEF_BPS,    // fail-safe: mínimo
    };
    let drain = mul_bps(state.deferred_burn_queue, drain_bps)
        .max(1)
        .min(state.deferred_burn_queue);

    // Si el drenaje llevaría el supply por debajo de ENZ, recortar al delta exacto
    let max_burnable = state.current_supply.saturating_sub(SUPPLY_ENZ);
    let drain = drain.min(max_burnable);
    require!(drain > 0, LukashError::BurnComplete);

    // (07-b aplica su cap dentro de burn_tokens_cpi)
    state.deferred_burn_queue -= drain;
    // burn real vía CPI (07-a)

    state.last_queue_exec_ts = now;
    emit!(DeferredBurnExecuted {
        drained: drain,
        remaining: state.deferred_burn_queue,
        ts: now,
        mode: state.throttle_mode,
    });
    Ok(())
}
```

### 3.3 Evento — campo nuevo

```rust
#[event]
pub struct DeferredBurnExecuted {
    pub drained: u64,
    pub remaining: u64,
    pub ts: i64,
    pub mode: u8,   // 0=ACELERADO, 1=NORMAL, 2=CONSERVADOR, 3=DEFENSIVO
}
```

### 3.4 Error nuevo

```rust
#[error_code]
pub enum LukashError {
    // ...
    #[msg("Burn complete: supply has reached ENZ minimum (3.3B)")]
    BurnComplete,
}
```

### 3.5 Tabla de referencia en `docs/protocolo/LUKASH_Protocolo_v4.3.md` §7

Actualizar §7 tabla del Throttle para reflejar:

- Columna "Drenaje cola/sem" con valores: ACEL 25% · NORMAL 10% · CONS 5% · DEF 2%.
- Retirar la mención "125% quema base" y sustituir por "100% quema + drenaje acelerado
  25%/sem de la cola diferida".
- Añadir nota: "Todos los modos drenan la cola — la quema nunca se detiene hasta ENZ
  (3.3B supply). En ENZ, la maquinaria de quema se apaga definitivamente."

---

## 4. Pruebas

### 4.1 Happy path (4 modos)

- Cola = 10_000 USD, modo ACELERADO, ≥1 semana → drena 2_500 USD (25%).
- Cola = 10_000 USD, modo NORMAL, ≥1 semana → drena 1_000 USD (10%).
- Cola = 10_000 USD, modo CONSERVADOR, ≥1 semana → drena 500 USD (5%).
- Cola = 10_000 USD, modo DEFENSIVO, ≥1 semana → drena 200 USD (2%).

### 4.2 Edge cases

- Cola = 1 USD (mínimo), cualquier modo → drena 1 (por `max(1, ...)`), cola queda en 0.
- Última ejecución hace 6 días → `QueueCooldown`.
- Modo ACELERADO pero cap del 07-b lleno por otras quemas: drenaje se recorta al cap
  remanente; el resto **permanece en cola** (no se pierde).
- Alternancia de modo entre dos ejecuciones (semana A DEFENSIVO 2%, semana B ACEL 25%):
  los drenajes son independientes, cada uno usa el modo vigente al ejecutar.
- **Supply = SUPPLY_ENZ exacto** → `BurnComplete`, no drena nada.
- **Supply = SUPPLY_ENZ + 50 tokens**, cola = 1_000 → drena solo 50 (recortado al
  delta exacto para no bajar de ENZ).

### 4.3 Invariantes nuevos

- **I9:** la cola nunca tiene deriva positiva permanente — todos los modos drenan,
  así que mientras haya transacciones, la cola eventualmente converge a 0.
- **I9b:** `current_supply >= SUPPLY_ENZ` siempre. Ninguna ruta de quema (directa o
  por drenaje) puede bajar el supply por debajo de 3.3B.
- **I9c:** una vez `supply == SUPPLY_ENZ`, no se ejecuta ningún drenaje posterior
  (la cola se congela con su saldo residual).

---

## 5. Fuera de alcance

- **Quema real vía CPI:** 07-a.
- **Cambiar el % del tramo LP quemado en la tx**: no se toca (100% en NORMAL/ACEL,
  60%/25% en CONS/DEF sigue igual). Solo cambia el drenaje de la cola.
- **Añadir un 5º modo Throttle "Súper-ACEL"**: no se contempla; overengineering.
- **Qué hacer con la cola congelada post-ENZ**: decisión para Milestone 3/DAO.
  Los USD equivalentes representan valor "comprometido a quema" que nunca se
  ejecutará — podría redirigirse al Vault por gobernanza, pero eso es Etapa 4.
- **Throttle invertido post-ENZ** (Etapa 3): usa los mismos modos pero con
  lógica opuesta. Es Milestone 3, no 2.

---

## 6. Done cuando

- [ ] 4 constantes de drenaje (`QUEUE_DRAIN_*_BPS`) + `SUPPLY_ENZ` en `constants.rs`.
- [ ] `execute_deferred_burn` lee `throttle_mode` y aplica el bps correcto (4 modos).
- [ ] Guardia ENZ al inicio de `execute_deferred_burn` + recorte de drenaje al delta
      exacto si el supply está cerca de ENZ.
- [ ] Guardia ENZ equivalente en `process_fee` (tramo LP → no quema si supply ≤ ENZ).
- [ ] Error `BurnComplete` definido.
- [ ] Evento incluye el campo `mode` (0-3 para los 4 modos).
- [ ] Protocolo v4.3 §7 actualizado (tabla del Throttle con drenaje en los 4 modos +
      nota ENZ hard-stop).
- [ ] Tests: happy path × 4 modos + 6 edge cases (incluyendo ENZ boundary) pasan.
- [ ] Simulación: todos los modos drenan → cola pico se reduce vs baseline;
      supply NUNCA cae por debajo de 3.3B en ningún escenario.
- [ ] Guardia ENZ en `process_fee` (tramo LP): si `current_supply <= SUPPLY_ENZ`, redirigir tramo LP al Vault en vez de quemar. Snippet explícito en el código.
- [ ] BITÁCORA actualizada.
