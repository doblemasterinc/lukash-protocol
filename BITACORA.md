# Bitácora — LUKASH

## 2026-08-19 — Arranque del proyecto en el Venture Studio
- Revisión completa del Protocolo v4.2 (2.312 líneas) y BMC v4.2 FINAL (1.825 líneas). Diseño económico excepcionalmente maduro y validado con Monte Carlo (5.000 iteraciones, 0.0% ruina).
- Creada carpeta `LUKASH/` con scaffold completo del Studio.
- Migrados 126 archivos desde "PROYECTO CRYPTO" (docs núcleo, análisis, app, presentaciones, research + toda la marca: logos, isotipos, personajes, conceptos UI, estilo visual, audio). Carpeta original conservada intacta.
- Decisión de marca: **Aura = rebrand de Jaguar Score** (ADR-002, aprobado por Sebastián).
- Primer milestone técnico definido: smart contracts core en devnet (ADR-003).
- Iniciada auditoría de discrepancias del protocolo (primer hallazgo confirmado: composición del Vault suma 105%).
- Pendiente prioritario de Sebastián: diseño del concepto de juego y las Misiones de Caza para ganar Aura.
- **Barrido comprehensivo de TODO el corpus** (5 revisores paralelos: estrategia/GTM, app/fases, simulaciones, auditorías previas v3, narrativa). Resultados:
  - `audits/DIVERGENCIAS_v3.1_vs_v4.2.md`: el PRD/UI/estrategia son v3.1 (pre-v4.2). Decisiones de marca pendientes (ticker $LUKA, niveles de Aura, naming Tótems).
  - `audits/AUDITORIA_COMPLEMENTARIA.md`: **HALLAZGO CRÍTICO (H10)** — los números "Monte Carlo Validated" ($37.2M/113x/0.0%/1.4%) provienen del modelo v3.1, NO del código v4.2 (que da 10-200× más, irreales). Sharpe/drawdown no reproducibles. + riesgos abiertos de auditorías v3 (contratos sin auditar, oráculo único, correlación Vault ~90%, vesting seed).
  - `docs/discovery/SINTESIS_DISCOVERY.md`: material consolidado para juego/Aura/App/GTM (universo narrativo, mecánicas de Jungle Arena, Manadas, Universal Fee Extractor, growth).
  - `brand/DESIGN_TOKENS.md` completado con paleta y sistema de UI del PRD/UI Kit.
- **Corrección detectada por Sebastián**: la liberación del Vault Sociedad estaba atada a dos hitos contradictorios ($30M mes 13 en Etapa 2B vs $50M en Etapa 3). Regla única canónica definida (ADR-006).
- **Tres entregables del día** (decisiones: Aura 5 niveles narrativos, Monte Carlo pendiente):
  1. `specs/01-aura-jungle-arena.md` — diseño completo de Aura (5 niveles) + Jungle Arena (3 senderos, catálogo de Misiones de Caza, economía sin inflación).
  2. `docs/protocolo/LUKASH_Protocolo_v4.3.md` — protocolo canónico con las 9 correcciones (C1-C9) aplicadas. Base para el código.
  3. `specs/02-smart-contracts-milestone-1.md` — spec de contratos devnet (4 programas Anchor, PDAs, instrucciones, distribución atómica, plan de pruebas).
- **Estrategia (Concepto Objetivo de Socio):** foco radical — lanzar solo el átomo (token+Vault+dashboard), comunidad primero, fasear el resto. Rating honesto y verdades duras en `docs/discovery/CONCEPTO_OBJETIVO_SOCIO.md`.
- **Reposicionamiento de marca (ADR-010):** fuera ideología ("pueblo", anti-banco); conservar el jaguar reencuadrado (ascenso, no colectivo); mezcla de 3 ejes; trilingüe ES/EN/PT; audiencia <45. `brand/POSICIONAMIENTO_NARRATIVA.md`.
- **Lanzamiento faseado (ADR-009):** F0 comunidad → F1 solo cripto (sin distribución a holders) → F2 app → F3/F4 instrumentos. Postura regulatoria de-riesgada. `specs/03` + `specs/04` (plan 12 semanas).
- **Niveles de Aura finales (ADR-005):** Cachorro → Rastreador → Cazador → Alfa → Jaguar.
- **Contratos Milestone 1:** scaffold Anchor completo en `contracts/` (programa lukash_protocol con lógica de fees, Vault, Throttle, B0/B2, Timelock, pausa + tests + README).
- **✅ CONTRATOS COMPILAN** ("Build successful" en Solana Playground, confirmado por Sebastián). Lógica auditada contra Protocolo v4.3 (`contracts/AUDITORIA_LOGICA_MILESTONE1.md`) — núcleo correcto, sin bugs críticos. Se añadió `execute_deferred_burn` (drena la cola de quema ≤10%/semana) y recompiló OK. Milestone 1 CERRADO.
- **Página Génesis** ([artifact] con arte real: logo, Reserva Sagrada, Camino del Jaguar, Tótems, Jungle Arena) + prototipo app. Marca v2 sin ideología, trilingüe.
- **Manual de lanzamiento** (`specs/05`): Ruta A comunidad-only (fair-launch) + Ruta B (directorio real de MM e influencers con contactos). Decisión: community-only primaria, MM fuera de lo público (ADR-011).
- **Auditoría integral + veredicto** (`audits/AUDITORIA_INTEGRAL_Y_VEREDICTO.md`): concepto 8.5/10, viabilidad con foco 6.5-7/10; vale la pena por el camino disciplinado de bajo capital.

## 2026-08-20 — Cierre de sesión
- **Decisiones cerradas:** fair-launch community-only sin MM (ADR-011) · KOLs/MM en tokens vesteados, no Vault Sociedad · tokenomics sin cambios (45/30/10/10/5) · financiación no-dilutiva primero (Superteam/grants/Colosseum) antes que SAFE.
- **`specs/06` Plan Maestro capital-cero:** escalera bootstrapping (credibilidad→comunidad→financiación→build→fair-launch) con rutas reales de Solana (Superteam Earn/Instagrants, Solana Foundation + Finternet grants, Colosseum $250K pre-seed, Alliance DAO/Outlier, SAFE+token warrant, ángeles LatAm).
- **Contratos Milestone 1 CONFIRMADO compilando** por Sebastián; lógica auditada; execute_deferred_burn añadido.
- Repo Git propio creado y sincronizado en GitHub (privado).
- **Próxima sesión:** (1) auditoría/revisión contratos + Milestone 2, (2) Etapa 0 (pitch deck, founding myth, one-pager, landing, data room), (3) arrancar Superteam + grants.

## 2026-08-20 (sesión 2) — Corrección Anti-Whale + cierre del protocolo
- Aclaración del **Jaguar Exit Fee**: el "100% al Vault" es el DESTINO del fee (5/3/1%), NO una penalización del 100%. Se mantiene **5% para Génesis** (ADR-012).
- **Anti-Whale corregido (C10 en Protocolo v4.3):** aplica solo a **ventas** (no compras); umbral por **% del pool de liquidez** (no del supply — que era brutal al inicio: 1% supply ≈ $10K al TGE); fix del nombre de exención → nivel **Jaguar**. Se implementa en el Jaguar Shield (Milestone 2).
- **Protocolo v4.3 finalizado** (10 correcciones de auditoría aplicadas C1-C10). Auditoría interna completa; auditoría externa (Halborn/OtterSec) queda como candado pre-mainnet.
