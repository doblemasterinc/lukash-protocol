# Spec — Smart Contracts Milestone 1 (devnet)

> v1.0 · 2026-08-19 · Base: Protocolo v4.3 §6, §7, §13. Stack: Rust + Anchor + Solana.
> Alcance: el **corazón económico** del protocolo en **devnet** — captura y distribución atómica de fees,
> Vault, conmutación B0↔B2 y Throttle. NO incluye App, Aura, cNFT ni Motor C (milestones posteriores).
> Objetivo "done": en devnet, un swap de prueba paga el fee de etapa, se distribuye 35/35/15/15 atómicamente,
> el Vault crece con la composición correcta, y el estado del Motor B conmuta a B2 al cruzar K_min. Con tests.

---

## 1. Programas (Anchor)

Para el milestone 1 se implementan **4 programas** (o un solo programa con módulos; recomiendo separarlos por
dominio de seguridad):

| Programa | Responsabilidad |
| --- | --- |
| `lukash_config` | Estado global del protocolo: etapa actual, fees, K_min, thresholds del Throttle, targets del Vault, timelock. Solo cambia vía Timelock 48h + Tridente. |
| `lukash_fee_router` | **Universal Fee Extractor.** Recibe el monto, calcula el fee según etapa/motor/capa, ejecuta la distribución atómica 35/35/15/15 (Vault / Quema-LP / O&M / Staking). |
| `lukash_vault` | Custodia y contabilidad del Vault KASH Core (Core 70% / Sociedad 30% del Asset Layer). Composición objetivo, rebalanceo solo con nuevas entradas, R_op. |
| `lukash_orchestrator` | LUKAI v1.0 on-chain: lee Pyth, conmuta B0↔B2 (K vs K_min), calcula el modo del Throttle (P vs EMA30) y gestiona la cola de quema diferida. |

CPIs: `fee_router` → Jupiter (swaps), → `vault` (depósito), → programa de burn (dirección null), → staking.

---

## 2. Cuentas / PDAs

```
ProtocolConfig (PDA: ["config"])                      // singleton, gobernado
  authority: Pubkey (Tridente Multisig)
  stage: u8                                            // 1=Genesis, 2=App, 3=Soberania (2A/2B via motor_b_state)
  fee_bps: { motor_a, motor_a_wl, motor_b, motor_c,    // en basis points (400 = 4.00%)
             d_capa0, d_capa1, d_capa2_luka, d_capa2_alt, d_capa3a, d_capa3b }
  dist_bps: { vault: 3500, lp_burn: 3500, om: 1500, staking: 1500 }   // suma 10000
  fee_split_bps: { core: 7000, sociedad: 3000 }        // split del Asset Layer
  k_min_usd: u64 (25_000_000e6)                         // 6 decimales USD
  vault_target_bps: { cbtc: 3500, sol: 1500, lst: 2000, usdc_res: 2500, usdc_lend: 500 } // suma 10000
  throttle_thresholds: { accel: 12000, cons: 8000, def: 5000 }        // % de EMA30 en bps
  throttle_burn_bps:  { accel: 12500, normal: 10000, cons: 6000, def: 2500 }
  burn_daily_cap_bps: u16 (100)                         // 1%/día
  timelock_seconds: i64 (172800)                        // 48h
  paused: bool                                          // circuit breaker

VaultState (PDA: ["vault"])
  balances: { cbtc, sol, lst, usdc_res, usdc_lend: u64 }   // en unidades del token
  k_usd: u64                                            // valor total del Core en USD (6 dec), vía Pyth
  r_op_usd: u64                                         // Reserva Operativa (idle)
  sociedad_usd: u64                                     // acumulado del Vault Sociedad
  jaguar_lock_hit: bool                                 // true al cruzar $30M Core o 12 meses
  genesis_ts: i64

MotorBState (PDA: ["motor_b"])
  state: u8                                             // 0 = B0, 1 = B2
  switched_ts: i64

ThrottleState (PDA: ["throttle"])
  ema30: u64                                            // precio EMA30 (6 dec), actualizado por orchestrator
  mode: u8                                              // 0 ACEL, 1 NORMAL, 2 CONS, 3 DEF
  deferred_burn_queue: u64                              // $LUKA acumulado pendiente de quema
  last_queue_exec_ts: i64

StakingPool (PDA: ["staking"])
  total_staked: u64
  reward_accumulator: u128                              // por-share, escalado
  ...

FeeBuffer (PDA: ["buffer"])                             // acumula hasta ~$500/20 SOL antes de swapear (gas)
  pending_sol: u64
  pending_by_motor: ...
```

---

## 3. Instrucciones

| Instrucción | Firma / actor | Lógica |
| --- | --- | --- |
| `initialize_protocol` | authority | Crea ProtocolConfig con valores del §13 del v4.3. Stage=1. |
| `process_fee` | usuario (vía CPI del router) | **Core del milestone.** Entrada: monto, motor, capa, divisa. Calcula fee_bps → fee. Bufferiza; al alcanzar el umbral ejecuta `distribute` (fracciona en swaps Jito para minimizar slippage). |
| `distribute` (interna) | — | Split atómico: 35% Asset Layer → `vault.deposit` (compra composición, split Core/Sociedad 70/30) · 35% LP/Quema → **B0: burn / B2: recirc LP** (según MotorBState) · 15% → O&M wallet · 15% → compra $LUKA → StakingPool. **Todo en una tx; overflow checks; suma verificada = monto del fee.** |
| `update_oracle_state` | orchestrator (keeper) | Lee Pyth: precio $LUKA, valor de activos del Vault → recalcula `k_usd`, `ema30`. Actualiza `throttle.mode` según P vs EMA30. Valida redundancia Pyth+Switchboard (umbral 2%). |
| `switch_motor_b` | orchestrator | Si `k_usd >= k_min_usd` y `state==B0` → B2 (evento público). Idempotente. Verificación Pyth. |
| `check_jaguar_lock` | orchestrator | Si `k_usd >= 30M` **o** `now - genesis_ts >= 12 meses` → `jaguar_lock_hit = true` (habilita liberación Sociedad, contabilidad off-chain para vesting). |
| `execute_deferred_burn` | keeper | Ejecuta ≤10%/semana de `deferred_burn_queue` cuando el modo vuelve a NORMAL. Respeta `burn_daily_cap_bps`. |
| `rebalance_vault` | orchestrator | Dirige SOLO nuevas entradas hacia la composición objetivo según régimen (contra-cíclico). **No liquida posiciones existentes.** |
| `distribute_staking` | keeper/usuario | Reparte el 15% acumulado a stakers (accumulator pattern). |
| `admin_set_param` | Tridente + Timelock | Cambios a k_min/fees/thresholds/vault_target: encolar → esperar 48h → ejecutar. |
| `emergency_pause` | Tridente | Circuit Breaker: pausa swaps/salidas (config.paused=true). |

---

## 4. Reglas de implementación críticas

1. **Atomicidad:** `process_fee`→`distribute` en una sola transacción. Si cualquier CPI falla, revert total. Nunca distribución parcial.
2. **Overflow:** toda aritmética con `checked_add/checked_mul/checked_div` o `u128` intermedio. Los bps se aplican como `monto * bps / 10000`.
3. **Invariante de suma:** `vault + lp_burn + om + staking == fee` (probar con property test). `dist_bps` debe sumar 10000.
4. **Composición del Vault suma 100%:** `sum(vault_target_bps) == 10000` (validado en `initialize`). Sin oráculos como reserva.
5. **B0/B2:** el tramo LP consulta `MotorBState` en cada distribución. B0 → burn a dirección null; B2 → recirculación al pool Meteora. Sin R_op en ninguno.
6. **Timelock:** ningún parámetro crítico cambia sin encolar + 48h. Guardar `pending_change` con `execute_after_ts`.
7. **Access control:** `authority = Tridente Multisig`; keeper (orchestrator) solo puede actualizar estado derivado (ema30, k_usd, mode, switch), nunca mover fondos arbitrariamente.
8. **Oráculos:** validar staleness y confianza de Pyth; requerir concordancia con Switchboard (≤2%) o abortar.
9. **Buffer/gas:** acumular hasta ~20 SOL equiv. antes de swapear; fraccionar en 3 swaps vía Jupiter routing (bundle Jito) para minimizar slippage.

---

## 5. Plan de pruebas (TypeScript, Anchor + Bankrun/devnet)

- **Unit (distribución):** para montos 1, 100, 999999, y valores límite → verificar 35/35/15/15 exacto y el invariante de suma. Fuzz con montos aleatorios.
- **Fees por etapa/capa:** Motor A 4%/2.5%WL; Motor B 2.5%; Motor D capas 0/1/2/3A/3B. Verificar diferencial 0.5%.
- **B0→B2:** simular Vault < y ≥ K_min; comprobar switch, idempotencia y que el tramo LP cambia de burn a recirc.
- **Throttle:** mover el precio simulado a cada zona (ACEL/NORMAL/CONS/DEF) y verificar % de quema y cola diferida.
- **Cola diferida:** acumular en DEF, volver a NORMAL, verificar ejecución ≤10%/semana y cap 1%/día.
- **Jaguar Lock:** cruzar $30M Core o simular 12 meses → `jaguar_lock_hit`.
- **Timelock:** intentar cambiar K_min sin esperar 48h → debe fallar; tras 48h → éxito.
- **Overflow/seguridad:** montos extremos, autoridad incorrecta (debe revertir), pausa de emergencia.
- **Integración devnet:** desplegar, ejecutar un swap real de prueba, verificar el crecimiento del Vault y los eventos on-chain.

## 6. Estructura de repositorio propuesta (`contracts/`)
```
contracts/
  Anchor.toml
  programs/
    lukash_config/      src/lib.rs
    lukash_fee_router/  src/lib.rs
    lukash_vault/       src/lib.rs
    lukash_orchestrator/src/lib.rs
  tests/                *.ts (Mocha/Anchor)
  migrations/
```

## 7. Fuera de alcance (milestones siguientes)
Aura on-chain · cNFT (Bubblegum) + mercado secundario · Motor C · App/LUKAI v2.0 · Jungle Arena · Manadas ·
Circuit Breaker LP completo · integración RWA. **Antes de mainnet:** auditoría Halborn/OtterSec + 2+ semanas en
devnet/testnet + reconciliación Monte Carlo (H10) + fallback de oráculo implementado.
