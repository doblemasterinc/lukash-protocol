# TODO — LUKASH

## 📋 PLAN DE TRABAJO CONTINUO (orden de ejecución)

### FASE A — Decisiones estratégicas (sesión actual, ago-2026)
- [x] A1. Revisión BMC v4.2 punto por punto (12 secciones analizadas)
- [x] A2. Redistribución de costos $500K (de 50/25/15/10 → 35/20/30/10/5)
- [x] A3. Estrategia MM/KOL en 3 fases (TGE orgánico → MM contingente mes 2-3 → CEX mes 4+)
- [x] A4. Separar tokens vesteados (servicios) de KASH Sociedad (equity) — regla confirmada
- [x] A5. Rondas de inversión escalonadas: Seed ángeles $500K → Estratégica fintech $1-2M
- [x] A6. Roadmap KASH Sociedad (30%): ~17% fundador / 5-8% seed / 5-8% estratégica / ~6-13% reserva
- [x] A7. Identidad del fundador: pseudónimo conocido con exposición por niveles
- [x] A8. Gaming, Tótems, Avatar, educación financiera y Sellos de Manada — ADR-022 a 026 (sesión 18, 2026-08-28)
- [x] A9. Totem Guard — seguro digital con reaseguro externo: póliza O&M (Capa 1) + microseguro paramétrico para Tótems (Capa 2). ADR-027 (sesión 18, 2026-08-28)
- [x] A10. Sandbox híbrido — ABSORBIDO en Jungle Arena (sesión 18). No se construye sandbox separado; la Arena ya gamifica las interacciones DeFi. Evolución visual tipo DeFi Land diferida post-Etapa 2
- [x] A11. Diseño detallado de Manadas: Sellos de Manada como cNFT custom para membresía — ADR-025
- [x] A12. Consolidar BMC v4.3 con ADR-022 a 027 (sesión 18, 2026-08-28): tabla de cambios, §04 Relaciones, §12 Aura, §13 KASH Shield actualizados
- [x] A13. Registrar decisiones como ADRs — ADR-022 a 026 registrados

### FASE B — Preparación inversión (pre-ronda seed)
- [ ] B1. Estructura legal: entidad en El Salvador o equivalente, framing regulatorio cNFT/Motor C
- [x] B2. Pitch deck para ángeles cripto-nativos — **Completado** (sesión 20, 2026-08-31): 12 slides con ADR-028 (Banco Central Optimizado), MC v4.3, KASH Shield, roadmap, ask. HTML+PDF en `docs/LUKASH_Pitch_Deck_v4_3.*`
- [x] B3. Litepaper v1 — **Completado** (`docs/etapa-0/LITEPAPER_v1.md`, 22.6KB). Pendiente: data room formal con proyecciones y Vault Sociedad.
- [x] B4. One-pager EN/PT — **Completado** (`docs/etapa-0/ONE_PAGER_en.md` + `ONE_PAGER_pt.md`)
- [ ] B5. Preparar dashboard de Vault en testnet como pieza de credibilidad
- [~] B6. Identificar y contactar 10-20 ángeles cripto — **Pipeline de 36 contactos creado** (sesión 19, 2026-08-31): 6 ángeles tier 1 (Santiago Roel Santos máx prioridad), 6 aceleradoras, 12 fondos, 6 grants, 6 redes. Artefacto publicado. Pendiente: contactar.
- [x] B7. Preparar aplicaciones a grants — **Textos listos** (sesión 20, 2026-08-31): 5 aplicaciones redactadas en `docs/etapa-0/GRANT_APPLICATIONS.md` (Superteam Instagrants, Finternet x Solana, Solana Foundation, Colosseum, Alliance DAO). Finternet cerrado al 31/08, monitorear reapertura. Superteam listo para enviar inmediatamente.
- [ ] B8. Consulta legal: abogado cripto (framing cNFT como no-security, Motor C como no-transmisión)

### FASE C — Pre-lanzamiento (F0 comunidad, 8-12 semanas pre-TGE)
- [ ] C1. Reservar handles (X, TG, Discord) con alias del fundador
- [ ] C2. Founding myth + hype de misterio ("ruge desde el génesis")
- [x] C3. Landing en producción (Vercel + Supabase waitlist) — **Desplegada** (sesión 21): https://landing-chi-livid-91.vercel.app
- [ ] C4. Configurar plataformas orgánicas: Zealy (quests), Galxe (credenciales), Guild.xyz (roles)
- [ ] C5. Contactar KOLs hispanos educativos (Hugo Botto, Catalina Castro, Daniel Muvdi, Criptolawyer)
- [ ] C6. Contratar community manager 24/7
- [ ] C7. Whitelist gamificada con tiers (Alfa/Emperador/Cachorro) + snapshot pre-TGE
- [ ] C8. Dashboard del Vault en testnet público como marketing de transparencia
- [ ] C9. AMAs de voz (X Spaces, Discord) — voz sin cara en primeras etapas
- [ ] C10. Contenido 30 días: threads educativos, Reels, memes, founding myth

### FASE D — Técnica (en paralelo con B y C)
- [x] **D0. Build + deploy v10.2 en Solana devnet** (sesión 19, 2026-08-31): Guardian 2-de-3, Capa 0→1.5%, Totem Guard placeholder. Build ✅ + Deploy ✅ devnet.
- [x] **D0b. Ejecutar MC v4.3** (sesión 19, 2026-08-31): suite completa SIMs 0-5 (200 iters × 3 campañas).
  - SIM 0: invariantes contrato ✅ (dist 100%, OM=STK, supply floor, muro OK).
  - SIM 1: MC principal — CONSERV K_med=$90M espiral=40% / BASE K_med=$388M espiral=0% / AGRESIVO K_med=$1.88B espiral=0%.
  - SIM 2: sensibilidad — Volumen (#1, swing $1.29B) > Fee Motor A ($278M) > Yield ($6M) > App day ($2M) > K_min ($0).
  - SIM 3: estrés — Crash BTC -80% solo -3.8%. Exploit 15% Vault -0.3%. Motor A -70% perm único riesgo (espiral 5.8%).
  - SIM 4a: Throttle umbrales actuales óptimos (todos dan K_med=$367M). SIM 4b: masa crítica marginal ($381M→$395M).
  - **SIM 5: MM vs No-MM** — MM sube Vault 11-19% pero baja precio 10-25% por dilución. Solo CONSERV se beneficia (espiral 34.5%→3.5%). Valida ADR-011/019 (fair-launch + MM contingente).
  - Salidas en `simulations/out/`. Pendiente: actualizar cifras en BMC/protocolo con rangos MC v4.3.
- [x] **D0c. Crear PDF/versión legible de la presentación v4.3** — **Completado** (sesión 20, 2026-08-31): combinado con B2 en pitch deck 12 slides. PDF 667KB en `docs/LUKASH_Pitch_Deck_v4_3.pdf`
- [ ] D1. Migración Token-2022 con Transfer Hook (devnet)
- [ ] D2. Contratos Milestone 2 restantes: staking, cNFT/Aura, CPI safety
- [ ] D3. Auditoría estática + Soteria CI (ya configurado) — mantener limpio
- [ ] D4. Frontend App MVP (wallet + dashboard Vault + LUKAI v1 keeper)
- [ ] D5. Deploy Milestone 2 en devnet 2+ semanas sin bugs
- [ ] D6. Auditoría externa pre-mainnet (Halborn/OtterSec vía subsidio Areta/Colosseum)

### FASE E — TGE y lanzamiento (Día 0)
- [ ] E1. Vesting equipo on-chain publicado ANTES del TGE
- [ ] E2. Activar Guardian de Pausa 2-de-3 (T1-T2 de bloqueantes, ADR-022)
- [ ] E3. Pool $LUKA/SOL en Meteora con $100K + LP lock permanente
- [ ] E4. Lanzamiento coordinado: listing + vault funding + influencers simultáneos
- [ ] E5. LUKASH Games semana 1: Spot the Whale, Meme Wars, The First Hunt
- [ ] E6. Monitoreo: volumen diario, wallets activas, retención Discord

### FASE F — Post-TGE (mes 1-6)
- [ ] F1. Monitoreo del gate MM: si volumen <$500K/día por 5d → activar Kairon Labs o Gravity Team
- [ ] F2. Utility bombs semanales: App beta (holders), marketplace, LUKAI preview
- [ ] F3. Partnership avalanche: 1 anuncio/semana (Jupiter, Phantom, merchant)
- [ ] F4. Campaña CEX listing (MEXC/Gate.io) cuando volumen sostenido
- [ ] F5. Recopilación de métricas (Vault growth, volumen, holders) para ronda estratégica
- [ ] F6. Desarrollo Jungle Arena / gaming (A8-A11 ya cerrados para esta fase)

### FASE G — Ronda estratégica (mes 6-12)
- [ ] G1. Pitch a entidades financieras con tracción demostrada (Vault creciendo, volumen real, 3-6 meses data)
- [ ] G2. Target: fintechs LatAm, cooperativas de ahorro, remesadoras (inversión $1-2M por 5-8% KASH Sociedad)
- [ ] G3. Integración Motor C con socios financieros
- [ ] G4. Evaluación de doxx público del fundador según tracción y contexto de seguridad

---

## 🚨 BLOQUEANTES PRE-TGE MAINNET (recordar en cada session-close)

Estos NO son bloqueantes de desarrollo (devnet funciona sin ellos), pero **el TGE mainnet no puede ocurrir sin resolverlos**. El contrato tiene un candado estructural que rechaza el paso a Etapa 2 si el Tridente no está activado, así que Sebastián no puede olvidarlos por accidente — pero el trabajo de identificar y crear los wallets es humano y toma tiempo.

- [ ] **T1. Identificar 2 firmantes adicionales del Guardian de Pausa (2-de-3, ADR-022)** — hoy solo está Sebastián. Los 3 deben ser distintos y ninguno puede ser la authority. Idealmente hardware wallets (Ledger). Publicar pubkeys en ADR. Nota: ya no es 3-de-3, basta 2-de-3.
- [ ] **T2. Ejecutar `activate_tridente(pk1, pk2, pk3)`** post-deploy mainnet → renombrar a `activate_guardian`. **Instrucción one-way / irreversible.** Sin esto, el contrato bloquea el paso a Etapa 2. Guardian solo puede pausar/vetar, no mover fondos. Sunset automático en Etapa 3.
- [ ] **T3. Crear multisig O&M** (2-de-3 con Squads Protocol) para el 15% operativo de fees. Hoy la wallet es single-sig de Sebastián — antes del TGE debe ser multisig con firmantes documentados en ADR (fundador + dev senior + tercero de confianza).
- [ ] **T4. LP Fundador con lock 365d on-chain** verificable antes del TGE.
- [ ] **T5. Auditoría externa** completada vía subsidio (Colosseum / Areta / Superteam) — no bloqueante si se hace en paralelo pero no puede saltarse.
- [ ] **T6. Deploy Milestone 2 en devnet 2+ semanas sin bugs** antes de mainnet.

Detalle completo y procedimientos en [`specs/07-milestone-2/07-CUENTAS-Y-CUSTODIA.md`](../specs/07-milestone-2/07-CUENTAS-Y-CUSTODIA.md) §7 (checklist pre-TGE).

## ⭐ PRÓXIMA SESIÓN (empezar por aquí)
### Prioridad: EJECUCIÓN DE MERCADO (no más build)
1. [ ] **Decidir ADR Ruta 3** — % Vault Sociedad para inversores (habilita conversaciones)
2. [ ] **Enviar Superteam Instagrants** — application lista, copiar-pegar
3. [ ] **Reservar handles** — X (@LukashProtocol), Telegram, Discord
4. [ ] **Primer outreach** — Santiago Roel Santos (ángel T1 máx prioridad)
5. [ ] **Dashboard Vault devnet** — pieza de credibilidad técnica para demos
6. [ ] **Hospedar prototipo v2** como URL pública

### Historial de hitos técnicos completados
0. [x] **★ AUDITORÍA PROFUNDA DE LOS MOTORES por SIMULACIÓN FIEL AL CONTRATO** (sesión 5): suite `simulations/` (engine=réplica de lib.rs + capa económica prima/Markov/vesting). SIM 0 invariantes OK · SIM 1 MC (espiral CONSERV 44%/BASE 0%/AGRESIVO 0% → tesis lanzamiento agresivo confirmada) · SIM 2 sensibilidad (volumen driver #1) · SIM 3 estrés (resiliente salvo Motor A −70% perm) · SIM 4 throttle irrelevante al Vault. Reporte `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` + Artifact `19a8160e`.
   - [x] **Herramientas estáticas DESBLOQUEADAS** (sesión 14, 2026-08-26): GitHub Actions CI con clippy (lints DeFi: arithmetic_side_effects, unwrap_used, expect_used) + Soteria (25+ vulns Solana) + anchor build + anchor test. Sec3 X-Ray descartado (requiere cuenta). Auditoría manual de seguridad: 3 CRITICAL + 2 HIGH + 3 MEDIUM + 5 LOW + 12 patrones positivos → reporte `audits/SECURITY_AUDIT_LIB_RS_V9_1.md`.
   - [x] **Security hardening v10** (sesión 14): 7 fixes aplicados, Build + Deploy ✅ en devnet. Fixes: has_one=authority en process_fee/refresh_vault_valuation/execute_vault_swaps (CRITICAL×3), Pyth owner validation via pyth_oracle::ID (CRITICAL), DEVNET_MODE flag para fallbacks (HIGH), confidence interval check conf/price<2% (MEDIUM), safe u64::try_from en update_market_regime (MEDIUM).
   - [x] **Specs Milestone 2 en `specs/07-milestone-2/`** (2026-08-21, sesión 5): 07-INDICE + 6 specs (07-a a 07-f). Base: ADR-015.
   - [x] **07-b ratificada ✅** (cap quema 1%/día)
   - [x] **07-d ratificada ✅** (drenaje siempre activo 25/10/5/2 + hard-stop ENZ, ADR-016)
   - [x] **07-a ratificada ✅** (Switch B0→B2 por Pyth + Token Accounting real, persistencia 7d aprobada)
   - [x] **07-e ratificada ✅** (módulo contra-cíclico LUKAI, mayoría 3 señales + fail-safe NEUTRAL)
   - [x] **07-c ratificada ✅** (KASH Shield + Tridente inactivo + CB + Seguro Anti-Exploit)
   - [x] **07-f ratificada ✅** (Anti-Whale + Exit Fee + Token-2022 Transfer Hook + WhaleDebt suave)
   - Orden de implementación: Sprint 1 (07-b + 07-d) → Sprint 2 (07-a) → Sprint 3 (07-c + 07-e) → Sprint 4 (07-f) → Sprint 5 (endurecimiento + auditoría + mainnet).
   - [x] **Sprint 1 implementado** (sesión 8): 07-b (cap quema 1%/día) + 07-d (drenaje 4 modos + ENZ hard-stop) en `lib.rs` v3 (828 líneas). Pendiente: compilar en Playground.
1. [x] **Compilar Sprint 1 en Playground + deploy devnet** — Build ✅ + Deploy ✅ (2026-08-22). Program Id: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`.
2. [x] **Sprint 2 (07-a) Capa 1**: Switch B0→B2 por valoración de mercado + token accounting + doble candado 7d. Implementado + desplegado en devnet.
   - Capa 2 implementada en Sprint 5B (sesión 12): quema real + oráculos Pyth + swaps a oráculo.
   - [x] **Sprint 3 (07-c + 07-e) implementado** (sesión 10): KASH Shield (Tridente inactivo + CB 24h + Seguro Anti-Exploit) + módulo contra-cíclico LUKAI (régimen BULL/NEUTRAL/BEAR, mayoría 3 señales, fail-safe NEUTRAL 48h) en `lib.rs` v5 (1365 líneas). Pendiente: compilar en Playground + deploy devnet.
   - [x] **Sprint 4 (07-f) implementado + desplegado + verificado** (sesión 10-11): Anti-Whale (tier 3/6/10% sobre excedente de 1% pool) + KASH Exit Fee (dual: <0.7×EMA30 AND >0.3% supply/h, 5/3/1% por etapa) + Transfer Hook Capa 1 + MMRegistry PDA + sell_pressure rodante 1h + LP Fundador ATA + exenciones diferenciadas (AW:6, Exit:5 sin staking) en `lib.rs` v6 (1718 líneas). **Build + Deploy + Anti-Whale verificado en devnet** (sesión 11).
   - [x] **Sprint 5A (security hardening) implementado + desplegado + verificado** (sesión 11): 6 fixes de seguridad en `lib.rs` v7 (1756 líneas). Fix #1 (CRÍTICO): burns decrementan `current_supply` (ENZ ya no es dead letter). Fix #2: revoke_mm cierra PDA. Fix #3: LP Fundador ATA valida pubkey. Fix #4: insurance amount>0. Fix #5: CB active check. Fix #6: `close_protocol` para devnet. **Fix #1 verificado: 14M tokens quemados correctamente tras process_fee Motor A $100.**
   - [x] **Sprint 5B Fase A (Capa 2: quema real)** (sesión 12): CPI `token::burn` real en `process_fee` y `execute_deferred_burn`. `lib.rs` v8 (1867 líneas). Nuevos: `initialize_burn_vault` (PDA token account), `burn_vault` en contexts, evento `RealBurnExecuted`, dependencia `anchor-spl`.
   - [x] **Sprint 5B Fase B (Capa 2: oráculos Pyth)** (sesión 12): `refresh_vault_valuation` permissionless con lectura directa de feeds Pyth devnet (BTC/USD, SOL/USD). Deserialización manual Pyth V2 (`parse_pyth_price` + `pyth_price_to_usd6`). LST/LUKA como parámetros. `lib.rs` v9.
   - [x] **Sprint 5B Fase C (Capa 2: swap adapter)** (sesión 12): `execute_vault_swaps` permissionless a precio de oráculo Pyth. `process_fee` acumula `pending_swap_*_usd`, el keeper ejecuta los swaps. Devnet: accounting puro (mock). Helper `usd_to_native`. `lib.rs` v9 (2066 líneas).
   - [x] **Sprint 5B Fase D (Capa 2: integración)** (sesión 12): RUNBOOK_DEVNET.md actualizado con secuencia de deploy completa v9, flujo operativo, documentación de `execute_vault_swaps`. `initialize_vaults` no necesario — vault ATAs se crean con `initialize_burn_vault` + fondeo manual. client.ts pendiente de actualizar en Playground.
   - [x] **Deploy v9 en devnet** (sesión 12 cont): Build + Deploy ✅ (Slot 488093788). Migración v7→v9 (UncheckedAccount close). initialize + initialize_burn_vault + fondeo 1M LUKA ✅.
   - [x] **Testing v9.1 en devnet**: updateOracleState ✅. refreshVaultValuation ✅ (v9.1 fallbacks Pyth). process_fee Motor A ✅ (sesión 13, 2026-08-26).
   - [x] **Post-fix Pyth**: process_fee Motor A $100 → quema real 14M tokens (14 LUKA a $0.10) ✅, vault_core_usd +980K ✅, pending_swap_*_usd acumulados 980K total (cBTC 367K/SOL 157K/LST 210K/USDC-r 204K/USDC-l 41K) ✅. Tx: Q1BBq6hmekT8caxANoNysF9k1D1cktGoxiQTStrcB6YCtiNBLx4jkNCiMLaf3KdtGvWftHPmCMfM9xXC1D6dRy6.
   - [x] **Post-fix Pyth**: execute_vault_swaps ✅ (sesión 13, 2026-08-26). Pending 980K USD convertidos a nativos: cBTC +565 sat, SOL +1.05M lam, LST +1.4M lam, USDC-r +204K (1:1), USDC-l +41K (1:1). Pending limpiados a 0. Tx: 2bmSoiE2efowtj8BCJ2YG82wXt7CiJ6ZRJJjA126kme1NXNjcKf2nteiyEbTP32d6MLX2BGjicaS85hiM5U8ScgD.
   - [x] **CAPA 2 COMPLETA** (sesión 13): process_fee (quema real CPI + distribución atómica + pending swaps) + execute_vault_swaps (USD→native a precio Pyth) verificados end-to-end en devnet v9.1.
   - [x] **Security hardening v10.1 verificado en devnet** (sesión 15, 2026-08-26): has_one=authority en process_fee/refreshVaultValuation/executeVaultSwaps PASS. Pyth owner validation movida a runtime (constraint Anchor bloqueaba fallback devnet; mainnet valida pyth_oracle::ID). Quema real 14M tokens PASS. Vault +980K PASS. Pending swaps→nativos PASS. Client scripts actualizados (caller→authority). RUNBOOK actualizado.
3. [x] **Análisis de recursos de LUKAI** completado → `docs/analisis/LUKAI_COSTO_ARQUITECTURA.md`. Resultado: v1.0 keeper $150-500/mes (sin LLM), v2.0 con routing 80/20 Haiku/Sonnet = $1.77/usuario/mes. Break-even a ~$3K O&M (Haiku mínimo) o ~$9K (routing). **Recomendación: solo keeper v1.0 hasta Etapa 2A con O&M>$10K/mes.**
4. [x] **Landing → producción** (sesión 21, 2026-08-31): HTML optimizado de 3.6MB a 49KB (base64→archivos). Watermarks Gemini removidos (5 imágenes). Waitlist Supabase operativa (proyecto oilo-miranda). **Desplegada en Vercel:** https://landing-chi-livid-91.vercel.app
   - [x] Crear tabla `waitlist` en Supabase (SQL en `landing/supabase-setup.sql`)
   - [x] Obtener URL + anon key de Supabase y reemplazar placeholders en `landing/index.html`
   - [x] Deploy a Vercel: proyecto `doblemasterincs-projects/landing`, GitHub conectado
   - [ ] Configurar dominio custom cuando se tenga
   - [ ] Renombrar proyecto Vercel de "landing" a "lukash"
   - [x] Regenerar `feature-app.jpg` — actualizada con HOME "Magic Sensei", sin "JAGUAR Pay" (sesión 23)
5. **Etapa 0 (continuar)**: [x] one-pager ES, [x] founding myth + micro-myth (ES/EN/PT), [x] landing, [x] one-pager EN/PT, [x] litepaper v1. [ ] data room formal (tokenomics completo + Vault Sociedad + proyecciones). Arrancar Superteam Earn + grant Finternet.
5b. [ ] **Estrategia Ruta 3 (ADR-018)**: decidir % del Vault Sociedad a ceder, # inversores, estructura legal, capital target, KASH Lock. Fair launch compatible.
5c. [x] **Prototipo app v2** (`prototype/lukash-app-v2.html`): rediseño visual completo con concept art, 6 tabs, SVG icons, assets de marca reales, glassmorphism. Sesión 22, 2026-08-31.
5d. [x] **Prototipo app v2 upgrade** (sesión 23, 2026-09-01): LUKAI chat funcional (11 temas), 10+ misiones interactivas (quiz/simulator/checkin/prediction/share/deposit/join), minting de tótems con 3 pasos animados + localStorage, terminología corregida (RESERVA SAGRADA), splash 5s, teléfono 880px, isotipo 48px.
5e. [x] **Biblioteca completa de prompts visuales** (sesión 24, 2026-09-02): avatares 14 (7 niveles × M/F) + accesorios 126 piezas (6 tiers: Bronce 30, Plata 30, Oro 30, Obsidiana 15, Platino 15, Diamante 6) + 3 ejemplos avatar+accesorio equipado. Todo en `brand/prompts/`. INDICE.md completo.
   - [ ] Condensar prompts old-format (Cachorro M/F, Cachorra F, Rastreadora F, Cazador M/F) al formato anti-texto condensado (prevenir que Gemini renderice texto)
6. **Contratos Milestone 2 (restante)**: integración real SPL/Jupiter, quema real, staking, cNFT/Aura, oráculo Pyth+Switchboard, CPI safety.
6. [x] **Sub-estructura del 45% "Venta"** decidida → Seed dinámico→Public (ADR-013) + fila Equipo/Fundador 2% (ADR-014).


## Decisiones pendientes de Sebastián
- [ ] **DECISIÓN — ADR Ruta 3 (ADR-018)**: % del Vault Sociedad a ceder a inversores, estructura, KASH Lock. Necesario antes de contactar ángeles.
- [ ] **DECISIÓN — H10 (Monte Carlo)**: elegir modelo oficial de cifras (recomiendo v3.1 conservador) + etiquetar fuente.
- [ ] **DECISIÓN — marca**: D1 ticker ($LUKA), D2 niveles de Aura (naming), D3 Tótems cNFT

## Diseño pendiente (no bloqueante para outreach)
- [ ] Cerrar pendientes Jungle Arena: costos de Energía, reglas de Duelos, pools de Temporada, fórmula de Aura con pesos
- [ ] Mercado secundario cNFTs: fee de venta, herencia del modo de rendimiento, actualización de Aura al transferir
- [ ] Frecuencia de distribución Modo B (semanal/mensual, fijo/elegible)
