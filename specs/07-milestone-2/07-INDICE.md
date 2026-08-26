# Spec 07 — Milestone 2: Smart contracts LUKASH · Índice

> Ing. Sebastián Botero · 2026-08-21 · Base: ADR-015 + validación de motores sesión 5
> Estado del contrato de partida: Milestone 1 compilando en Playground (`contracts/playground/lib.rs`).
> Deploy Milestone 2 objetivo: **devnet primero** (mint Token-2022 dedicado), luego mainnet.

---

## 1. Qué cubre Milestone 2

Cerrar la brecha entre el contrato Milestone 1 (contabilidad pura en USD) y el
Protocolo v4.3 completo: introducir oráculos, swaps reales, quema real, KASH
Shield activo, Tridente Multisig (inactivo por defecto), Anti-Whale + Exit Fee
sobre Token-2022 transfer hook, y módulo contra-cíclico de LUKAI.

Al finalizar Milestone 2, el contrato es **auditable pre-TGE** y ejecuta la
lógica económica completa del protocolo, no una contabilidad simulada.

## 2. Las specs — orden de dependencia técnica

**Spec transversal (leer PRIMERO):**
- [**07-CUENTAS-Y-CUSTODIA**](./07-CUENTAS-Y-CUSTODIA.md) — mapa completo de las 17 cuentas del protocolo, filosofía "cero billeteras humanas para fondos" (todo PDAs excepto 2 multisigs), procedimientos operativos del multisig O&M y Tridente, checklist pre-TGE.

**Specs por hallazgo (orden de dependencia técnica):**

| # | Spec | Prioridad | Depende de | Habilita |
| --- | --- | --- | --- | --- |
| 1 | [**07-b — Cap de quema 1%/día**](./07-b-cap-quema-diaria.md) | alta | — (soft: 07-a para quema real) | 07-d |
| 2 | [**07-d — Throttle: drenaje siempre activo + hard-stop ENZ**](./07-d-acelerado-drenaje-rapido.md) | media (quick win) | 07-b (soft: 07-a para quema real) | — |
| 3 | [**07-a — Switch B0→B2 por Pyth + Token Accounting real**](./07-a-switch-b2-por-pyth-y-token-accounting.md) | **alta (estructural)** | 07-b | 07-e, 07-c, 07-f |
| 4 | [**07-e — Módulo Contra-Cíclico LUKAI**](./07-e-modulo-contraciclico.md) | media | 07-a | — |
| 5 | [**07-c — KASH Shield del Vault + Tridente inactivo**](./07-c-kash-shield-vault-tridente.md) | alta | 07-a | Etapa 2 (candado) |
| 6 | [**07-f — Anti-Whale + Exit Fee + Token-2022**](./07-f-antiwhale-exitfee-token2022.md) | alta | 07-a, 07-c | Mainnet TGE |

## 3. ADRs referenciados

- **ADR-005** — Aura con 5 niveles narrativos (usado en exención Glow de Anti-Whale/Exit Fee).
- **ADR-011** — KOLs pagados en tokens vesteados (por qué NO tienen exención Anti-Whale).
- **ADR-012** — Anti-Whale corregido (C10): solo ventas, umbral por % del pool.
- **ADR-015** — KASH Shield restaurado + Token-2022 + Tridente inactivo + exenciones canónicas + auditor desacoplado.

## 4. Cambios en el Protocolo v4.3

Las specs actualizan operativamente §4.1 (composición dinámica del Vault vía
contra-cíclico), §5.1 (LUKAI orquestador), §7 (Throttle con ACELERADO redefinido),
§9 (KASH Shield completo), §13 (parámetros formales del Blueprint). El documento
`docs/protocolo/LUKASH_Protocolo_v4.3.md` refleja el estado tras aplicar todos los
cambios de este Milestone.

## 5. Superficie total del contrato Milestone 2

### 5.1 Instrucciones nuevas o modificadas

| Instrucción | Origen | Firmante | One-way |
| --- | --- | --- | --- |
| `initialize` | existente | authority | Sí |
| `process_fee` | **modificada (07-a, 07-b, 07-e, 07-c)** | caller | No |
| `update_oracle_state` | **reemplazada por `refresh_vault_valuation`** (permissionless) | cualquiera | No |
| `refresh_vault_valuation` | nueva (07-a) | permissionless | No |
| `update_market_regime` | nueva (07-e) | authority (keeper) | No |
| `switch_motor_b` | **reescrita (07-a)** con doble candado | permissionless | **Sí** |
| `execute_deferred_burn` | **modificada (07-b, 07-d)** | permissionless | No |
| `queue_admin_change` / `execute_admin_change` | **modificada (07-c)** — bloqueo Etapa 2 sin Tridente | authority + Timelock | No |
| `set_pause` | existente | authority | No |
| `activate_tridente(pk1,pk2,pk3)` | nueva (07-c) | authority | **Sí** |
| `cancel_circuit_breaker` | nueva (07-c) | Tridente 3-de-3 | No |
| `receive_insurance_recovery(amt)` | nueva (07-c) | Tridente 3-de-3 | No (cooldown 12m) |
| `register_market_maker(mm)` | nueva (07-f) | authority + Tridente 3-de-3 | No |
| `revoke_market_maker(mm)` | nueva (07-f) | authority + Tridente 3-de-3 | No |
| `transfer_hook(amount)` | nueva (07-f) | Token-2022 sistema | No |
| `activate_layer_3_emergency` | stub (07-c, se desarrolla en Milestone 3) | Tridente 3-de-3 | — |

### 5.2 Estado nuevo total

En `ProtocolConfig`: `tridente_activated`, 3× `tridente_signer_*`, `tridente_activated_ts`.

En `ProtocolState`: 5× `*_amount` (balances por bucket), `luka_price_usd`,
`k_market_usd_snapshot`, `k_market_snapshot_ts`, `k_min_reached_since_ts`,
`market_regime`, `regime_updated_ts`,
`regime_reason`, `burned_today_tokens`, `burn_day_start_ts`, `cb_active_until_ts`,
`cb_last_snapshot_usd`, `cb_last_snapshot_ts`, `last_insurance_recovery_ts`,
`insurance_recoveries_total_usd`, `sell_pressure_1h_supply_bps`,
`sell_pressure_last_reset_ts`.

PDA nueva: `MMRegistry` (por MM).

### 5.3 Errores nuevos totales (por spec)

07-b: 2 · 07-d: 1 · 07-a: 5 · 07-e: 2 · 07-c: 11 · 07-f: 3 = **24 errores nuevos**.

### 5.4 Eventos nuevos totales

07-b: 1 · 07-d: 0 (extiende) · 07-a: 3 · 07-e: 1 · 07-c: 4 · 07-f: 5 = **14 eventos nuevos**.

### 5.5 Constantes nuevas totales

~30 constantes nuevas — todas explícitas en `constants.rs`. Ninguna magic number en el código.

## 6. Invariantes a validar en `simulations/suite.py inv`

I7-I22 (16 nuevos) — cada spec detalla los suyos. Todos deben pasar antes de
considerar el Milestone 2 cerrado. En particular:

- **I11:** switch B0→B2 es one-way (no path que revierta).
- **I14:** balances por bucket monotónicos crecientes (el Vault no se toca).
- **I16:** Tridente activado es monotónico (una vez sí, siempre sí).
- **I19:** `receive_insurance_recovery` incrementa solo `usdc_res_amount`.
- **I20:** fees de Anti-Whale + Exit Fee van 100% al Vault Core.

## 7. Simulación como validación empírica

Cada spec extiende `simulations/` con un test económico específico. Cambios clave
esperados vs Milestone 1:

- **07-a:** brecha `dia_b2` (valorizado) vs `dia_b2_contrato` (costo) < 5 días en mediana.
- **07-d:** cola pico media en escenario AGRESIVO −15% vs baseline.
- **07-e:** espiral CONSERVADOR baja de 44% → ~15-20% (converge al informe v4.2).
- **07-f:** escenario "Ballena 10% pool" mantiene 0% espiral vs baseline sin Anti-Whale.

Estos son los tests que dan confianza empírica antes de auditoría externa.

## 8. Orden sugerido de ejecución

1. **Sprint 1:** 07-b + 07-d juntos (aisla subsistema quema; quick win visible).
2. **Sprint 2:** 07-a completo (el grande — infra oráculo + Jupiter + valoración).
3. **Sprint 3:** 07-e + 07-c en paralelo (usan 07-a como base).
4. **Sprint 4:** 07-f (el más complejo — Token-2022 dedicado en devnet).
5. **Sprint 5:** endurecimiento + auditoría interna + deploy mainnet.

## 9. Fuera de Milestone 2

- Instrumentos cNFT como llaves de posición (Milestone 3).
- `activate_layer_3_emergency` con transferencia real (Milestone 3, requiere Sanctum).
- DAO gobernanza dinámica (Etapa 4, Milestone 4).
- Módulo contra-cíclico invertido post-ENZ (Milestone 3).
- Aura on-chain como PDA activa (Milestone 3 — la lectura desde `is_exempt` es
  interfaz; el registro se popla en Milestone 3).

## 10. Decisiones pendientes (no bloqueantes)

- **3 firmantes concretos del Tridente:** pendiente pre-TGE (mainnet). En devnet
  Milestone 2 sigo yo como authority única con Tridente inactivo. Candado estructural
  garantiza que no puedo ir a Etapa 2 mainnet sin activarlo.
- **Auditor(a) externa** vía subsidios (Colosseum / Areta / Superteam): cotización real cuando corresponda.
- **Cuentas concretas Jupiter/Meteora/Sanctum:** se resuelven en integración con
  cada CPI; interfaces ya definidas en las specs.

---

*Índice del Milestone 2. Base: ADR-015. Los 6 sub-documentos son fuente de verdad
para la implementación; este índice solo consolida.*
