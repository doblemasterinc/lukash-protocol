# Audit Readiness — LUKASH Protocol (Milestone 1)

> 2026-08-20 · Paquete de preparación para la auditoría externa (Halborn/OtterSec, pre-mainnet).
> Un auditor cobra menos y encuentra menos si le entregas esto: modelo de amenazas, invariantes, control de
> acceso, alcance, y hallazgos ya resueltos. **Adelanta la auditoría externa sin gastar.**

## 1. Alcance auditado
**Dentro (Milestone 1):** lógica económica aislada — `process_fee` (distribución 35/35/15/15), Vault
(composición + split Core/Sociedad), Throttle (modo + cola diferida + `execute_deferred_burn`), conmutación
B0/B2, Timelock 48h, pausa (circuit breaker), control de acceso. Montos como **notional USD** (sin tokens reales).

**Fuera (Milestone 2 — a auditar después):** transferencias reales SPL, CPIs a Jupiter, quema real a null,
staking distributor real, integración de oráculo (Pyth + Switchboard redundante), cNFT/Aura on-chain, y el
**KASH Shield** completo (Exit Fee, Anti-Whale corregido, Circuit Breaker LP, seguro anti-exploit).

## 2. Modelo de amenazas
| Amenaza | Estado en M1 | Mitigación / nota |
| --- | --- | --- |
| Overflow aritmético | ✅ Mitigado | `checked_*` + u128 intermedio en toda operación (`mul_bps`, sumas, restas) |
| Autoridad comprometida | ✅ Diseño | `authority` = Tridente Multisig (3-de-3) + **Timelock 48h** en cambios críticos |
| `process_fee` con montos falsos | ⚠️ M2 | En M1 solo mueve contadores; **en M2 el fee lo respalda el movimiento REAL de tokens** (no se puede fingir). Documentado como requisito M2. |
| Manipulación de oráculo (precio/EMA30) | ⚠️ M2 | Throttle y switch B2 dependen del oráculo; **M2 exige Pyth + Switchboard con umbral 2%** (redundancia). No en M1. |
| Reentrancy | ✅ N/A | Modelo de cuentas Solana; en M2 cuidar orden de CPIs |
| Colisión de PDAs | ✅ Mitigado | Seeds fijas (`config`/`state`), bump almacenado y verificado |
| Cambio malicioso de parámetros | ✅ Mitigado | `has_one = authority` + Timelock; sin bypass |
| DoS en `execute_deferred_burn` | ✅ Mitigado | Cooldown semanal + condición de precio; permissionless pero acotado |

## 3. Invariantes (deben cumplirse SIEMPRE — candidatos a tests/fuzzing)
1. **Distribución:** `to_vault + to_lp_burn + to_om + to_staking == fee` (verificado en runtime).
2. **Composición del Vault:** `cbtc + sol + lst + usdc_res + usdc_lend == core` (sin fuga; `usdc_lend` = resto).
3. **Split Asset Layer:** `core + sociedad == to_vault`.
4. **Sumas de config:** `dist_bps == 10000` y `vault_target_bps == 10000` (validado en `initialize`).
5. **Vault monótono:** `vault_core_usd` nunca decrece vía `process_fee` (solo crece).
6. **Motor B unidireccional:** `motor_b_state` solo transiciona B0→B2, nunca vuelve.
7. **Conservación de cola:** lo que sale de `deferred_burn_queue` entra a `burned_total` (sin crear/destruir).
8. **Timelock:** ningún parámetro crítico cambia sin `now >= pending_execute_after` (48h).

## 4. Matriz de control de acceso
| Instrucción | Quién | Gate |
| --- | --- | --- |
| `initialize` | authority (firmante) | una vez |
| `process_fee` | cualquiera* | bloqueado si `paused` (*en M2: respaldado por transferencia real) |
| `update_oracle_state` | authority | `has_one` (en M2: rol keeper) |
| `switch_motor_b` | cualquiera | condición on-chain (K≥K_min) — trustless |
| `execute_deferred_burn` | cualquiera | condición (modo NORMAL/ACEL + cooldown) — trustless |
| `queue/execute_admin_change`, `set_pause` | authority | `has_one` + Timelock 48h |

## 5. Hallazgos de auto-revisión (ya resueltos)
- ✅ Comentarios `//!` inválidos a mitad de archivo → corregidos.
- ✅ Cola de quema diferida sin drenar → añadido `execute_deferred_burn`.
- 🔵 Simplificaciones intencionales documentadas: notional USD, ACCEL 125%→100%, `process_fee` sin gate real (M2).
- 🟡 Pendientes de M2 (no bugs de M1): convergencia de fees Etapa 3, Motor C como inyección-LP (no quema), Anti-Whale/Exit Fee (KASH Shield).

## 6. Herramientas GRATIS para correr antes del auditor pagado
Córrelas en la terminal de Solana Playground o local (reducen el costo del audit externo):
- `cargo clippy` — linter de Rust (bugs comunes, malas prácticas).
- `cargo-audit` — vulnerabilidades en dependencias.
- **Sec3 X-Ray** — analizador estático gratuito para programas Solana/Anchor.
- **Trident** (Ackee) — framework de **fuzzing** para Anchor (prueba los invariantes de §3 con inputs aleatorios).
- `anchor test` — correr los tests (`tests/lukash_protocol.ts`) y ampliar cobertura.

## 7. Qué mirará el auditor externo (para que llegues preparado)
Control de acceso · validación de cuentas (`has_one`, seeds, owner checks) · signer checks · seguridad de PDAs ·
aritmética/overflow · seguridad de CPIs (M2) · invariantes económicos del Vault/fees · y los riesgos DeFi
específicos (manipulación de precio, MEV, drenaje del Vault). Ya tienes cubierto lo de M1; el grueso del audit
pagado será sobre M2 (los movimientos reales de tokens).

## 8. Endurecimiento de seguridad APLICADO (contrato v2)
Tras la investigación de mejores prácticas (coral-xyz/sealevel-attacks, Neodyme "Common Pitfalls", Helius), se aplicó al código:
- **Validación de inputs:** `currency ≤ 1`, `amount > 0`, y oráculo con `precio/EMA30 > 0` (rechaza valores en cero).
- **Protección de gobernanza:** el cambio de autoridad **no puede ser la dirección cero** (evita brickear el control); `k_min` no puede quedar en 0.
- **Freeze en emergencia:** `switch_motor_b` y `execute_deferred_burn` ahora también respetan el `paused` (el circuit breaker congela transiciones de estado).
- **Observabilidad:** eventos nuevos (`PauseSet`, `AdminChangeQueued`, `AdminChangeExecuted`) para monitoreo off-chain.
- *(Ya presentes: checked math + `overflow-checks=true`, `has_one`, seeds+bump canónico guardado, `init` anti-reinit, tipos tipados `Account<T>`, sin `unwrap`/`panic` en runtime, Timelock 48h.)*
- **Pendiente M2 (donde está el riesgo real):** separación de roles (admin/pauser/keeper), oráculo real **Pyth+Switchboard** con staleness/confidence, **CPI safety** (`Program<Token>` tipado, validar mint/owner, checks-effects-interactions), caps por-tx/diarios.

## 9. Rutas de auditoría BARATAS/GRATIS (capital cero) — el orden que baja el costo a la mitad
1. **GRATIS primero (NO negociable):** correr **Sec3 X-Ray** + **Trident** (fuzzing) + `cargo clippy` + `cargo-audit`; arreglar todo high/critical; documentar invariantes (§3). Publicar en **Superteam** para revisión comunitaria.
2. **⭐ Subsidio (la palanca clave):** **Areta — $1M Solana Audit Subsidy Program.** Subsidia auditorías de firmas top (Zellic, Oak Security, QuillAudits, Sherlock, Statemind, Quantstamp…). Cohortes **mensuales** (cierre día 7). **Los equipos del Colosseum Accelerator entran fast-track.** (blog.colosseum.com/audit-subsidy-program · earn.superteam.fun). El % exacto no está publicado — confirmar con Areta.
3. **Grants:** **Solana Foundation** (solana.org/grants-funding) pueden cubrir costos de seguridad. **Certora Solana Security Hub** (verificación formal subsidiada).
4. **Concursos** (si hay algo de capital): **Code4rena / Sherlock** (soportan Solana/Rust; pools desde ~$100K para TVL alto — para ti, formato boost/solo o un bounty pequeño). **Cantina** (grandes).
5. **Boutique barato:** **SigIntZero, Zealynx, Bunzz** (~$5K–$20K un programa Solana). Descuento 20–30% si llegas con **tests >90% + spec + hallazgos automáticos ya resueltos**.
6. **Post-mainnet continuo:** **Immunefi bug bounty** — pay-as-you-go, sin pool adelantado, mínimos bajos; pagas **solo si aparece un bug real**. Red de seguridad permanente barata.

**Regla de oro:** nunca pagues un audit de código inmaduro. Las capas 1–2 son gratis y son las que más bajan el costo final. *(Precios/programas cambian — verificar. No es asesoría de inversión.)*

## Conclusión
El código de M1 está **audit-ready a nivel interno y endurecido (v2)**. La auditoría externa se hace **sobre M2**
(tokens reales), justo antes de mainnet — y con las rutas de §9 (subsidio Areta + herramientas gratis) puede
salir **casi gratis**. Este paquete + tests + fuzzing harán ese audit **más barato, rápido y con menos hallazgos**.
