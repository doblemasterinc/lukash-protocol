# TODO — LUKASH

## 🚨 BLOQUEANTES PRE-TGE MAINNET (recordar en cada session-close)

Estos NO son bloqueantes de desarrollo (devnet funciona sin ellos), pero **el TGE mainnet no puede ocurrir sin resolverlos**. El contrato tiene un candado estructural que rechaza el paso a Etapa 2 si el Tridente no está activado, así que Sebastián no puede olvidarlos por accidente — pero el trabajo de identificar y crear los wallets es humano y toma tiempo.

- [ ] **T1. Identificar 2 firmantes adicionales del Tridente Multisig (3-de-3)** — hoy solo está Sebastián. Los 3 deben ser distintos y ninguno puede ser la authority. Idealmente 3 hardware wallets (Ledger). Publicar pubkeys en ADR.
- [ ] **T2. Ejecutar `activate_tridente(pk1, pk2, pk3)`** post-deploy mainnet. **Instrucción one-way / irreversible.** Sin esto, el contrato bloquea el paso a Etapa 2.
- [ ] **T3. Crear multisig O&M** (2-de-3 con Squads Protocol) para el 15% operativo de fees. Hoy la wallet es single-sig de Sebastián — antes del TGE debe ser multisig con firmantes documentados en ADR (fundador + dev senior + tercero de confianza).
- [ ] **T4. LP Fundador con lock 365d on-chain** verificable antes del TGE.
- [ ] **T5. Auditoría externa** completada vía subsidio (Colosseum / Areta / Superteam) — no bloqueante si se hace en paralelo pero no puede saltarse.
- [ ] **T6. Deploy Milestone 2 en devnet 2+ semanas sin bugs** antes de mainnet.

Detalle completo y procedimientos en [`specs/07-milestone-2/07-CUENTAS-Y-CUSTODIA.md`](../specs/07-milestone-2/07-CUENTAS-Y-CUSTODIA.md) §7 (checklist pre-TGE).

## ⭐ PRÓXIMA SESIÓN (empezar por aquí)
0. [x] **★ AUDITORÍA PROFUNDA DE LOS MOTORES por SIMULACIÓN FIEL AL CONTRATO** (sesión 5): suite `simulations/` (engine=réplica de lib.rs + capa económica prima/Markov/vesting). SIM 0 invariantes OK · SIM 1 MC (espiral CONSERV 44%/BASE 0%/AGRESIVO 0% → tesis lanzamiento agresivo confirmada) · SIM 2 sensibilidad (volumen driver #1) · SIM 3 estrés (resiliente salvo Motor A −70% perm) · SIM 4 throttle irrelevante al Vault. Reporte `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` + Artifact `19a8160e`.
   - [ ] Pendiente: re-correr herramientas estáticas (Sec3/Trident/clippy) sobre `lib.rs` — BLOQUEADO local (sin toolchain Solana); hacer en Playground/CI.
   - [x] **Specs Milestone 2 en `specs/07-milestone-2/`** (2026-08-21, sesión 5): 07-INDICE + 6 specs (07-a a 07-f). Base: ADR-015.
   - [x] **07-b ratificada ✅** (cap quema 1%/día)
   - [x] **07-d ratificada ✅** (drenaje siempre activo 25/10/5/2 + hard-stop ENZ, ADR-016)
   - [x] **07-a ratificada ✅** (Switch B0→B2 por Pyth + Token Accounting real, persistencia 7d aprobada)
   - [x] **07-e ratificada ✅** (módulo contra-cíclico LUKAI, mayoría 3 señales + fail-safe NEUTRAL)
   - [x] **07-c ratificada ✅** (Jaguar Shield + Tridente inactivo + CB + Seguro Anti-Exploit)
   - [x] **07-f ratificada ✅** (Anti-Whale + Exit Fee + Token-2022 Transfer Hook + WhaleDebt suave)
   - Orden de implementación: Sprint 1 (07-b + 07-d) → Sprint 2 (07-a) → Sprint 3 (07-c + 07-e) → Sprint 4 (07-f) → Sprint 5 (endurecimiento + auditoría + mainnet).
1. **Análisis de recursos de LUKAI** (IA a escala): costo por usuario, arquitectura por etapas (v1 orquestador/plantillas → v2 conversacional Etapa 2A), break-even, financiado por O&M. Decidir si/cuándo activarlo (en landing quedó como mención ligera).
2. **Landing → producción**: desplegar en Vercel + waitlist real capturando correos (Formspree o Supabase). Landing lista en `landing/index.html` (trilingüe, Artifact `6c7985dd`).
3. **DEVNET pendiente**: [x] Token $LUKA con nombre+logo (Metaplex). [ ] Deploy del programa: requiere **2.12 SOL** (faucet 429; Helius RPC / otra IP). Ver `contracts/playground/RUNBOOK_DEVNET.md`.
4. **Etapa 0 (continuar)**: [x] one-pager ES, [x] founding myth + micro-myth (ES/EN/PT), [x] landing. [ ] traducir one-pager EN/PT, [ ] pitch deck (Colosseum/grants), [ ] data room/litepaper (ahí va tokenomics completo + Vault Sociedad). Arrancar Superteam Earn + grant Finternet.
5. **Contratos Milestone 2**: integración real SPL/Jupiter, quema real, staking, cNFT/Aura, oráculo Pyth+Switchboard, CPI safety.
6. [x] **Sub-estructura del 45% "Venta"** decidida → Seed dinámico→Public (ADR-013) + fila Equipo/Fundador 2% (ADR-014).


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
