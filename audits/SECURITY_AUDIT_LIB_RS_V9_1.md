# Auditoría de Seguridad — LUKASH Protocol (lib.rs v9.1)

**Contrato**: `contracts/playground/lib.rs`
**Líneas**: ~2090 | **Framework**: Anchor 0.30.1 | **Red**: Solana devnet
**Fecha**: 2026-08-26 | **Auditor**: Claude Opus 4.6 (revisión manual)

---

## Resumen Ejecutivo

| Severidad | Cantidad |
|---|---|
| **CRITICAL** | 3 |
| **HIGH** | 2 |
| **MEDIUM** | 3 |
| **LOW** | 5 |
| **GOOD (patrones positivos)** | 12 |

**Riesgo global**: **ALTO para mainnet** en estado actual. **ACEPTABLE para devnet** con advertencias.

La lógica económica (35/35/15/15, Throttle, ENZ, Anti-Whale, Exit Fee) está correctamente implementada. La aritmética checked es consistente. PDAs y CPI bien asegurados. Sin embargo, los 3 CRITICAL hacen el contrato **no desplegable en mainnet** sin corrección.

---

## Top 3 Prioridades

1. **[CRITICAL] Restringir `process_fee` a callers autorizados** (#1)
2. **[CRITICAL] Restringir `refresh_vault_valuation` + validar feeds Pyth** (#2, #3)
3. **[HIGH] Eliminar fallbacks de precio hardcoded antes de mainnet** (#4)

---

## Hallazgos

### #1 — process_fee sin control de acceso al caller

- **Severidad**: CRITICAL
- **Categoría**: Access Control
- **Ubicación**: `process_fee` (L526-712) + contexto `ProcessFee` (L1728-1741)
- **Descripción**: El contexto `ProcessFee` solo requiere un `caller: Signer<'info>` genérico. No hay constraint `has_one = authority`. Cualquier wallet puede invocar `process_fee` con un `amount` arbitrario (hasta u64::MAX).
- **Impacto**: Un atacante puede inflar `vault_core_usd` hasta superar `k_min_usd` ($25M), habilitar switch B0→B2 prematuramente, inflar contadores de quema/staking/O&M, y drenar el `burn_vault` de tokens reales vía CPI `token::burn`.
- **Recomendación**: Agregar `has_one = authority @ LukashError::Unauthorized` al constraint de `config` en `ProcessFee`, o whitelist de callers autorizados.

### #2 — refresh_vault_valuation permissionless con parámetros controlados por el caller

- **Severidad**: CRITICAL
- **Categoría**: Access Control + Account Validation
- **Ubicación**: `refresh_vault_valuation` (L733-829) + contexto `RefreshVaultValuation` (L1794-1805)
- **Descripción**: Función permissionless que recibe `lst_price_usd`, `luka_price_usd`, `cbtc_amount`, `sol_amount`, etc. como parámetros directos del caller.
- **Impacto**: Manipulación total de `k_market_usd_snapshot`, `luka_price`, Circuit Breaker, y umbral B0→B2.
- **Recomendación**: Agregar `has_one = authority`. En Capa 2, leer balances de vault ATAs on-chain.

### #3 — Feeds de Pyth sin validación de owner (cuenta falsificable)

- **Severidad**: CRITICAL
- **Categoría**: Account Validation + Oracle Safety
- **Ubicación**: `pyth_btc_feed`/`pyth_sol_feed` en contextos `RefreshVaultValuation` y `ExecuteVaultSwaps`; `parse_pyth_price` (L1501-1529)
- **Descripción**: Feeds Pyth son `AccountInfo<'info>` con `/// CHECK:` sin validación del owner. `parse_pyth_price` valida magic/tamaño/status/staleness, pero un atacante puede crear una cuenta propia que pase todas estas validaciones con precio manipulado.
- **Impacto**: Valoración del Vault y swaps con precios falsos.
- **Recomendación**: Validar `owner == PYTH_PROGRAM_ID` (`FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH` en mainnet):
  ```rust
  #[account(owner = PYTH_PROGRAM_ID)]
  pub pyth_btc_feed: AccountInfo<'info>,
  ```

### #4 — Precios hardcoded como fallback en Pyth

- **Severidad**: HIGH
- **Categoría**: Oracle Safety
- **Ubicación**: `refresh_vault_valuation` (L755, L764) y `execute_vault_swaps` (L1373, L1382)
- **Descripción**: Si Pyth falla, fallback a BTC=$65,000 y SOL=$150 hardcoded ("devnet fallback").
- **Impacto**: En mainnet, precios desactualizados en outage de Pyth. Atacante puede forzar el fallback pasando un feed válido con status != Trading.
- **Recomendación**: Eliminar fallbacks para mainnet. Revertir si Pyth falla. Feature flag `#[cfg(feature = "devnet")]` para devnet.

### #5 — execute_vault_swaps permissionless con feeds no validados

- **Severidad**: HIGH
- **Categoría**: Access Control + Oracle Safety
- **Ubicación**: `execute_vault_swaps` (L1355-1422) + contexto `ExecuteVaultSwaps` (L1808-1819)
- **Descripción**: Permissionless + feeds sin validación de owner (= #3).
- **Impacto**: Con feed falso de precio bajo, inflar balances del Vault masivamente.
- **Recomendación**: Validar owner de feeds + considerar `has_one = authority`.

### #6 — Intervalo de confianza de Pyth no verificado

- **Severidad**: MEDIUM
- **Categoría**: Oracle Safety
- **Ubicación**: `parse_pyth_price` (L1501-1529)
- **Descripción**: Se define `PYTH_AGG_CONF_OFFSET` pero nunca se usa. Pyth recomienda verificar `confidence/price < umbral` (1-2%).
- **Impacto**: En alta volatilidad, precios con incertidumbre amplia se usan sin filtro.
- **Recomendación**: Leer campo `conf` y verificar `conf * BPS / price < ORACLE_DEVIATION_BPS_MAX` (200 = 2%).

### #7 — Truncamiento u128→u64 en update_market_regime

- **Severidad**: MEDIUM
- **Categoría**: Arithmetic Overflow
- **Ubicación**: `update_market_regime` (L1027-1030)
- **Descripción**: `let ratio = ratio_bps as u64;` — cast sin checked puede truncar silenciosamente si EMA90 ~ 0.
- **Impacto**: Régimen de mercado incorrecto → asignación de Vault errónea.
- **Recomendación**: `u64::try_from(ratio_bps).map_err(|_| LukashError::MathOverflow)?`

### #8 — transfer_hook Capa 1 confía en parámetros del caller para exenciones

- **Severidad**: MEDIUM
- **Categoría**: Access Control
- **Ubicación**: `transfer_hook` (L1077-1182)
- **Descripción**: Recibe `sender_is_mm`, `sender_aura_score`, `sender_has_staking`, etc. como parámetros del caller, no leídos on-chain. Mitigado por `has_one = authority`.
- **Impacto**: Deuda técnica para Capa 2. Si authority comprometida, exención total de Anti-Whale/Exit Fee.
- **Recomendación**: En Capa 2, leer de PDAs on-chain. Documentar que Capa 1 es trusted-authority-only.

### #9 — BURN_ACCEL_BPS (125%) capped a 100%

- **Severidad**: LOW
- **Categoría**: Economic Logic
- **Ubicación**: `process_fee` (L649) + constante `BURN_ACCEL_BPS` (L61)
- **Descripción**: `BURN_ACCEL_BPS = 12_500` (125%) pero `.min(BPS_DENOMINATOR)` lo capea a 100%. Modo ACCELERATED quema igual que NORMAL.
- **Recomendación**: Verificar contra protocolo v4.3. Si 100% es correcto, cambiar constante a 10_000.

### #10 — saturating_sub en current_supply puede saltar ENZ

- **Severidad**: LOW
- **Categoría**: Arithmetic Overflow
- **Ubicación**: `process_fee` (L636, L659) y `execute_deferred_burn` (L1293)
- **Descripción**: `saturating_sub` en vez de `checked_sub` — si `tokens > current_supply`, supply llega a 0 saltando ENZ.
- **Recomendación**:
  ```rust
  let max_burnable = state.current_supply.saturating_sub(SUPPLY_ENZ);
  let actual_tokens = tokens.min(max_burnable);
  state.current_supply = state.current_supply.checked_sub(actual_tokens)?;
  ```

### #11 — Staleness de oracle demasiado permisivo (86400s)

- **Severidad**: LOW
- **Categoría**: Oracle Safety
- **Ubicación**: Constante `ORACLE_FEED_MAX_STALENESS` (L90)
- **Descripción**: 24 horas de staleness. Comentario dice "devnet", mainnet debería ser 60-120s.
- **Recomendación**: Feature flag o variable en `ProtocolConfig` para devnet/mainnet.

### #12 — TridenteAction no requiere authority como caller

- **Severidad**: LOW
- **Categoría**: Access Control
- **Ubicación**: Contexto `TridenteAction` (L1784-1791)
- **Descripción**: Solo verifica 3 firmas del Tridente, no que el caller sea authority.
- **Impacto**: Bajo — las 3 firmas son el requisito real. Permite que un tercero pague el fee de tx.
- **Recomendación**: Documentar si es intencional o agregar `has_one = authority`.

### #13 — close_protocol usa UncheckedAccount

- **Severidad**: LOW
- **Categoría**: Account Closing
- **Ubicación**: `close_protocol` (L1335-1343) + contexto `CloseProtocol` (L1876-1885)
- **Descripción**: `state` es `UncheckedAccount` para soportar migraciones. Seeds validan PDA pero no discriminador ni owner.
- **Impacto**: Bajo (devnet only, solo authority).
- **Recomendación**: Eliminar para mainnet (programa immutable).

---

## Patrones Positivos (GOOD)

| Patrón | Ubicación | Detalle |
|---|---|---|
| Distribución 35/35/15/15 con invariante | `process_fee` L562-572 | `to_staking` como remainder + assert sum == fee |
| Aritmética checked consistente | Todo el archivo | `checked_mul/add/sub/div`, `mul_bps` usa u128 intermedio |
| PDAs correctamente validadas | Todos los contextos | Seeds y bumps almacenados y validados |
| CPI burn seguro | L694-706, L1313-1324 | Signer seeds correctos, burn_vault por seeds+mint+authority |
| Checks-effects-interactions | `process_fee`, `execute_deferred_burn` | Estado antes de CPI |
| ENZ hard-stop múltiple | L623, L1224, L1253 | En process_fee + execute_deferred_burn + drenaje |
| Timelock 48h | `queue_admin_change`/`execute_admin_change` | Cambios críticos con espera |
| Tridente Multisig 3-de-3 | `assert_tridente_signed` L1557-1563 | Verifica is_signer de 3 pubkeys, one-way activation |
| Cierre anti-revival | `close_protocol` L1336-1341 | Zeroes lamports, reasigna system_program, realloc(0) |
| Daily burn cap | `apply_burn_cap` L1437-1464 | 1%/día con rollover UTC |
| Fail-safe régimen | `resolve_regime_effective` L1567-1577 | 48h sin update → NEUTRAL automático |
| Compras sin fee | `transfer_hook` L1099 | Exime correctamente compras (pool→user) |
