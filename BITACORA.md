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

## 2026-08-20 (sesión 3) — Token $LUKA vivo en devnet + tokenomics de fundador
- **🐆 HITO: Token $LUKA CREADO en devnet.** Mint `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr` (SPL clásico, 6 dec, supply 10B), visible en el explorador. Mint authority = wallet Playground.
  - La versión de `spl-token` de Playground es vieja: NO soporta `--enable-metadata` ni `--program-2022`. Token creado clásico. **Metadata (nombre/logo) pendiente vía Metaplex** (hostear `luka.json`+isotipo público, firmar con wallet Playground / metaboss).
- **Deploy del programa BLOQUEADO por faucet:** requiere **2.12 SOL**, solo se consiguió 1 SOL; faucet devnet respondió **429 (límite/seco)** todo el día (web pide GitHub con antigüedad; CLI rate-limited; bug "body stream" del RPC default). Reintentar con Helius RPC / otra IP / esperar. Deploy queda "cargado" en `contracts/playground/RUNBOOK_DEVNET.md` (copiar-pegar).
- **Insight de Sebastián — "farmear aura":** convergencia cultural gratis con la marca (Aura ya nombrada). Adoptado como **copy core** → `brand/AURA_FARMEO_POSICIONAMIENTO.md`. Estética pixel tipo DefiLand = candidata para F2; mini-juego plataformas estilo Mario = **diferido** (mucho dev, distrae del átomo).
- **Etapa 0 arrancada:** `docs/etapa-0/ONE_PAGER_es.md` (pieza madre, hook "farmea tu Aura", marco de utilidad sin promesa de retornos). Faltan EN/PT + founding myth + landing + deck.
- **ADR-013:** el 45% "Venta (Seed → Public)" se dimensiona como **Seed dinámico**: Seed ≤5% supply (SAFE+warrant, vesting on-chain), ventana cierra en el TGE, **lo no vendido rueda a Public**; Public ≥40% fair-launch. Seed opcional (solo si grants+Colosseum no dan runway).
- **ADR-014:** nueva fila **Equipo/Fundador 2%** (200M, de Marketing 10%→8%), **cliff 12m + lineal 48m** (espejo del Vault Sociedad). Economía de fundador en **3 capas**: O&M (ingreso corto plazo) · Equipo 2% (upside del token + gobernanza) · Vault Sociedad ≥10% del 30% (patrimonio). Resuelve el hueco de la tabla (ahora suma 100%). Tabla v4.3 + CLAUDE.md actualizados.
- **Próxima sesión:** (1) metadata del token (hostear isotipo+luka.json, Metaplex/metaboss → nombre/logo en explorer), (2) deploy del programa (juntar 2.12 SOL), (3) continuar Etapa 0 (EN/PT, founding myth, landing con waitlist, deck).

## 2026-08-21 (sesión 4) — Token con logo + Landing trilingüe (Etapa 0)
- **🐆 Token $LUKA con NOMBRE + LOGO en devnet.** Metadata Metaplex aplicada vía script TS en el cliente de Playground (`createMetadataAccountV3`, sin exportar keypair). Assets hosteados en repo público nuevo **`github.com/doblemasterinc/lukash-brand`** (isotipo optimizado 256px + `luka.json`). Logo elegido = `lukash protocol isotipo.png` (line-art, mejor a tamaño pequeño); ISOTIPO V2 reservado para héroe/marketing. Nota: explorer.solana.com cachea lento en devnet; Solscan lo muestra al toque.
- **Founding myth + micro-myth trilingües** (`docs/etapa-0/FOUNDING_MYTH.md`, `MICRO_MYTH.md`) — reescritos según el MASTER SPEECH: el problema real no es "exclusión" sino que **el fruto del trabajo se evapora** ("tus Lukas se escapan como niebla"; te enseñan a gastar, no a conservar).
- **Landing con waitlist construida y publicada** (Artifact `6c7985dd`, guardada en `landing/index.html`). ~16 iteraciones de copy con Sebastián. **Trilingüe funcional (ES/EN/PT)** con toggle i18n (74+ claves). Diseño on-brand (obsidiana+oro+verde, Cinzel/IBM Plex/Space Mono, 5 imágenes de marca reales, partículas bioluminiscentes). Secciones: hero (**"Ruge al llamado de la selva"**) → Respaldo real (no es meme) → Problema → 3 pilares (control sin intermediarios / fuerza de la Manada / farmea tu Aura) → Farmea tu Aura + niveles → Cómo funciona (cada transacción → Reserva crece) → **La bondad del protocolo** (Motores descentralizados) → **Transparencia** (qué hay en la Reserva: BTC/SOL/staking líquido/USDC + distribución del token) → La app → Manifiesto → CTA. Waitlist visual (captura real al desplegar en Vercel).
- **Decisiones de marca/copy (→ learned-rules):** Reserva = **"Reserva Sagrada"** (común, NUNCA "tu Reserva"/"KASH"); dinero coloquial = **"Lukas"** (no "plata"); tagline = "Dueño de tu camino" (footer) con h1 CTA "Ruge al llamado de la selva"; el "casino" son los **memes**, no la cripto (no dispararse al pie); **LUKAI de-enfatizado** a mención ligera hasta analizar recursos de IA a escala; el 2% del equipo NO se explica en landing (evita confundir con Vault Sociedad; va en litepaper).
- **Próxima sesión (prioridad de Sebastián):** (1) **auditoría profunda de los MOTORES + seguridad pre-auditoría** — validar que el protocolo funcione como se planteó; ref. visual `PROYECTO CRYPTO/lukash_flow_v6.html`. (2) análisis de recursos de LUKAI (IA a escala). (3) desplegar landing en Vercel + waitlist real. (4) deploy del programa (2.12 SOL).
- **Protocolo v4.3 finalizado** (10 correcciones de auditoría aplicadas C1-C10). Auditoría interna completa; auditoría externa (Halborn/OtterSec) queda como candado pre-mainnet.

## 2026-08-20 (sesión 3) — Endurecimiento de seguridad + rutas de auditoría baratas
- **Contrato endurecido v2** (✅ Build successful confirmado): validaciones de inputs, freeze en pausa (switch_motor_b + execute_deferred_burn), protección de cambio de autoridad (no dirección cero), eventos de observabilidad (PauseSet/AdminChangeQueued/AdminChangeExecuted). Basado en sealevel-attacks/Neodyme/Helius. Ya cumplía checked math, has_one, seeds+bump, init anti-reinit, tipos tipados, Timelock.
- **Paquete audit-readiness** (`contracts/AUDIT_READINESS.md`): modelo de amenazas, invariantes, matriz de acceso, herramientas gratis, y **rutas de auditoría capital-cero**: gratis (Sec3/Trident) → **subsidio Areta $1M** (Colosseum fast-track) → grants → boutique $5-20K → Immunefi.
- **Regla de seguridad clave:** en mainnet NUNCA contratos del Vault sin auditar (custodian fondos = honeypot). Devnet sí, libre.
- **Próxima sesión:** desplegar en DEVNET (programa + token $LUKA para verlo) → herramientas gratis → Milestone 2 → Etapa 0 (pitch deck) + Superteam/Colosseum.
