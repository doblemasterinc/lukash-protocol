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

## 2026-08-21 (sesión 5) — Validación de motores por simulación FIEL AL CONTRATO
- **Giro de enfoque (Sebastián):** no contrastar contra `lukash_flow_v6.html` (visualización v4.2 con Vault al 105%), sino **construir simulaciones que respondan si el protocolo funciona motor por motor**. Alinear con las simulaciones previas (no reinventar): `docs/protocolo/lukash_mc_v4.py`, informe `docs/analisis/LUKASH_Informe_simulaciones cuantitativas.pdf` (4 SIMs: sensibilidad/estrés/usuarios/throttle).
- **Suite nueva `simulations/`** (motor fiel al contrato): `engine.py` = réplica *línea por línea* de la aritmética entera de `contracts/playground/lib.rs`. `economic.py` (precio=P_KASH×prima, Markov+halving, vesting — portado del modelo v4). `trajectory.py` (una trayectoria de 5 años conduciendo el contrato). `suite.py` (4 SIMs + invariantes). `market.py`/`run.py` = primer intento descartado (impacto AMM colapsaba al piso). README completo.
- **Aporte vs informe previo:** aquel validó el DISEÑO económico (float); esta suite valida (a) que el CÓDIGO implementa el diseño (invariantes on-chain exactos) y (b) que las conclusiones estructurales se sostienen con la lógica on-chain real.
- **Resultados (25 min, semillas fijas):**
  - **SIM 0 invariantes:** TODOS OK. Distribución 35/35/15/15 cierra al micro-dólar (err<1e-10), O&M≡Staking, supply∈[3.3B,10B], precio≥P_KASH.
  - **SIM 1 Monte Carlo (tesis del lanzamiento agresivo CONFIRMADA):** CONSERVADOR espiral 44% / BASE 0% / AGRESIVO 0%. B2 mediana d1034/d448/d195. Vault med $90M/$389M/$1882M (ref informe $117M/$483M/$2.42B — mismo orden).
  - **SIM 2 sensibilidad:** Volumen $1287M ≫ Fee A $278M > Yield $7M > Launch $2M > K_min $0M (neutro, correcto). Volumen = driver #1.
  - **SIM 3 estrés:** crash BTC −80% → −3.8%, exploit 15% → −0.3%, retiro LP → −0.8%. Único dañino: Motor A −70% *permanente* → −68%. Alineado con informe.
  - **SIM 4 throttle/usuarios:** throttle irrelevante para el Vault (6 configs = $368M idéntico; solo afecta timing de quema del Motor B, que no toca el Core). Usuarios +$14M de 1K→100K (Motor D amplificador, no motor).
- **5 hallazgos de contrato (Milestone 2)** al construir el motor fiel: (1) modo ACELERADO inerte (`lib.rs:343` capa 125%→100%); (2) switch B0→B2 usa costo, no valor de mercado (falta oráculo Pyth); (3) sin cap de quema 1%/día (spec §13); (4) sin módulo contra-cíclico LUKAI (composición fija → campaña débil más frágil, explica espiral 44% vs 14%); (5) Jaguar Shield pendiente (correcto, ADR-012). + confirmado: falta límite retiro 5%/día del Vault.
- **Entregables:** `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` (reporte) + Artifact dashboard on-brand (`19a8160e`, obsidiana/oro, Cinzel/IBM Plex/Space Mono).
- **Bloqueado local:** cargo/clippy/Sec3/Trident (requieren toolchain Solana) — el re-run de herramientas estáticas sobre `lib.rs` se hace en Playground/CI, no local.
- **Próxima sesión:** priorizar hallazgos de contrato en spec de Milestone 2; análisis LUKAI (IA a escala); landing a producción (Vercel+waitlist); deploy del programa (2.12 SOL).

## 2026-08-21 (sesión 5, continuación) — ADR-015 + specs Milestone 2 (07-a a 07-f)
- **Arqueología documental** (`audits/HISTORIAL_JAGUAR_SHIELD.md`): rastreo v1.0 → v4.2 confirmó que Seguro Anti-Exploit original tenía 4 condiciones (5% + 1×/12m + activo tras año 1 + DAO año 5) que la v4.2 perdió/mutó. "Tridente" es de v1.0, "3-de-3" entra en v2.3. Halborn/OtterSec eran arrastres de v2.1 sin ADR formal. El "5%/día" **no existe en ninguna versión** — fue error mío arrastrado del informe.
- **ADR-015 aprobado por Sebastián:** (1) restaurar Seguro Anti-Exploit v1.0/v4.1 con activación desde **Etapa 2B** (opción B, condición on-chain no calendario ambiguo); (2) Tridente 3-de-3 ratificado, firmantes pendientes; (3) sacar Halborn/OtterSec, auditor a cotizar vía subsidios (Solana/Colosseum/Areta/Superteam); (4) regla de higiene documental (contrastar v4.2 con v1.0/v4.1); (5) **Token-2022 con Transfer Hook aprobado** para mainnet; (6) exenciones canónicas ratificadas (swaps Motor D, staking, LP lock, LP Fundador 365d, MMs registrados en Multisig, nivel Jaguar Aura — KOLs NO tienen exención, se alinean por vesting); (7) auditor externa vía subsidios.
- **Tridente inactivo por defecto** (patrón aprobado): el contrato construye toda la lógica 3-de-3 pero nace con `tridente_activated = false`. Instrucción one-way `activate_tridente(pk1, pk2, pk3)`. **Candado estructural**: contrato rechaza paso a Etapa 2 si Tridente no activado — imposible ir a mainnet por olvido.
- **6 specs Milestone 2 en `specs/07-milestone-2/`** + índice:
  - `07-b` cap quema 1%/día con exceso a cola (reusa cola existente)
  - `07-d` ACELERADO redefinido = drenaje 25%/sem vs 10%/sem NORMAL
  - `07-a` **estructural**: switch B0→B2 por Pyth + token accounting real por bucket + doble candado (frescura + persistencia 7 días para one-way irreversible) + quema real vía CPI
  - `07-e` módulo contra-cíclico EMA30/EMA90 BTC (aplica solo a nuevas entradas, Vault existente no se toca)
  - `07-c` Jaguar Shield del Vault: Tridente inactivo + Circuit Breaker 24h + `receive_insurance_recovery` (entrada, nunca salida)
  - `07-f` Anti-Whale + Exit Fee sobre Token-2022 transfer hook (WhaleDebt PDA modo suave; MMRegistry PDA)
- **Superficie Milestone 2:** ~17 instrucciones (10 nuevas), ~30 constantes nuevas, 24 errores nuevos, 13 eventos nuevos. Invariantes I7-I22 (16 nuevos) validables en `simulations/suite.py`.
- Correcciones aplicadas: `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` §6.6 + R3 (eliminado el "5%/día" inventado), `tasks/todo.md`, `learned-rules.md` (2 reglas nuevas).
- **Próxima sesión:** empezar Sprint 1 (07-b + 07-d) o revisar y ajustar specs con Sebastián antes de código.

## 2026-08-22 (sesión 6) — Ratificación specs 07-b/07-d + ADR-016
- **Spec 07-b (cap quema 1%/día) ratificada ✅** (sesión anterior).
- **Spec 07-d ratificada ✅** con dos cambios sustanciales respecto al borrador original:
  - **Drenaje en todos los modos del Throttle (ADR-016):** escala geométrica ACEL 25%/sem · NORMAL 10% · CONS 5% · DEF 2%. Evolución consciente del diseño original v2.1→v4.3 ("bono de deflación futura" / "al regresar a NORMAL"). Razón del cambio: en bear prolongado (2+ años) la cola acumula demasiado, dificultando la recuperación. El "bono" sigue existiendo (inflow > outflow en bear), solo de menor magnitud. Verificado contra TODAS las versiones del protocolo — el cambio se documenta como evolución, no corrección.
  - **Hard-stop ENZ:** guardia `supply ≤ 3.3B` → toda la maquinaria de quema se apaga definitivamente, cola congelada, supply nunca baja de 3.3B. 100% alineado con todas las versiones (v1.0→v4.3).
- **Próxima:** revisión spec 07-a (Switch B0→B2 por Pyth + Token Accounting real).

## 2026-08-22 (sesión 7) — Ratificación de 4 specs restantes + auditoría integral + fixes
- **4 specs ratificadas por Sebastián:**
  - **07-a ✅** (Switch B0→B2 por Pyth + Token Accounting real, persistencia 7d aprobada)
  - **07-e ✅** (módulo contra-cíclico LUKAI, mayoría 3 señales + fail-safe NEUTRAL)
  - **07-c ✅** (Jaguar Shield + Tridente inactivo + CB + Seguro Anti-Exploit)
  - **07-f ✅** (Anti-Whale + Exit Fee + Token-2022 Transfer Hook)
- **Milestone 2 completo: las 6 specs ratificadas.** Orden de implementación confirmado: Sprint 1 (07-b+07-d) → Sprint 2 (07-a) → Sprint 3 (07-c+07-e) → Sprint 4 (07-f) → Sprint 5 (hardening+audit+mainnet).
- **Auditoría integral post-ratificación** (3 agentes: filosofía, consistencia inter-spec, ADRs). 13 hallazgos: 1 BLOCKER + 6 diseño + 6 menores. **Todos resueltos:**
  - **BLOCKER B1:** `state.luka_price` referenciado en 07-f (Exit Fee) pero nunca definido. Fix: `luka_price_usd: u64` añadido a ProtocolState en 07-a, actualizado por `refresh_vault_valuation`.
  - **C2 (diseño clave):** WhaleDebt modo suave vulnerable a wallets desechables (deuda incobrable). **Decisión de Sebastián: cobro atómico directo en el transfer hook.** Eliminados: WhaleDebt PDA, `collect_whale_debt` instrucción, `WhaleDebtExceedsBalance` error. Nuevo evento `ShieldFeeCollected`. Fee se retiene del monto transferido y se swapea a USDC vía Jupiter CPI → Vault Core.
  - **C1:** Exención staking sin umbral mínimo — dejado como decisión consciente con trade-off documentado (los tiers de fee SON la protección, no el gate de exención). Insight de Sebastián: "las ballenas van a querer salir en algún momento... ¿y en eso no consistía el aumento del fee?"
  - **C3-C6, M1-M6:** clarificaciones de destino de fees ($LUKA→USDC vía Jupiter), scope Anti-Whale (solo ventas en pool, no P2P), firmante register_market_maker (authority + Tridente 3-de-3), desviación ADR-P01 en NEUTRAL documentada, conteos corregidos en índice, PDA numbering, ENZ guard en process_fee.
- **Archivos modificados:** 07-a, 07-f (mayor cambio), 07-e, 07-d, 07-INDICE, 07-CUENTAS-Y-CUSTODIA. La nota "WhaleDebt suave" en BITACORA sesión 5 queda como registro histórico del diseño original antes de la evolución a cobro atómico.
- **Próxima sesión:** Sprint 1 de implementación (07-b cap quema + 07-d drenaje/ENZ), análisis LUKAI, landing a producción, deploy programa devnet.

## 2026-08-20 (sesión 3) — Endurecimiento de seguridad + rutas de auditoría baratas
- **Contrato endurecido v2** (✅ Build successful confirmado): validaciones de inputs, freeze en pausa (switch_motor_b + execute_deferred_burn), protección de cambio de autoridad (no dirección cero), eventos de observabilidad (PauseSet/AdminChangeQueued/AdminChangeExecuted). Basado en sealevel-attacks/Neodyme/Helius. Ya cumplía checked math, has_one, seeds+bump, init anti-reinit, tipos tipados, Timelock.
- **Paquete audit-readiness** (`contracts/AUDIT_READINESS.md`): modelo de amenazas, invariantes, matriz de acceso, herramientas gratis, y **rutas de auditoría capital-cero**: gratis (Sec3/Trident) → **subsidio Areta $1M** (Colosseum fast-track) → grants → boutique $5-20K → Immunefi.
- **Regla de seguridad clave:** en mainnet NUNCA contratos del Vault sin auditar (custodian fondos = honeypot). Devnet sí, libre.
- **Próxima sesión:** desplegar en DEVNET (programa + token $LUKA para verlo) → herramientas gratis → Milestone 2 → Etapa 0 (pitch deck) + Superteam/Colosseum.
