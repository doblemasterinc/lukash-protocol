# Spec 07-e — Módulo Contra-Cíclico de LUKAI (EMA30 vs EMA90 BTC)

> Milestone 2 · Referencia: Protocolo v4.3 §5.2 · Prioridad: **media**
> Dependencias upstream: 07-a (oráculos Pyth + token accounting).
> Habilita: nada bloqueante — cierra la brecha simulada de la sesión 5.

---

## 1. Problema (qué hay hoy)

El contrato asigna el 35% Asset Layer siempre con la **misma receta fija** al
comprar hard assets (cBTC 35% / SOL 15% / LST 20% / USDC 25% / lending 5%).

El Protocolo v4.3 §5.2 ("Módulo de Gestión Contra-Cíclica") especifica que LUKAI
debe **dirigir el flujo de NUEVAS ENTRADAS** según el régimen de mercado detectado.
Regla textual:

> "LUKAI NO gestiona el Vault existente. LUKAI dirige el flujo de NUEVAS ENTRADAS
> según el régimen. El Vault existente solo cambia por rendimiento natural.
> Esto elimina el riesgo de rebalanceo forzado en momentos de crisis."

Distribución dinámica de nuevas entradas por régimen (detectado con EMA30 vs
EMA90 de BTC + volatilidad realizada 30d + volumen del Motor A 7d/30d):

| Régimen (EMA30 vs EMA90 BTC) | Volátiles | USDC | Lógica |
| --- | ---: | ---: | --- |
| **BEAR** confirmado (EMA30 < EMA90) | 70% | 30% | Acumular activos duros baratos |
| **NEUTRAL** | 75% | 25% | Sin sesgo direccional |
| **BULL** confirmado (EMA30 > EMA90) | 40% | 60% | Construir USDC para el próximo bear |

**Hallazgo #6.4 de la sesión 5:** la ausencia de este módulo **eleva el riesgo de
espiral de la campaña CONSERVADORA de ~14% (informe v4.2) a 44% (mi simulación)**.
Es decir, el módulo no es opcional — es el pilar que hace robusto al protocolo
ante campañas débiles.

## 2. Diseño

### 2.1 Principio (crítico — no violar)

**Solo NUEVAS ENTRADAS.** El Vault existente **NO se toca** para rebalancear. Este
es el punto que aisla a LUKASH de la clase de fallo típica de otros protocolos:
"rebalanceo forzado en crisis" (vender BTC caro en bull para comprar más caro; o
peor, vender BTC barato en bear porque el ratio se desvió). Aquí eso no existe.

### 2.2 Régimen del mercado — detección

Nuevo estado:

```rust
pub market_regime: u8,           // 0=BULL, 1=NEUTRAL, 2=BEAR
pub regime_updated_ts: i64,
pub regime_reason: [u8; 32],     // hash de {ema30, ema90, vol30, vol_a_7d, vol_a_30d}
```

Nueva instrucción **`update_market_regime`** (llamada por keeper, típicamente 1×/día):

```rust
pub fn update_market_regime(
    ctx: Context<UpdateMarketRegime>,
    ema30_btc_price: u64,
    ema90_btc_price: u64,
    realized_vol_30d_bps: u64,   // volatilidad realizada BTC, 30d, bps anualizados
    vol_a_7d_usd: u64,           // volumen Motor A últimos 7 días
    vol_a_30d_usd: u64,          // volumen Motor A últimos 30 días
) -> Result<()>
```

**Firmante:** authority (keeper LUKAI). Superficie de ataque acotada intencionalmente:
solo la authority puede empujar el régimen, y como los 3 splits posibles son todos
"razonables" (40-75% volátil), el peor caso (régimen manipulado o stale) tiene
**impacto acotado** — no hay asignación catastrófica.

### 2.3 Lógica de decisión del régimen

```rust
// Señal primaria: EMA30 vs EMA90 BTC
let ratio_bps = (ema30_btc_price as u128 * BPS_DENOMINATOR as u128)
    .checked_div(ema90_btc_price as u128).ok_or(LukashError::MathOverflow)?;

let signal_primary = match ratio_bps as u64 {
    r if r > 10_200 => 0,  // BULL (EMA30 > EMA90 con margen 2%)
    r if r <  9_800 => 2,  // BEAR (EMA30 < EMA90 con margen 2%)
    _                    => 1,  // NEUTRAL (banda ±2% alrededor)
};

// Confirmadores: aumentan la certeza; en caso de disenso → forzar NEUTRAL
let signal_vol_conf = realized_vol_30d_bps > VOL_HIGH_THRESHOLD_BPS;  // vol alta → confirma tendencia
let signal_vol_a_conf = if vol_a_30d_usd == 0 { 1 } else {
    let ratio = (vol_a_7d_usd as u128 * 10_000) / (vol_a_30d_usd as u128 / 30 * 7).max(1);
    if      ratio > 12_000 { 0 }  // volumen creciente → BULL
    else if ratio <  8_000 { 2 }  // volumen cayendo → BEAR
    else                        { 1 }
};

// Regla conservadora: al menos 2 de 3 señales de acuerdo, si no → NEUTRAL
let regime = majority_vote(signal_primary, signal_vol_conf, signal_vol_a_conf, /*default*/ 1);
```

**Fail-safe:** si el keeper no actualiza en 48h (`regime_updated_ts` viejo),
`process_fee` lee `regime = NEUTRAL` por defecto (no confía en un dato stale).

### 2.4 Aplicación en `process_fee` — reasignación del Asset Layer

**Antes** (composición fija):
```
Core recibe X USD → split cBTC 35 / SOL 15 / LST 20 / USDC-res 25 / lending 5
```

**Después** (módulo contra-cíclico):
```
Core recibe X USD:
  1. Leer state.market_regime (con fail-safe NEUTRAL si stale)
  2. Volátiles% y USDC% según régimen:
     BULL   → 40% volátiles / 60% USDC
     NEUTRAL→ 75% volátiles / 25% USDC
     BEAR   → 70% volátiles / 30% USDC
  3. Dentro de "volátiles", preservar proporciones RELATIVAS de la receta base:
     cBTC:SOL:LST = 35:15:20 → normalizado sobre 70% base = 50:21.4:28.6
     Aplicar esas proporciones a los "volátiles%" del régimen actual.
  4. Dentro de "USDC", split reserva/lending mantiene 25:5 → 83.3:16.7
     Aplicar sobre "USDC%" del régimen.
```

**Ejemplo numérico** — Motor A entra $100 al Core:

| Bucket | Fijo (hoy) | BULL | NEUTRAL | BEAR |
| --- | ---: | ---: | ---: | ---: |
| cBTC | $35 | $20 | $37.5 | $35 |
| SOL | $15 | $8.6 | $16.1 | $15 |
| LST | $20 | $11.4 | $21.4 | $20 |
| USDC-res | $25 | $50 | $20.8 | $25 |
| USDC-lend | $5 | $10 | $4.2 | $5 |

Los CPIs a Jupiter (07-a) usan estos nuevos porcentajes por tx.

### 2.5 Regla del Vault existente — protección

**Invariante añadido a SIM 0:** los balances `cbtc_amount`, `sol_amount`, `lst_amount`
solo pueden **crecer** por `process_fee` (compra) o por `yield_rebase` (07-a, LST
apreciándose). Ningún otro camino los reduce. Eso hace imposible el rebalanceo
forzado. Enforced por design (no hay instrucción para "vender cBTC del Vault").

---

## 3. Contrato — interfaces

### 3.1 Constantes nuevas

```rust
pub const REGIME_BULL:    u8 = 0;
pub const REGIME_NEUTRAL: u8 = 1;
pub const REGIME_BEAR:    u8 = 2;

// Umbrales para la señal primaria EMA30/EMA90 (con banda ±2%)
pub const EMA_BULL_THRESHOLD_BPS: u64 = 10_200;
pub const EMA_BEAR_THRESHOLD_BPS: u64 =  9_800;

// Volatilidad realizada 30d (BTC) — umbral "alta"
pub const VOL_HIGH_THRESHOLD_BPS: u64 = 6_000;  // 60% anualizado

// Fail-safe: si el keeper no actualiza en 48h, se usa NEUTRAL por defecto
pub const REGIME_MAX_STALENESS: i64 = 48 * 3600;

// Splits por régimen (bps del Asset Layer del Core)
// {volátiles_total_bps, usdc_total_bps}
pub const SPLIT_BULL:    (u64, u64) = (4_000, 6_000);
pub const SPLIT_NEUTRAL: (u64, u64) = (7_500, 2_500);
pub const SPLIT_BEAR:    (u64, u64) = (7_000, 3_000);

// Proporciones RELATIVAS dentro de "volátiles" (suman 10000): cBTC:SOL:LST
// Derivadas de 35:15:20 → normalizadas
pub const VOLATIL_CBTC_REL_BPS: u64 = 5_000;   // 50.0%
pub const VOLATIL_SOL_REL_BPS:  u64 = 2_143;   // 21.4%
pub const VOLATIL_LST_REL_BPS:  u64 = 2_857;   // 28.6% (10000 - 5000 - 2143 = 2857)

// Proporciones RELATIVAS dentro de "USDC" (suman 10000): reserva:lending
// Derivadas de 25:5 → normalizadas
pub const USDC_RES_REL_BPS:  u64 = 8_333;   // 83.33%
pub const USDC_LEND_REL_BPS: u64 = 1_667;   // 16.67%
```

### 3.2 Nueva instrucción `update_market_regime`

Firmante = authority (keeper). Escribe el régimen tras aplicar la lógica de decisión
descrita en §2.3. Emite evento `MarketRegimeUpdated`.

### 3.3 Función helper `resolve_regime_effective(state, now_ts) -> u8`

```rust
fn resolve_regime_effective(state: &ProtocolState, now_ts: i64) -> u8 {
    let age = now_ts.saturating_sub(state.regime_updated_ts);
    if age > REGIME_MAX_STALENESS {
        REGIME_NEUTRAL  // fail-safe
    } else {
        state.market_regime
    }
}
```

Se llama al inicio de `process_fee` para elegir el split del Asset Layer.

### 3.4 Errores nuevos

```rust
#[msg("Régimen inválido (debe ser 0=BULL, 1=NEUTRAL o 2=BEAR)")]
InvalidRegime,
#[msg("EMA90 no puede ser cero")]
InvalidEmaInput,
```

### 3.5 Eventos nuevos

```rust
#[event]
pub struct MarketRegimeUpdated {
    pub regime: u8,
    pub ema30_btc: u64,
    pub ema90_btc: u64,
    pub vol_30d_bps: u64,
    pub vol_a_7d: u64,
    pub vol_a_30d: u64,
    pub signals: [u8; 3],  // primaria, vol, vol_a
    pub ts: i64,
}
```

---

## 4. Pruebas

### 4.1 Happy path

- BTC subiendo 3 meses (EMA30 > 1.02×EMA90, vol alta creciendo, vol A 7d/30d >1.2)
  → régimen BULL → Motor A siguiente entrada asigna 60% a USDC.
- BTC lateral (ratio EMA en banda ±2%) → NEUTRAL → 75/25 estándar.
- BTC bajando 3 meses → BEAR → 70/30, prioridad acumular hard assets baratos.

### 4.2 Edge cases

- **EMA30 = EMA90 exacto:** `ratio_bps = 10_000` → NEUTRAL (dentro de banda). ✓
- **Señales en disenso** (primaria BULL, vol NEUTRAL, vol_a BEAR): majority_vote →
  no hay mayoría → default NEUTRAL. Conservador por diseño.
- **Keeper no actualiza en 49h:** `resolve_regime_effective` retorna NEUTRAL, no
  el último valor. Fail-closed contra régimen manipulado que se dejó "olvidado".
- **`ema90_btc = 0`:** `InvalidEmaInput`.
- **Régimen fuera de rango en update:** `InvalidRegime`.

### 4.3 Invariantes nuevos

- **I13:** `market_regime ∈ {0, 1, 2}` en todo momento.
- **I14:** los balances por bucket (`cbtc_amount`, etc.) son monotónicamente crecientes
  (nunca decrecen) — enforce del principio "el Vault no se toca".
- **I15:** para todo `process_fee`, la suma de USD asignada a los 5 buckets = USD
  del Core (invariante de conservación 07-a preservado bajo splits dinámicos).

### 4.4 Prueba de simulación (crítica)

En `simulations/trajectory.py`, sustituir la composición fija por la contra-cíclica
usando el régimen ya modelado en `economic.py` (`markov` + EMA30/EMA90). Re-correr
SIM 1:

**Hipótesis a validar:** el módulo contra-cíclico **reduce la espiral CONSERVADORA
del 44% actual a ~14% (referencia informe v4.2)**. Si el número converge, valida
que el módulo es el responsable de la brecha observada en sesión 5.

---

## 5. Fuera de alcance

- **Rebalanceo del Vault existente:** explícitamente prohibido por diseño §5.2.
- **Módulo contra-cíclico "invertido" post-ENZ (Etapa 3):** v4.3 §7B. Va en Milestone 3.
- **Aprendizaje dinámico de umbrales:** los thresholds son constantes v1; la
  gobernanza (DAO Etapa 4) los ajusta post-datos reales.
- **UCR / Recarga Forzada:** existe en el diseño pero es funcionalidad Milestone 3
  (activa solo bajo estrés extremo).

---

## 6. Done cuando

- [ ] Constantes y estado nuevos en `constants.rs` y `state.rs`.
- [ ] `update_market_regime` implementada con lógica de mayoría de señales.
- [ ] `resolve_regime_effective` con fail-safe NEUTRAL bajo staleness.
- [ ] `process_fee` usa el régimen para dividir Asset Layer volátiles/USDC.
- [ ] Los CPIs a Jupiter reciben los porcentajes correctos por régimen (integra 07-a).
- [ ] Los 2 errores + 1 evento nuevos definidos.
- [ ] Tests: 3 regímenes felices + 5 edge cases pasan.
- [ ] Invariantes I13-I15 pasan en `simulations/suite.py inv`.
- [ ] **Simulación SIM 1 con contra-cíclico: espiral CONSERVADOR baja a rango
      15-20%** (converge a la referencia del informe v4.2).
- [ ] BITÁCORA actualizada.
