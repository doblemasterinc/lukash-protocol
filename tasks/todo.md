# TODO — LUKASH

## ⭐ PRÓXIMA SESIÓN (empezar por aquí)
0. **DESPLIEGUE EN DEVNET** (lo que el usuario quiere ver): (a) desplegar el programa en Playground (connect wallet → `solana airdrop 2` → Deploy → Program ID en explorer devnet), (b) crear el token **$LUKA** SPL en devnet (LUKASH / LUKA / 6 dec / 10B / logo = isotipo) para verlo en el explorador. Guiar clic por clic (no se puede manejar el navegador desde aquí).
1. **Herramientas gratis de seguridad**: correr Sec3 X-Ray + Trident (fuzzing) + clippy sobre el contrato; arreglar hallazgos.
2. **Contratos Milestone 2**: integración real SPL/Jupiter, quema real, staking, cNFT/Aura, oráculo Pyth+Switchboard, CPI safety.
3. **Etapa 0 de lanzamiento**: pitch deck (Colosseum + grants + ángeles), founding myth (ES/EN/PT), one-pager, landing con waitlist, data room. Arrancar **Superteam Earn + grant Finternet** + preparar **hackathon Colosseum** (fast-track al subsidio de audit Areta).
4. **Decidir** sub-estructura del 45% "Venta" (fair puro vs seed chico).


## Sesión actual (2026-08-19) — Setup + Auditoría
- [x] Revisar Protocolo v4.2 y BMC v4.2 completos
- [x] Crear carpeta LUKASH con scaffold del Studio
- [x] Migrar todos los documentos y assets de marca desde PROYECTO CRYPTO
- [x] Definir Aura = rebrand de Jaguar Score (ADR-002)
- [x] Auditoría de discrepancias del protocolo → `audits/AUDITORIA_PROTOCOLO_v4.2.md`
- [x] Barrido comprehensivo de todo el corpus (5 revisores) → discovery + auditoría complementaria
- [x] H1 (Vault 105%) decidido: quitar oráculos → 100%
- [ ] **DECISIÓN Sebastián — H10 (Monte Carlo)**: elegir modelo oficial de cifras (recomiendo v3.1 conservador) + etiquetar fuente. No usar cifras del código v4.2 en material de inversión
- [ ] **DECISIÓN Sebastián — marca**: D1 ticker ($LUKA), D2 niveles de Aura (naming), D3 Tótems cNFT
- [ ] Riesgos abiertos de auditorías v3 (contratos sin auditar, oráculo fallback, vesting seed, correlación Vault) — condiciones de TGE

## Diseño de juego / Aura → `specs/01-aura-jungle-arena.md`
- [x] Diseñar el loop de Jungle Arena (3 senderos: Aprendiz/Rastro Diario/Rugido de la Manada)
- [x] Definir las Misiones de Caza concretas (catálogo A1-A5, D1-D4, R1-R6 con valores de Aura)
- [x] Aura 5 niveles (Cachorro→Jaguar Sabio) — ADR-005
- [x] Economía sin emisión inflacionaria (Energía como sink, recompensas desde Marketing/Staking)
- [ ] Cerrar pendientes de diseño: costos de Energía, reglas de Duelos, pools de Temporada, fórmula de Aura con pesos

## Bloqueantes de smart contract (del protocolo)
- [ ] Mercado secundario cNFTs: fee de venta, herencia del modo de rendimiento, actualización de Aura al transferir
- [ ] Frecuencia de distribución Modo B (semanal/mensual, fijo/elegible)
- [ ] Resolver discrepancias numéricas del Vault (105% → 100%)

## Primer milestone — Smart contracts core (devnet) [ADR-003]
- [x] Spec técnica de contratos → `specs/02-smart-contracts-milestone-1.md`
- [x] Scaffold Anchor completo → `contracts/` (programa lukash_protocol: lib.rs, state, constants, errors, tests, README)
- [x] **COMPILA** ✅ "Build successful" en Solana Playground (single-file en `contracts/playground/lib.rs`)
- [x] Auditoría de lógica vs Protocolo v4.3 → `contracts/AUDITORIA_LOGICA_MILESTONE1.md` (núcleo correcto)
- [x] Añadido `execute_deferred_burn` (cola de quema al 10%/semana) → recompilado OK
- [ ] Milestone 2: integración SPL/Jupiter, quema real, staking distributor, cNFT/Aura, fallback oráculo, auditoría externa pre-mainnet

## Fase 0 — Comunidad (pre-token) [ADR-009]
- [x] Estrategia → `specs/03-estrategia-lanzamiento-comunidad.md`
- [x] Plan ejecutable 12 semanas → `specs/04-plan-f0-comunidad.md`
- [x] Marca v2 sin ideología + trilingüe → `brand/POSICIONAMIENTO_NARRATIVA.md` [ADR-010]
- [x] Niveles de Aura unificados: Cachorro/Rastreador/Cazador/Alfa/Jaguar [ADR-005]
- [ ] Acciones semana 1 (Sebastián): reservar handles, founding myth, landing waitlist, lista KOLs
- [ ] Implementar distribución atómica de fees con overflow checks
- [ ] Implementar conmutación B0↔B2 (K_min=$25M vía Pyth)
- [ ] Implementar Throttle dinámico (EMA30)
- [ ] Tests TypeScript + simulación en devnet
