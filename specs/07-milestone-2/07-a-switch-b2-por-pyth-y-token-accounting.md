# Spec 07-a — Switch B0→B2 por valor de mercado + Token Accounting real

> Milestone 2 · Referencia: Protocolo v4.3 §4.1, §5.1, §13 · Prioridad: **alta**
> **Es la spec estructural del Milestone 2** — introduce oráculos, quema real y balances por bucket.
> Dependencias upstream: 07-b (cap diario).
> Habilita: 07-e (contra-cíclico usa la misma infra de oráculo + régimen).

---

## 1. Problema (qué hay hoy)

**Milestone 1 = contabilidad pura en USD.** `contracts/playground/lib.rs`:

- `process_fee` acumula `vault_core_usd`, `cbtc_usd`, `sol_usd`, etc. como enteros
  USD 6-dec representando "fees depositados". Nunca se compran los activos reales.
- `switch_motor_b` compara `vault_core_usd` (costo acumulado) contra `k_min_usd`.
  **No usa el valor de mercado.**
- No hay CPI a Jupiter (swap), Pyth (precio), ni al mint (burn).

**Consecuencia:** el switch podría dispararse tarde en un mercado bull (donde el
Vault ya vale $30M pero solo se han depositado $20M en fees) o temprano en un bear
(donde se han depositado $30M pero el mercado los valora en $18M). El protocolo
v4.3 §5.1 es explícito: *"Monitorea K(t) vía Pyth Network en tiempo real"* — y §13:
*"K_min = $25M ... Verificación Pyth"*.

Además, el diseño requiere que el switch sea **one-way (irreversible)**, aprobado
por Sebastián — una vez B2, no vuelve a B0 aunque el mercado caiga. Esto exige
que el disparador sea **robusto** para no capturar picos transitorios.

Hallazgo #6.2 en `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md`.

## 2. Diseño

### 2.1 Cambio de modelo: de "USD depositados" a "cantidades por bucket"

El Vault deja de contabilizarse como "USD que entraron" y pasa a contabilizarse
como **cantidades reales de cada activo** (lamports de SOL, satoshis de cBTC,
micro-USDC, etc.). Sobre eso se calcula el valor de mercado en cada tick.

Nuevo estado en `ProtocolState`:

```rust
// balances por bucket, en UNIDADES NATIVAS del activo (no USD)
pub cbtc_amount:       u64,  // satoshis (8 dec) de cBTC
pub sol_amount:        u64,  // lamports (9 dec) de SOL nativo
pub lst_amount:        u64,  // lamports de JitoSOL+mSOL agregados (aprox)
pub usdc_res_amount:   u64,  // micro-USDC (6 dec) reserva inmediata
pub usdc_lend_amount:  u64,  // micro-USDC depositado en Kamino/Marginfi

// último snapshot valorizado (para lecturas baratas del dashboard)
pub k_market_usd_snapshot:  u64,  // USD 6-dec
pub k_market_snapshot_ts:   i64,
```

Los campos antiguos (`cbtc_usd`, `sol_usd`, etc.) **se mantienen** por compatibilidad
con la instrumentación existente (registran "costo depositado" = útil para split
Core/Sociedad y para métricas históricas), pero **no se usan para decidir el switch**.

### 2.2 Compra real de activos vía CPI a Jupiter

Cuando `process_fee` calcula el tramo Core que va a cada activo:

```
Motor A recibe X SOL fee → 35% Asset Layer → 70% Core = amount_core_sol
  → Jupiter CPI: swap amount_core_sol × 35% a cBTC → suma a cbtc_amount
  → Jupiter CPI: swap amount_core_sol × 20% a LST  → suma a lst_amount
  → amount_core_sol × 15% queda como SOL nativo    → suma a sol_amount
  → Jupiter CPI: swap amount_core_sol × 25% a USDC → suma a usdc_res_amount
  → Jupiter CPI: swap amount_core_sol ×  5% a USDC → deposit Kamino/Marginfi → usdc_lend_amount
```

Cada CPI aplica slippage máximo del **0.5%** vía `otherAmountThreshold`. Si el swap
excede slippage, la tx falla — mejor que absorber pérdidas silenciosas.

**Motor B** (fees en $LUKA): antes del split, el tramo Core primero se swapea
$LUKA→SOL vía Jupiter y luego sigue el flujo estándar. **Motor C** (USDC): parte
de USDC ya, salta el primer swap.

### 2.3 Valoración por oráculo (Pyth + Switchboard)

Nueva instrucción **permissionless** `refresh_vault_valuation`:

```rust
pub fn refresh_vault_valuation(ctx: Context<RefreshVaultValuation>) -> Result<()> {
    // Lee precio de cada activo desde Pyth Y Switchboard
    // Verifica desviación entre ambas fuentes < 2% (redundancia — v4.2 §9)
    // Verifica frescura del feed < 60 segundos
    // Calcula K_market = Σ (amount_bucket × precio_bucket_usd)
    // Actualiza k_market_usd_snapshot + k_market_snapshot_ts
    // Emite evento VaultValuationRefreshed
}
```

**Permissionless** = cualquiera puede llamarla (típicamente el keeper de LUKAI cada
15 minutos, o el propio usuario justo antes de intentar `switch_motor_b`).
Trustless porque solo *lee* de oráculos y calcula — no cambia balances.

### 2.4 Switch B0→B2 con doble candado

El switch **one-way** debe sobrevivir a picos transitorios de BTC. Dos protecciones:

**Candado A — Valoración fresca:**
```rust
require!(
    now_ts - state.k_market_snapshot_ts <= VALUATION_MAX_STALENESS,
    LukashError::ValuationStale
);
```
`VALUATION_MAX_STALENESS = 900` (15 minutos). Fuerza a llamar
`refresh_vault_valuation` justo antes.

**Candado B — Persistencia (TWAP simple):**
Nuevo estado:
```rust
pub k_min_reached_since_ts: i64,  // 0 si no está sobre K_min actualmente
```

En cada `refresh_vault_valuation`:
```rust
if k_market >= config.k_min_usd {
    if state.k_min_reached_since_ts == 0 {
        state.k_min_reached_since_ts = now_ts;  // arma el timer
    }
} else {
    state.k_min_reached_since_ts = 0;  // rearma si el mercado cae
}
```

En `switch_motor_b`:
```rust
require!(
    state.k_min_reached_since_ts > 0
    && (now_ts - state.k_min_reached_since_ts) >= K_MIN_PERSISTENCE_SECONDS,
    LukashError::KminNotPersistent
);
```

`K_MIN_PERSISTENCE_SECONDS = 7 * 24 * 60 * 60` (7 días). Elimina el riesgo del pump
transitorio de BTC: el Vault debe **sostener** el nivel K_min por 7 días
continuos antes de que el switch sea legal. En bull sostenido normal, esto solo
retrasa el switch ~1 semana — costo aceptable a cambio de la garantía de irreversibilidad.

### 2.5 Quema real vía CPI (habilita 07-b y 07-d)

Nueva función auxiliar `burn_luka_cpi(ctx, tokens)` que hace CPI al mint (SPL clásico
en Milestone 2 devnet, Token-2022 en mainnet — la interfaz es la misma). Consume
lo que 07-b calculó como `burn_ok`.

---

## 3. Contrato — interfaces

### 3.1 Constantes nuevas

```rust
pub const VALUATION_MAX_STALENESS: i64 = 900;              // 15 min
pub const K_MIN_PERSISTENCE_SECONDS: i64 = 7 * 24 * 3600;  // 7 días
pub const ORACLE_DEVIATION_BPS_MAX: u64 = 200;             // 2% Pyth vs Switchboard
pub const ORACLE_FEED_MAX_STALENESS: i64 = 60;             // 60s (frescura Pyth)
pub const JUPITER_MAX_SLIPPAGE_BPS: u64 = 50;              // 0.5%
```

### 3.2 Nueva instrucción `refresh_vault_valuation`

```rust
#[derive(Accounts)]
pub struct RefreshVaultValuation<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    // Feeds Pyth
    pub pyth_btc: AccountInfo<'info>,
    pub pyth_sol: AccountInfo<'info>,
    // Feeds Switchboard (redundancia)
    pub sb_btc: AccountInfo<'info>,
    pub sb_sol: AccountInfo<'info>,
    // El caller no necesita ser autoridad (permissionless)
    pub caller: Signer<'info>,
}
```

### 3.3 `switch_motor_b` — reescritura

```rust
pub fn switch_motor_b(ctx: Context<SwitchMotorB>) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, LukashError::ProtocolPaused);

    let state = &mut ctx.accounts.state;
    require!(state.motor_b_state == MOTOR_B_B0, LukashError::AlreadyB2);

    let now_ts = Clock::get()?.unix_timestamp;

    // Candado A: valoración fresca
    require!(
        now_ts.checked_sub(state.k_market_snapshot_ts).unwrap_or(i64::MAX)
            <= VALUATION_MAX_STALENESS,
        LukashError::ValuationStale
    );

    // K market ≥ K_min
    require!(
        state.k_market_usd_snapshot >= config.k_min_usd,
        LukashError::KminNotReached
    );

    // Candado B: persistencia ≥ 7 días
    require!(
        state.k_min_reached_since_ts > 0,
        LukashError::KminNotPersistent
    );
    require!(
        now_ts.checked_sub(state.k_min_reached_since_ts).unwrap_or(0)
            >= K_MIN_PERSISTENCE_SECONDS,
        LukashError::KminNotPersistent
    );

    // Conmutación ONE-WAY
    state.motor_b_state = MOTOR_B_B2;
    emit!(MotorBSwitched {
        k_market_usd: state.k_market_usd_snapshot,
        k_costo_usd: state.vault_core_usd,   // referencia histórica
        ts: now_ts,
        persistencia_dias: (now_ts - state.k_min_reached_since_ts) / 86_400,
    });
    Ok(())
}
```

### 3.4 Errores nuevos

```rust
#[msg("Valoración del Vault demasiado vieja (>15 min). Llamar refresh_vault_valuation primero")]
ValuationStale,
#[msg("K_min alcanzado pero no persistente 7 días — protección anti-pump transitorio")]
KminNotPersistent,
#[msg("Oráculos Pyth y Switchboard divergen >2%")]
OracleDeviationTooHigh,
#[msg("Feed del oráculo demasiado viejo (>60s)")]
OracleFeedStale,
#[msg("CPI a Jupiter falló por slippage")]
SwapSlippageExceeded,
```

### 3.5 Eventos nuevos

```rust
#[event]
pub struct VaultValuationRefreshed {
    pub k_market_usd: u64,
    pub cbtc_usd_share: u64,  // valor cBTC en USD
    pub sol_usd_share: u64,
    pub lst_usd_share: u64,
    pub usdc_total: u64,
    pub k_min_reached_since_ts: i64,  // 0 si no armado
    pub ts: i64,
}

#[event]
pub struct AssetPurchased {
    pub motor: u8,
    pub asset: u8,  // 0=cBTC, 1=SOL, 2=LST, 3=USDC-res, 4=USDC-lend
    pub input_usd: u64,
    pub output_amount: u64,  // en unidades nativas del activo
    pub slippage_bps_realized: u16,
}
```

---

## 4. Pruebas

### 4.1 Happy path

- Fee del Motor A entra → los 4 CPIs a Jupiter se ejecutan → balances por bucket
  suben en las cantidades correctas → `vault_core_usd` (costo) sube en el USD
  equivalente al momento del swap.
- `refresh_vault_valuation` → calcula `k_market = 30M` → `k_min_reached_since_ts`
  se arma.
- 7 días después, sigue armado y ≥ K_min → `switch_motor_b` conmuta a B2. ✅
- Intento de conmutar día 6.9 → `KminNotPersistent`.

### 4.2 Edge cases

- **Pump transitorio:** K sube a $27M en día 1, cae a $23M en día 3, sube a $28M
  en día 4 → `k_min_reached_since_ts` se rearmó en día 4; el switch requiere
  otros 7 días. Correcto.
- **Divergencia Pyth vs Switchboard >2%:** `refresh_vault_valuation` falla con
  `OracleDeviationTooHigh` → snapshot no se actualiza → `switch_motor_b` falla con
  `ValuationStale` si la última buena es >15 min. Fail-closed. Correcto.
- **Feed Pyth stale >60s:** igual, no snapshot, no switch.
- **Slippage Jupiter >0.5%:** el fee entero falla, el usuario debe reintentar. No
  hay quema/vault a medias.
- **Ya en B2, intento reconmutar:** `AlreadyB2` (irreversible confirmado).

### 4.3 Invariantes nuevos

- **I10:** `k_market_usd_snapshot = Σ (bucket_amount × precio_bucket)` con margen
  <0.5% (tolerancia numérica).
- **I11:** una vez `motor_b_state = MOTOR_B_B2`, nunca vuelve a `MOTOR_B_B0` en
  ninguna trayectoria de simulación (no hay path que lo revierta).
- **I12:** si `switch_motor_b` ejecutó exitosamente, entonces `now - k_min_reached_since_ts ≥ 7d`
  se cumple en ese bloque.

### 4.4 Prueba de simulación

Extender `simulations/engine.py` para modelar apreciación real de assets vs
"costo depositado", y verificar que la brecha `dia_b2 (valorizado)` vs
`dia_b2_contrato (costo)` — que en la sesión 5 vimos era pequeña en mediana
(d195 vs d200 AGRESIVO) — se **cierra** con esta spec (ambos convergen porque el
contrato ya usa la valoración correcta).

---

## 5. Fuera de alcance

- **Composición dinámica del Vault por régimen (contra-cíclico):** 07-e (usa la
  infra de oráculo que esta spec deja lista).
- **Anti-Whale y Exit Fee:** 07-f.
- **Instrumento cNFT como llave de posición:** fuera de Milestone 2 (Milestone 3).
- **Sanctum Fair-Price para LST:** se difiere; se usa Pyth JitoSOL/mSOL directo
  (aceptable para devnet; refinamiento post-auditoría).
- **Elección definitiva de las cuentas Jupiter/Meteora**: se deja abierta la
  interfaz; los `AccountInfo` concretos se resuelven en integración.

---

## 6. Done cuando

- [ ] `ProtocolState` extendido con los 5 balances por bucket + snapshot + `k_min_reached_since_ts`.
- [ ] Constantes nuevas en `constants.rs`.
- [ ] `refresh_vault_valuation` implementada, permissionless, con validación Pyth+Switchboard.
- [ ] `process_fee` hace CPI a Jupiter para los 5 swaps del Asset Layer con slippage 0.5%.
- [ ] `switch_motor_b` reescrita con doble candado (frescura + persistencia 7d).
- [ ] `execute_deferred_burn` y el `process_fee` (tramo LP) usan `burn_luka_cpi` real.
- [ ] Los 5 errores nuevos + 2 eventos nuevos definidos.
- [ ] Tests unitarios: happy path + 5 edge cases pasan.
- [ ] Invariantes I10, I11, I12 pasan en `simulations/suite.py inv`.
- [ ] Simulación actualizada: brecha `dia_b2` vs `dia_b2_contrato` < 5 días en mediana.
- [ ] Documentación de las cuentas Jupiter requeridas por motor en `contracts/INTEGRATION.md`.
- [ ] Compila (`anchor build`) y despliega en devnet.
