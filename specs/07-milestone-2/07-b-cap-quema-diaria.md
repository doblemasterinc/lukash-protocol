# Spec 07-b — Cap de quema diaria del 1% del supply

> Milestone 2 · Referencia: Protocolo v4.3 §13 · Prioridad: **alta**
> Dependencias upstream: ninguna (aisla el subsistema de quema).
> Habilita: 07-d (ACELERADO = drenaje de cola más rápido) · 07-a (quema real vía CPI).

---

## 1. Problema (qué hay hoy)

El Blueprint v4.3 §13 exige explícitamente:

| Parámetro | Valor v4.3 | Notas |
| --- | --- | --- |
| **Cap burn diario** | **1% supply/día** | Exceso al día siguiente |

En `contracts/playground/lib.rs` **no está implementado**. La instrucción
`process_fee` acumula `burned_total` en USD sin ningún tope diario ni conversión a
tokens quemados (Milestone 1 es contabilidad pura; la quema real vía CPI del mint
llega con esta spec + 07-a).

**Riesgo real:** a precios post-TGE muy bajos (~$0.0001), cada dólar del tramo LP
del Motor A/B/D quema muchos tokens. Sin cap, un pico de volumen podría contraer
supply demasiado rápido, disparar la deflación fuera de la senda diseñada y
distorsionar el precio de descubrimiento del token.

Registrado como Hallazgo #6.3 en `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md`.

---

## 2. Diseño

### 2.1 Principio

El cap protege el ritmo de contracción del supply. La regla es **suave**: no
descarta valor cuando se supera el cap, lo **difiere al día siguiente** vía la cola
`deferred_burn_queue` que **ya existe en el contrato**. Reutiliza la infraestructura
del Throttle CONSERVADOR/DEFENSIVO — misma cola, mismo drenaje.

### 2.2 Flujo diario

```
Fee entra → process_fee → distribución 35/35/15/15 → tramo LP (35%)
                                                     ↓
                              quema base a ejecutar (USD del día para este motor)
                                                     ↓
    ¿supply día - tokens acumulados quemados hoy ≥ cap_diario (1% supply)?
                                     │                                       │
                                    NO                                      SÍ
                                     ↓                                       ↓
                     quema completa vía CPI                    quema hasta llenar el cap
                     (burn_now_tokens += X)                         (llenar hasta cap)
                                     │                                       ↓
                                     │                       exceso USD → deferred_burn_queue
                                     │                                       ↓
                                     │                          se drena mañana (10%/sem)
                                     │                                       │
                                     └────────────────┬──────────────────────┘
                                                      ↓
                                       actualizar burn_day_start_ts si es un nuevo día UTC
```

### 2.3 Estado nuevo en `ProtocolState`

```rust
pub burned_today_tokens: u64,   // tokens quemados en el día UTC actual
pub burn_day_start_ts: i64,     // inicio del día UTC actual (86400s buckets)
```

### 2.4 Roll-over de día (medianoche UTC)

El "día" del cap se define como bucket UTC de 86_400 segundos, no como "24h desde
la última quema" (esa alternativa permite deriva). En cada `process_fee` y en
`execute_deferred_burn`:

```rust
let day_now = now_ts / WEEK_SECONDS * WEEK_SECONDS;  // start of UTC day
// (usando DAY_SECONDS = 86_400, no WEEK — corrección editorial abajo)
if state.burn_day_start_ts < day_now {
    state.burned_today_tokens = 0;
    state.burn_day_start_ts = day_now;
}
```

*Nota editorial:* introducir constante `DAY_SECONDS = 86_400` en `constants`.

---

## 3. Contrato — interfaces

### 3.1 Constantes nuevas (`constants.rs`)

```rust
pub const DAY_SECONDS: i64 = 86_400;
pub const DAILY_BURN_CAP_BPS: u64 = 100; // 1.00% del supply/día
```

### 3.2 Instrucción `process_fee` — modificaciones (aplica cuando la spec 07-a añada la quema real; hasta entonces, el cap opera en la contabilidad USD)

```rust
// Al principio de la sección "quema del tramo LP", tras calcular burn_now:
let now_ts = Clock::get()?.unix_timestamp;
let day_now = (now_ts / DAY_SECONDS) * DAY_SECONDS;
if state.burn_day_start_ts < day_now {
    state.burned_today_tokens = 0;
    state.burn_day_start_ts = day_now;
}

// convertir burn_now (USD 6-dec) → tokens usando precio del oráculo
let tokens_to_burn = usd_to_tokens(burn_now_usd, oracle_price)?;

// cap diario
let supply_now = get_mint_supply(ctx)?; // vía CPI Token-2022 al mint
let cap_today = mul_bps(supply_now, DAILY_BURN_CAP_BPS);
let remaining_cap = cap_today
    .saturating_sub(state.burned_today_tokens);

let burn_ok = tokens_to_burn.min(remaining_cap);
let burn_deferred_tokens = tokens_to_burn.saturating_sub(burn_ok);

// burn_ok se quema ahora (CPI a token burn en spec 07-a)
state.burned_today_tokens = state.burned_today_tokens
    .checked_add(burn_ok).ok_or(LukashError::MathOverflow)?;

// exceso a la cola diferida (USD equivalente, para consistencia con la cola existente)
if burn_deferred_tokens > 0 {
    let deferred_usd = tokens_to_usd(burn_deferred_tokens, oracle_price)?;
    state.deferred_burn_queue = state.deferred_burn_queue
        .checked_add(deferred_usd).ok_or(LukashError::MathOverflow)?;
    emit!(DailyCapReached {
        cap_tokens: cap_today,
        burned_today: state.burned_today_tokens,
        deferred_tokens: burn_deferred_tokens,
    });
}
```

**Nota crítica:** este bloque se activa cuando 07-a introduce la quema real vía CPI.
Hasta entonces el motor de simulación (`simulations/engine.py`) ya aplica el cap
en la capa económica — la fidelidad se preserva.

### 3.3 Instrucción `execute_deferred_burn` — modificaciones

Añadir el mismo chequeo de cap al drenar la cola: si el drenaje semanal excede el
cap remanente del día, se drena solo hasta el cap y la diferencia **permanece** en
la cola para la próxima semana (no se pierde).

### 3.4 Errores nuevos

```rust
#[msg("Precio del oráculo requerido para convertir USD↔tokens")]
OraclePriceMissing,
#[msg("Supply del mint requerido para calcular el cap diario")]
MintSupplyMissing,
```

### 3.5 Eventos nuevos

```rust
#[event]
pub struct DailyCapReached {
    pub cap_tokens: u64,
    pub burned_today: u64,
    pub deferred_tokens: u64,
}
```

---

## 4. Pruebas

### 4.1 Happy path

- Un motor A quema 0.3% del supply en una tx → todo se quema, `deferred_burn_queue` no cambia.
- 5 txs de 0.15% cada una en el mismo día UTC → las primeras 6 llenan el cap del 1%,
  la 7ª entera va a `deferred_burn_queue`, la 8ª también.
- Al día siguiente: `burned_today_tokens` se resetea a 0, el próximo `execute_deferred_burn`
  drena hasta el cap del nuevo día (10% de la cola O el cap remanente, lo menor).

### 4.2 Edge cases

- **Reloj retrocede** (`now_ts < burn_day_start_ts`): el condicional `<` protege,
  no se resetea. Nunca puede haber "cap negativo".
- **Overflow** en `burned_today + burn_ok`: `checked_add` → `MathOverflow`.
- **Precio del oráculo = 0**: `usd_to_tokens` retorna `OraclePriceMissing`, la tx
  falla — mejor fallar que quemar mal (fail-closed).
- **Supply < cap_today × burned_today** (imposible por diseño): `saturating_sub` en
  `remaining_cap` protege.
- **Cambio de día UTC en medio de una tx**: no aplica — cada tx lee `Clock` una vez
  y toma el bucket correspondiente atómicamente.

### 4.3 Invariantes a chequear (con `simulations/suite.py`)

Añadir a `SIM 0 — INVARIANTES`:

- **I7:** `burned_today_tokens ≤ 1% × supply` en todo momento.
- **I8:** `sum(burn_now_step) + sum(deferred_added_step)` a través del día ≤ `tokens_a_quemar_teóricos`
  (no se crea valor de la nada; el exceso solo se difiere).

---

## 5. Fuera de alcance

- **La quema real vía CPI** (`spl_token_2022::burn` o equivalente) va en spec **07-a**.
  Esta spec asume que existe una función `burn_tokens_cpi(amount)` disponible.
- **El drenaje de la cola bajo ACELERADO** va en spec **07-d** (aquí se mantiene el 10%/semana actual).
- **Régimen del mercado** para dirigir compra de assets al Vault va en **07-e**.

---

## 6. Done cuando

- [ ] Constantes `DAY_SECONDS` y `DAILY_BURN_CAP_BPS = 100` en `constants.rs`.
- [ ] `ProtocolState` incluye `burned_today_tokens: u64` y `burn_day_start_ts: i64`.
- [ ] `process_fee` respeta el cap y difiere el exceso a `deferred_burn_queue`.
- [ ] `execute_deferred_burn` también respeta el cap.
- [ ] Roll-over de día UTC funciona (test que avanza `Clock` 86_401s y verifica reset).
- [ ] Compila (`anchor build`).
- [ ] Tests unitarios de los 4 edge cases pasan.
- [ ] Invariantes I7 e I8 pasan en `python simulations/suite.py inv`.
- [ ] Registrado en BITÁCORA y ADR (no es un nuevo ADR — es implementación de v4.3 §13).
