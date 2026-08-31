# Key Decisions — LUKASH Protocol

> ADRs = decisiones cerradas. No contradecir sin un nuevo ADR que lo reemplace.
> Los ADR-P## capturan decisiones ya consolidadas en el Protocolo v4.2 (fuente de verdad).
> Los ADR-0## son decisiones tomadas en las sesiones de trabajo del Studio.

## ADR-001: Protocolo v4.2 es la arquitectura definitiva (2026-08-19)
El `LUKASH_Protocolo_v4_2_FINAL.docx` (carpeta "Back up conversación 16-03", más reciente
que el de raíz) es la fuente técnica de verdad. El BMC debe alinearse a él, no al revés.
Versiones anteriores (v1–v4.1) se conservan en `docs/protocolo/` solo como historial.

## ADR-002: Sistema de reputación = "Aura" (2026-08-19) — Aprobado por Sebastián
El sistema de reputación financiera on-chain pasa a llamarse **Aura**. Es un rebrand puro:
la mecánica NO cambia (7 niveles: Cachorro 0-499 / Rastreador 500-1499 / Cazador 1500-2999 /
Alfa 3000-4999 / Emperador 5000-9999 / Shamán 10000-24999 / Titán 25000+,
on-chain, no transferible, decay 2%/semana tras 90d de inactividad, umbral de nivel por
Score máximo histórico). "Aura" no existía en los documentos v4.2; es terminología nueva.
Las "misiones para ganar Aura" = las Misiones de Caza de Jungle Arena.

## ADR-003: Primer milestone de implementación = smart contracts core en devnet (2026-08-19)
El primer entregable construible son los smart contracts (Anchor/Rust) de los Motores A/B/D +
Vault KASH Core + distribución atómica 35/35/15/15 + Throttle dinámico, sobre Solana **devnet**.
No App ni dashboard primero. Condición del protocolo para el presupuesto $500K: diseño técnico
completo y simulado en pseudocódigo ANTES del primer día de desarrollo.

## ADR-004: Estructura del proyecto en el Venture Studio (2026-08-19)
Carpeta `LUKASH/` con scaffold completo (CLAUDE.md, .claude/knowledge, specs, tasks, audits,
contracts, brand, docs). Todos los documentos fuente de "PROYECTO CRYPTO" migrados y organizados
por categoría dentro de `docs/` y `brand/`. La carpeta original queda como respaldo intacto.

## ADR-005: Aura con 7 niveles narrativos (2026-08-19, actualizado 2026-08-26) — Aprobado por Sebastián
7 niveles del Camino del Rugido — progresión: habilidad física → liderazgo → trascendencia:
Cachorro (0-499) → Rastreador (500-1499) → Cazador (1500-2999) → Alfa (3000-4999) → Emperador (5000-9999) → Shamán (10000-24999) → Titán (25000+).
Nombres en español, son marca y no se traducen entre ES/EN/PT.
Gate del Tótem Universal en Rastreador (≥500). Exención Anti-Whale/Exit Fee en Titán (≥25000).
Diseño completo en `specs/01-aura-jungle-arena.md`.

## ADR-006: Liberación del Vault Sociedad — regla única KASH Lock (2026-08-19) — Aprobado por Sebastián
Corrige la discrepancia detectada: un solo hito gobierna la liberación de accionistas. Disparador =
**KASH Core alcanza $30M o 12 meses** (lo que ocurra primero). Fees a la Sociedad líquidos desde el
**mes 13** (dentro de Etapa 2B); capital principal lineal en 48 meses. **Etapa 3 (K>$50M) NO reinicia**
esta distribución (su foco es Motor C/fiat/escala). Se elimina "(TVL)": el $30M es sobre el KASH Core.

## ADR-007: Protocolo v4.3 es la base canónica de implementación (2026-08-19)
`docs/protocolo/LUKASH_Protocolo_v4.3.md` consolida v4.2 + todas las correcciones de auditoría (C1-C9).
Es la fuente para el código. Los .docx v4.2 quedan como respaldo histórico.

## ADR-008: Cifras Monte Carlo — usar modelo conservador v3.1, etiquetado (2026-08-19)
Decisión de Sebastián: dejar la reconciliación Monte Carlo para después. Mientras tanto, las cifras
publicadas ($37.2M/113x/0.0%/1.4%) se etiquetan como "modelo conservador v3.1", NO se atribuyen al
código v4.2 (que da valores 10-200× irreales). Sharpe/drawdown no reproducibles: retirados hasta recalcular.

## ADR-009: Lanzamiento faseado — comunidad primero, solo el token, sin distribución a holders (2026-08-19)
Estrategia aprobada por Sebastián. Secuencia: F0 comunidad (pre-token) → F1 solo cripto (token + Vault +
dashboard, EL ÁTOMO) → F2 App (Manadas + Aura) → F3/F4 instrumentos. **Regulatoria:** en F1 $LUKA es moneda
transaccional que construye su reserva, pero **la reserva NUNCA se reparte a holders** (solo respaldo). Sin
dividendos/yield a holders hasta F3/F4 con licencia. Marketing vende identidad/misión/transparencia, NUNCA
retornos (esto mantiene el perfil de utilidad, no security). Cada fase gated por tracción + licencia.
Estrategia completa en `specs/03-estrategia-lanzamiento-comunidad.md`. Alcance completo enfocado por el
Concepto Objetivo de Socio (`docs/discovery/CONCEPTO_OBJETIVO_SOCIO.md`).

## ADR-010: Reposicionamiento de marca sin ideología + trilingüe (2026-08-19) — Aprobado por Sebastián
Quitar toda lectura política/colectivista: fuera "pueblo", cadencias tipo "por/para la manada", tono
anti-banco/agravio. **Conservar el felino y TODA la identidad visual** (la imagen no era el problema, la palabra "jaguar" sí —
asociaciones con populismo LATAM) reencuadrando su significado: "el que asciende/caza/gana" (fuerza individual +
pertenencia + orgullo pan-americano). "Soberanía" siempre como soberanía PERSONAL, no política. Posicionamiento =
mezcla de 3 ejes con jerarquía: (1) Dueño de tu ascenso, (2) Tu manada, (3) Tu reserva a la vista; "el banco que
vas a construir" se reserva para Fase 3. Audiencia = todos los menores de 45 (Segmentos A y B), no solo jóvenes.
Operar en **ES/EN/PT** (términos de marca no se traducen: LUKA/KASH/Aura/Manada/LUKAI/Reserva/Titán/Shamán/Emperador).
Detalle y taglines en `brand/POSICIONAMIENTO_NARRATIVA.md`. Niveles de Aura (7): Cachorro→Rastreador→Cazador→Alfa→Emperador→Shamán→Titán.

## ADR-012: Corrección del Anti-Whale + Exit Fee 5% Génesis (2026-08-20) — Aprobado por Sebastián
**Exit Fee:** se mantiene en **5% para el Génesis** (Et.1), 3%/1% en Et.2/3. Aclaración: el "100% al Vault" es
el DESTINO del fee, no una penalización del 100% (el fee es 5/3/1%). Solo se activa en pánico real (precio
<0.7×EMA30 AND venta >0.3% supply/hora) y exime a holders comprometidos.
**Anti-Whale (corregido):** (1) aplica **solo a ventas/transferencias, NO a compras** (quieres ballenas
comprando); (2) el umbral se mide por **% del pool de liquidez, no del supply** — la métrica de supply era
brutal al inicio (1% supply ≈ $10K al TGE, ahogaba el volumen de lanzamiento) y laxa después. Tiers 3/6/10%
sobre el excedente, 100% al Vault. (3) Fix del nombre de exención: nivel máx de Aura = **Titán** (≥25000).
Aplicado en Protocolo v4.3 (C10). Se implementa en el KASH Shield (Milestone 2).

## ADR-011: Lanzamiento community-only (fair-launch) como ruta primaria; MM/influencers para después (2026-08-19)
Con poco capital: **Ruta A = fair-launch comunidad-only** (liquidez propia lockeada/quemada en Meteora + volumen
100% orgánico + anti-dump: vesting on-chain, Exit Fee, Anti-Whale). El **Market Maker se quita de todo material
público** (indicación de Sebastián) y queda como **Ruta B (opcional, para cuando haya tracción/capital)**, con
directorio real y contactos documentados. KOLs = hispanos de educación financiera pagados en **tokens vesteados,
nunca cash**. Manual completo (ambas rutas + directorios MM e influencers + best practices) en
`specs/05-manual-lanzamiento.md`. Contratos en un solo archivo para compilar en Playground: `contracts/playground/`.
Trade-off honesto: sin MM el volumen depende 100% de la comunidad — más limpio pero más exigente (ver auditoría integral).

## ADR-013: 45% Venta = Seed dinámico con rollover a Public (2026-08-20) — Aprobado por Sebastián
El 45% "Venta (Seed → Public)" (4,500M) se dimensiona así: **Seed ≤5% del supply (≤500M, ~11% del 45%)**,
vendido bajo SAFE + token warrant con descuento y **vesting escalonado on-chain** + disclosure público.
**Ventana del Seed cierra en el TGE**; lo NO vendido **rueda automáticamente a Public** (fair-launch,
circulante desde TGE), anunciado on-chain. **Public ≥40% del supply (≥4,000M).** Si Seed vendido = 0 → 100%
del 45% es Public → "cero venta privada" es literalmente cierto (narrativa fair intacta). El Seed solo se
activa si grants + Colosseum no cubren runway (coherente con financiación no-dilutiva primero, ADR-011).
MM y KOLs NO salen de aquí: tienen su bolsa en el 10% Marketing/CEX. Aplicado en Protocolo v4.3 (tabla §supply).

## ADR-014: Bucket Equipo/Fundador 2% + economía de fundador en 3 capas (2026-08-20) — Aprobado por Sebastián
Se crea fila explícita **Equipo/Fundador = 2% (200M)**, fondeada recortando **Marketing/CEX 10%→8%** (KOLs/MM
quedan con 800M, suficiente). Vesting **cliff 12m + lineal 48m** on-chain pre-TGE (espejo del Vault Sociedad;
cliff 12m reemplaza el 18m "no negociable" anterior — decisión consciente del fundador, sigue siendo creíble).
Resuelve el hueco de la tabla (antes sumaba 100% sin fila de Equipo). Razón: NO es ingreso (para eso está el
O&M), es **alineación + gobernanza + upside del token** (el Vault Sociedad da upside del negocio, no del precio
de $LUKA — son streams distintos). **Economía de fundador en 3 capas:** (1) **O&M 15%** = compensación operativa
por trabajar (corto plazo); (2) **Equipo 2%** = upside del token + voz en DAO (largo, vesteado); (3) **Vault
Sociedad ≥10% del 30%** = patrimonio del negocio (largo). Se descartó 0% (pureza fair-launch) por la señal de
"sin piel en el juego" que incomoda a ángeles/Colosseum. Aplicado en v4.3 (tabla §supply + vesting).

## ADR-015: KASH Shield — restauración del Seguro Anti-Exploit + Tridente 3-de-3 + auditor desacoplado (2026-08-21) — Aprobado por Sebastián
Consolida 4 decisiones tras arqueología del historial v1.0→v4.2 (ver `audits/HISTORIAL_KASH_SHIELD.md` — archivo renombrado desde HISTORIAL_JAGUAR_SHIELD.md):

**(1) Seguro Anti-Exploit restaurado con condiciones de v1.0/v4.1** (la v4.2 canónica las había perdido y mutado "1×/12 meses" en "renovación anual", que ya no significa lo mismo):
- Cobertura: **hasta 5% del valor total del Vault**
- Frecuencia: **máximo 1 evento cada 12 meses**
- **Activo desde Etapa 2B** (K ≥ K_min $25M, Motor B2 activado on-chain) — condición on-chain, no calendario. Alinea el seguro con el momento en que la cobertura tiene sentido económico (5% de $25M+ = ≥$1.25M) y cubre el tramo estadísticamente más vulnerable de un protocolo DeFi. Descarta "año 1" (ambiguo) y "solo Etapa 3+" (deja sin defensa el tramo vulnerable).
- **Desde Etapa 4 (DAO):** el control pasa a la comunidad, que redefine estas reglas.
- **NO hay "5% de retiro por día"** — ese concepto nunca existió en tu documentación, fue un artefacto del informe cuantitativo que el análisis previo arrastró por error.

**(2) Tridente Multisig 3-de-3 ratificado** (introducido en v2.3, se mantiene). Es la firma para: activar Capa 3 (cBTC Reserva Profunda), cancelar Circuit Breaker LP, modificar K_min. Los 3 firmantes concretos = **decisión pendiente**, se define pre-TGE.

**Patrón de implementación aprobado (2026-08-21):** el Tridente se **construye completo pero nace INACTIVO** (`tridente_activated = false`), con toda la lógica del 3-de-3 codificada, testeada y auditable desde el inicio. Mientras esté inactivo, cualquier operación que lo requiera falla con `TridenteNotActivated`. Instrucción one-way `activate_tridente(pk1, pk2, pk3)` que solo la authority actual puede llamar, con validación de pubkeys distintas y no-default. **Candado estructural:** el contrato **rechaza el paso a Etapa 2** si el Tridente no está activado (`require!(tridente_activated, TridenteRequiredForStage2)`) — el propio contrato fuerza la activación pre-TGE, sin depender de memoria humana. En devnet Milestone 1-2 se opera en Etapa 1 (Génesis) con Tridente inactivo — Sebastián como authority única.

**(3) Auditor externo desacoplado.** Las menciones a "Halborn / OtterSec" que arrastraba la documentación desde v2.1 no fueron decisión formal — eran sugerencias en tablas de próximos pasos. Se retiran; queda **"auditor(a) externa a cotizar pre-TGE"** (Halborn, OtterSec, Sec3, Zellic, Neodyme o similar). La cotización real irá en un ADR de contratación cuando se decida.

**(4) Regla de higiene documental:** la v4.2 canónica perdió/mutó decisiones tuyas de v1.0 y v4.1. Cuando algo en v4.2/v4.3 se sienta raro o ambiguo, contrastar con v1.0 y v4.1 antes de dar por buena la canónica. Se anota en `learned-rules.md`.

**(5) Mint del token: Token-2022 con Transfer Hook aprobado (2026-08-21) — Aprobado por Sebastián.** El token de mainnet será **Token-2022** (no SPL clásico) para que el KASH Shield (Anti-Whale + KASH Exit Fee) se aplique **en cada transferencia on-chain**, imposible de evadir vendiendo en otro DEX. Sin esto, el value prop anti-dump quedaría vacío. **Implicaciones operativas:**
- El mint devnet actual (`2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`) es SPL clásico y **se mantiene solo para pruebas Milestone 1**. El mint de mainnet será nuevo (Token-2022).
- Validar antes del TGE: compatibilidad Meteora (LP), Jupiter (routing), CEXs objetivo. Solana Foundation ha empujado adopción Token-2022 desde 2024 y el soporte hoy es amplio (Jupiter y Meteora sí soportan).
- El transfer hook implementa: KASH Exit Fee dual (precio<0.7×EMA30 AND venta>0.3% supply/hora) + Anti-Whale por % del pool (ADR-012) + verificación de exenciones (ver punto 6).

**(6) Exenciones canónicas del KASH Shield (Anti-Whale + Exit Fee) — de v4.1 §9.1-9.2, ratificadas:**
- **Swaps internos del Motor D** (operaciones dentro del protocolo, no son "ventas de verdad").
- **Staking activo** (holders comprometidos).
- **LP Comprometido en lock activo** (proveen liquidez, no la sacan).
- **LP Fundador 365d** (lock máximo, incentivo estructural).
- **Market Makers registrados en el Tridente Multisig** (registro explícito on-chain, no auto-declarado — el MM debe estar aprobado por Tridente 3-de-3 para figurar en la lista de exentos).
- **Nivel Titán de Aura** (Aura ≥25,000, el pináculo — ADR-005). Nivel máximo del sistema Aura.
- **KOLs no tienen exención Anti-Whale/Exit Fee.** El mecanismo que los alinea es distinto: reciben tokens **vesteados** (ADR-011) desde el bucket Marketing/CEX 8%, y el vesting on-chain les impide dumpear (aunque el Anti-Whale sí les aplicaría si lograran vender por encima del umbral, lo cual es improbable con vesting escalonado). Esa es la razón por la que en el manual `specs/05` los KOLs se pagan en tokens vesteados, nunca cash — el vesting hace redundante una exención explícita.

**(7) Auditoría externa vía subsidios — ratificado.** No comprometemos gasto directo. La ruta primaria (ya documentada en `contracts/AUDIT_READINESS.md`) es: (a) herramientas gratis pre-cotización (Sec3 X-Ray, Trident fuzzing, clippy); (b) **subsidio Areta $1M** vía Colosseum fast-track; (c) grants Solana Foundation / Superteam Instagrants; (d) recién si nada anterior alcanza, boutique paga $5-20K o Immunefi bug bounty. **Sin ADR de contratación hasta cotización real.**

Aplicado: parche Protocolo v4.3 §9 · corrección `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` (eliminado el "5%/día" inventado) · `tasks/todo.md` actualizado.

## ADR-016: Drenaje de cola en todos los modos + hard-stop ENZ (2026-08-22) — Aprobado por Sebastián
**Evolución consciente** del diseño original (v2.1→v4.3 decían "al regresar a NORMAL" / "bono de deflación futura"). Razón del cambio: en un bear prolongado (2+ años), la cola se acumula tanto que al normalizar el drenaje de 10%/sem tardaría meses en limpiarla — riesgo de acumulación excesiva. La quema inmediata por tx (25-100% del tramo LP) ya ocurre en todos los modos; lo nuevo es que la cola también drena siempre. Escala geométrica: ACEL 25%/sem · NORMAL 10% · CONS 5% · DEF 2%. El "bono de deflación futura" sigue existiendo (la cola crece en bear porque el inflow supera el drenaje), solo es de menor magnitud. **Hard-stop ENZ:** toda la maquinaria de quema se apaga definitivamente cuando `supply ≤ 3.3B` (alineado con todas las versiones del protocolo). La cola se congela con su saldo residual (destino = DAO Etapa 4). Supply NUNCA baja de 3.3B. Spec `07-d`.

## ADR-017: Capa 2 sin programa mock separado — swaps como accounting a oráculo (2026-08-25) — Aprobado por Sebastián
Sprint 5B implementa Capa 2 (interacciones reales on-chain) sin requerir un programa mock swap separado. En devnet, `execute_vault_swaps` convierte USD pendientes a balances nativos a precio de oráculo Pyth (accounting puro). En mainnet, misma instrucción haría CPIs reales a Jupiter V6. Quema real via CPI `token::burn`; oráculos BTC/SOL via deserialización manual Pyth V2 (sin dependencia `pyth-sdk-solana`). `refresh_vault_valuation` es permissionless.

## ADR-P01: Vault KASH Core 100% Solana-nativo — composición RESUELTA (Protocolo v4.3)
Composición canónica (suma 100%): cBTC 35% · SOL 15% · SOL/LST 20% · USDC reserva 25% · USDC lending 5%.
Los oráculos (PYTH/Switchboard/Jupiter) son **infraestructura operativa (O&M), no reserva** (resuelve la
discrepancia del 105% de v4.2, ADR corregido). 0% riesgo de bridge. Nunca liquidable; solo crece.
Determina el Precio KASH (P_KASH) y activa Motor B2 en K_min=$25M.

## ADR-P02: Regla universal 35/35/15/15 (Protocolo v4.2)
Todos los motores y todas las capas del Motor D distribuyen cada fee así:
35% Vault/Asset Layer · 35% LP/Quema · 15% O&M · 15% Staking. Inamovible.

## ADR-P03: Emisión Neta Cero al alcanzar 3.3B supply (Protocolo v4.2)
Supply inicial 10B → objetivo 3.3B (quemar 6.7B). Al llegar a 3.3B, PDA on-chain suspende
la quema y redirige el valor capturado a recompensas perpetuas. $LUKA pasa a comportarse
como acción preferente con dividendos reales del Vault.

## ADR-P04: R_op NO interviene en el flujo normal del Motor B (corrección clave v4.2)
La Reserva Operativa USDC (30% del yield del Vault) se acumula IDLE como reserva de
estabilización de último recurso. En B0 el tramo LP quema directo sin USDC; en B2 el precio
se recupera por inercia estructural (Motor A + Exit Fee + Anti-Whale + cola diferida +
arbitraje natural). Cualquier residuo v3.1 que diga lo contrario es un error a corregir.

## ADR-P05: cNFT = instrumento financiero tokenizado, no llave de acceso (Protocolo v4.2)
Cada cNFT representa una posición de inversión (capital + rendimiento). APY variable,
transferible, con disclaimer on-chain obligatorio en metadata. 3 tipos: Nativo $LUKA (Capa 0,
0%, nivel Cachorro), Tótem Universal (Capas 1/3A, 1.5%, gated por Aura ≥500), Tótem Estándar SOL/USDC
(Capa 3B, 2%, sin gate).

## ADR-P06: El Vault NO se toca para defender el precio en B2 (Protocolo v4.2)
En B2 consolidado la defensa del precio opera vía flujos de fees, no tocando el Vault existente
ni la R_op. La redención soberana directa al Precio KASH queda como opción del DAO en Etapa 3,
nunca automática.

## ADR-P07: Hub legal El Salvador (CNAD), expansión todo LatAm + Brasil + España (Protocolo v4.2)
Fundación en El Salvador (0% impuestos cripto, capital mínimo $2K, tarifa registro ~$5.5K).
Lanzamiento primario simultáneo Colombia + México. Expansión escalonada por prioridad y CAC.

## ADR-P08: Inversión por fases $500K / $2M / $5M autofinanciada (Protocolo v4.2)
Solo la Fase 1 requiere capital externo. Fases 2 y 3 se co-financian con el Vault Sociedad
acumulado en Etapa 1. Sin dependencia de VCs, sin dilución de socios fundadores.

## ADR-017: Rebrand completo — eliminación de la palabra "jaguar" (2026-08-26) — Aprobado por Sebastián
La palabra "jaguar" se elimina de TODO el vocabulario activo del proyecto. Razón: asociaciones con
retórica populista en LATAM (economías jaguar, identidad ideológica) que chocan con ADR-010 (cero
ideología). La identidad visual del felino SE CONSERVA (isotipo, imágenes) — el problema era la
palabra, no la imagen. Cambios aplicados en contratos, specs, audits, brand, landing, docs y simulaciones:

**Seguridad:** Jaguar Shield → **KASH Shield** · Jaguar Exit Fee → **KASH Exit Fee** · Jaguar Lock → **KASH Lock**
**Niveles de Aura (7 niveles, ADR-005 actualizado):** Cachorro (0-499) → Rastreador (500-1499) → Cazador (1500-2999) → Alfa (3000-4999) → Emperador (5000-9999) → Shamán (10000-24999) → Titán (25000+).
  Nombres en español, son marca y no se traducen.
**cNFTs:** Los tipos pasan a llamarse **Tótems**: Tótem Nativo / Tótem Universal / Tótem Estándar.
**Productos:** Jaguar Pay → **LUKASH Pay** · Jaguar Chat → **LUKASH Chat** · Jaguar Games → **LUKASH Games** · Jaguar AI → **LUKAI**
**Narrativa:** Camino del Jaguar → **Camino del Rugido** · El Resplandor del Jaguar → **El Rugido de la Manada**
**Comunidad:** "jaguares fundadores" → "rugidores fundadores" o "fundadores"

Archivos históricos (`extracted/historial/`, `docs/presentaciones/`, `docs/app/`) conservan la terminología
original como registro. La única mención permitida de "jaguar" en docs activos es como referencia al
motivo del cambio (ej: "la palabra jaguar fue eliminada porque...").

## ADR-018: Redistribución del presupuesto $500K (2026-08-28) — Aprobado por Sebastián
La distribución original (App 50% / Liquidez 25% / Marketing 15% / Legal 10%) subdimensionaba
marketing dado que las simulaciones muestran 44% espiral de muerte con campaña débil vs 0% con
moderada/agresiva. Nueva distribución:
- **App/Tech 35% ($175K):** contratos ya 70% hechos, auditoría boutique $15-25K, frontend MVP, LUKAI v1.
- **Liquidez inicial 20% ($100K):** pool $LUKA/SOL en Meteora con LP lock permanente.
- **Marketing + MM contingente 30% ($150K):** KOLs hispanos (tokens vesteados, $0 cash), plataformas
  orgánicas, community manager, ads, MM contingente $40-60K (activar solo si volumen <$500K/día por 5d),
  CEX listing $30K.
- **Legal 10% ($50K):** abogado cripto, entidad El Salvador, framing regulatorio.
- **Reserva operativa 5% ($25K):** O&M fundador meses 1-6, emergencias.

## ADR-019: Estrategia MM/KOL en 3 fases — separación servicios vs equity (2026-08-28) — Aprobado por Sebastián
Confirma y extiende la regla de sesión 4: **tokens vesteados = pago por servicio (MM/KOLs).
KASH Sociedad = solo para capital/largo plazo (inversores).** Nunca mezclar.
- **Fase 1 (TGE, día 1-30):** Sin MM. Ruta A community-only con liquidez propia ($100K).
  KOLs hispanos educativos pagados en $LUKA vesteado (cliff 6mo + 12mo linear) del pool
  Marketing/CEX (8% supply). Prioridad: Hugo Botto, Catalina Castro, Daniel Muvdi, Criptolawyer.
- **Fase 2 (mes 2-3):** MM contingente (Kairon Labs o Gravity Team). Modelo loan + call option
  (1-3% supply como inventario). Gate de activación: volumen diario <$500K por 5 días consecutivos.
  Compatible con fair launch (ADR-011).
- **Fase 3 (mes 4+):** CEX listing (MEXC/Gate.io). Algunos listan gratis si volumen >$500K/día.

## ADR-020: Rondas de inversión escalonadas + roadmap KASH Sociedad (2026-08-28) — Aprobado por Sebastián
El KASH Sociedad (30% de los fees del protocolo) se distribuye en rondas con valoración creciente:
- **Ronda Seed (pre-TGE):** ángeles cripto-nativos, $500K por 5-8%, valoración implícita ~$6-10M.
- **Ronda Estratégica (mes 6-12, post-tracción):** entidades financieras LatAm (fintechs, cooperativas,
  remesadoras), $1-2M por 5-8%, valoración implícita ~$15-25M.
- **Distribución resultante:** ~17% fundador / 5-8% seed / 5-8% estratégica / ~6-13% reserva DAO/futuro.
Razón del escalonamiento: (1) inversores ángeles invierten en visión y diseño, entidades financieras
invierten en tracción demostrada; (2) la naturaleza blockchain exige legitimidad en el ecosistema cripto
ANTES de buscar socios TradFi; (3) la revalorización entre rondas es el incentivo de early-stage para ángeles.

## ADR-021: Identidad del fundador — pseudónimo conocido (2026-08-28) — Aprobado por Sebastián
El fundador opera con exposición escalonada por niveles de confianza:
- **Inversores/legal/auditoría:** identidad completa (KYC, contrato).
- **Equipo/socios:** nombre real, comunicación directa.
- **Comunidad pública:** alias consistente + voz (AMAs sin cámara), historial verificable del proyecto.
- **Prensa/redes:** el proyecto y su narrativa, no la persona.
Razones: seguridad personal en LatAm, flexibilidad regulatoria pre-framing legal, protección ante
phishing/extorsión dirigida. Se re-evalúa el doxx público en mes 6-12 según tracción y contexto.
LinkedIn/perfil profesional NO menciona LUKASH en etapas tempranas.

## ADR-022: Tridente → Guardian de Pausa con sunset (2026-08-28) — Aprobado por Sebastián
El Tridente Multisig se redefine como **Guardian de Pausa** con alcance ultra-limitado:
- **Solo puede:** pausar el protocolo + vetar operaciones pendientes en timelock.
- **No puede:** mover fondos, modificar parámetros, hacer upgrades.
- **Firma:** 2-de-3 (antes 3-de-3). Tolera la pérdida de un firmante.
- **Sunset automático:** se desactiva on-chain al llegar a Etapa 3 (K>$50M) o cuando el DAO lo vote.
Razón del cambio: reducir superficie regulatoria (no controla fondos → debilita argumento Howey de
"esfuerzos de otros"), menor dependencia operativa, y mantener la red de seguridad contra exploits
zero-day que los mecanismos automáticos (CB, Exit Fee, Anti-Whale) no pueden detectar. El candado
estructural que bloquea Etapa 2 sin Tridente activado se mantiene.

## ADR-023: Eliminación de Capa 0 — staking solo a través de Tótems (2026-08-28) — Aprobado por Sebastián
La Capa 0 del Motor D (staking puro, fee 0%, "exención absoluta e incondicional") se **elimina**. El staking
deja de existir como producto independiente. La única forma de stakear $LUKA es depositando en un **Tótem
Nativo** (Capa 1, fee 1.5%). Motor D queda con 3 capas: Capa 1 (1.5%), Capa 2 (3%/3.5%), Capas 3A/3B
(1.5%/2%). Razones: (1) simplifica la UX — el usuario no necesita entender "staking", solo Tótems;
(2) cada depósito alimenta la Reserva Sagrada (35/35/15/15 desde la primera interacción); (3) elimina el
riesgo de que staking masivo a 0% deje al protocolo sin generación de fees en ese tramo. El rendimiento
de los holders de Tótems sigue viniendo del 15% Staking de todos los demás motores.

## ADR-024: Tótems con tiers visuales por monto + Avatar como identidad NFT (2026-08-28) — Aprobado por Sebastián
**Tótems por tiers:** Cada categoría de Tótem (Nativo, Universal, Estándar) tiene 3 tiers visuales
diferenciados por monto de depósito. Misma mecánica, mismo fee, mismo rendimiento proporcional — la
diferencia es estética (color/aspecto) y de capital comprometido:
- **Nativo:** Bronce (10K $LUKA) / Plata (50K) / Oro (100K). Fee 1.5%. Sin requisito de Aura.
- **Universal:** Bronce (50K $LUKA) / Plata (200K) / Oro (500K). Fee 1.5%. Aura ≥500.
- **Estándar:** Tiers y montos por definir en SOL/USDC. Fee 2%. Etapa 3.
Montos de referencia, ajustables pre-TGE. El usuario elige su compromiso; el tier es aspiracional y
coleccionable. Se puede depositar más LUKA después para subir de tier.

**Avatar = identidad del usuario, separada de los Tótems:**
- Al crear cuenta, el usuario recibe un **avatar in-app** (off-chain, sin costo). Evoluciona visualmente
  con el Aura (Cachorro → ... → Titán). Representa al usuario en Duelos, Manadas y leaderboards.
- Después de una misión milestone (por definir: Aura ≥500 o misión específica), el avatar puede
  **mintearse como NFT completo** (Metaplex, no comprimido) pagando un monto en $LUKA (fee por definir).
- El NFT Avatar es **identidad on-chain**: uno por usuario, único, transferible, con metadata dinámica
  que refleja el Aura actual. Diferente de los Tótems (instrumentos financieros, cNFTs, múltiples por usuario).
- Mercado secundario: un Avatar con historial de Shamán o Titán tiene valor intrínseco (reputación incluida).

**Tótem Nativo gated por misión educativa:** El primer Tótem Nativo no se compra desde un menú — se
desbloquea al completar las primeras misiones del Sendero del Aprendiz (LUKAI explica qué es un Tótem,
cómo funciona, qué genera). Al final de la misión se ofrece la compra en 3 tiers (Bronce/Plata/Oro).

## ADR-025: Sellos de Manada — cNFT custom para membresía y acceso (2026-08-28) — Aprobado por Sebastián
Se crea una nueva categoría de cNFT: el **Sello de Manada**, creado por el líder de una Manada para
representar y distribuir membresía/acceso. El nombre visible lo define el creador; el sistema lo
llama "Sello" internamente. Requisito para crear: Aura ≥ Cazador (1,500+).

**Tipos de Manada que el Sello habilita:**
- **Vaca:** ahorro grupal para una meta (fin de año, vacaciones). El Sello = tu cuota.
- **Fondo:** crowdlending entre miembros. El Sello = tu participación.
- **Negocio:** crowdfunding. El Sello = tu stake en el proyecto.
- **Evento:** ticket de acceso verificable on-chain, revendible.
- **Club:** membresía abierta con acceso a marketplace interno, descuentos, votación.

El líder configura: tipo, cupo máximo, aporte mínimo/máximo, duración, meta de capital, arte del Sello.
Todos los fees pasan por Motor D Capa 2 (3% en $LUKA / 3.5% en SOL) → distribución 35/35/15/15.
LUKAI actúa como árbitro de disputas (P2P entre miembros — el protocolo NO presta ni capta).
Las Manadas son funcionalidad core desde el lanzamiento de la App/Web.

## ADR-026: Educación financiera multi-track + gaming integrado (2026-08-28) — Aprobado por Sebastián
LUKASH incluye un sistema completo de educación financiera gamificada con **múltiples tracks en paralelo**
(no secuenciales). El usuario avanza simultáneamente en cada área según su interés:

**4 tracks con niveles progresivos (fácil → experto):**
- **Finanzas personales:** ahorro, presupuesto, deuda, fondo de emergencia, inversión, planificación fiscal.
- **Finanzas del ciudadano:** bancos, tasas, inflación, impuestos, política monetaria, macroeconomía.
- **Cripto y blockchain:** Bitcoin, wallets, exchanges, seguridad, DeFi, smart contracts, auditoría.
- **LUKASH:** Reserva Sagrada, Tótems, Manadas, Motores, Capas, DAO.

**Formato:** lecciones cortas (2-3 min), formato tipo TikTok (video + reto), generadas/personalizadas
por LUKAI. Streaks por días consecutivos. Cada lección da XP al Tótem (habilidades) y Aura al usuario.
LUKAI detecta el nivel del usuario en cada track y sugiere la próxima lección.

**Gaming (Jungle Arena) integrado:**
- **Duelo de Tótems:** tu criatura vs otra, stats basados en actividad real, apuesta en LUKA.
- **Caza del Tesoro:** LUKAI esconde drops, pistas diarias, consume Energía.
- **Conquista de Territorio:** Manadas reclaman zonas en un mapa con Stake de Batalla.
- **Predicción Relámpago:** ¿sube o baja? 1 predicción diaria, streak.
- **Misiones de LUKAI:** retos personalizados diarios (ahorra, invita, aprende, comparte).
- **Retos virales / Proof of Roar:** contenido verificable sobre LUKASH → validado por LUKAI → Aura.
- **Battle Pass mensual ("Temporada de Caza"):** track gratuito + premium, misiones estacionales.

Toda la actividad de gaming opera en Motor D Capa 2 (3% $LUKA). La educación financiera ocurre como
consecuencia del juego — el gameplay es lo primero, divertido para un joven de 13 años y para un
adulto. Las lecciones no son quizzes aburridos sino retos interactivos y visuales.

## ADR-027: Totem Guard — seguro digital con reaseguro externo (2026-08-28) — Aprobado por Sebastián
Sistema de protección de activos digitales en dos capas con mecanismo de reaseguro:

**(1) Capa 1 — Póliza externa de activos digitales (financiada por O&M):**
- Un porcentaje del 15% O&M (por definir pre-TGE, estimado 5-10% del O&M) se destina a contratar
  una **póliza real de activos digitales** con proveedor externo especializado (a cotizar: Evertas,
  Coincover, Aon Digital, Nexus Mutual, u otro).
- Cubre: **Vault KASH Core** contra exploits de smart contract, hacks, manipulación de oráculos.
- Reemplaza la frase histórica "financiado por Halborn u OtterSec" (que ADR-015(3) ya aclaró no era
  decisión formal) con un mecanismo real de financiación.
- Activa desde que O&M genere ingresos suficientes (Etapa 2A en adelante).
- Esta póliza actúa como **reaseguro** del producto interno Totem Guard.

**(2) Capa 2 — Totem Guard (producto interno para usuarios):**
- **Qué es:** seguro paramétrico opt-in que protege los Tótems del usuario contra eventos catastróficos.
- **Activación:** toggle en la App sobre cada Tótem. Periodo de espera: 30 días (anti-gaming).
- **Prima:** micro-prima periódica (mensual) en $LUKA, proporcional al valor del Tótem.
  El pago de la prima es transacción Motor D Capa 1 (1.5% fee → distribución 35/35/15/15).
  La prima neta va al **Pool Totem Guard** (PDA on-chain, auditable).
- **Primas de referencia** (% mensual del valor del Tótem, moduladas por Aura):
  Cachorro 0.5% · Rastreador 0.4% · Cazador 0.35% · Alfa 0.3% · Emperador+ 0.25%.
  Montos ajustables pre-TGE y por gobernanza (Timelock 48h).
- **Eventos cubiertos** (paramétricos — pago automático, sin claims ni votación):
  (a) Exploit de smart contract confirmado (Guardian de Pausa activado por 2-de-3).
  (b) Manipulación de oráculo confirmada (post-mortem verificado por LUKAI + Guardian).
- **NO cubre:** caída normal de mercado, venta voluntaria, pérdida de wallet/claves, errores del usuario.
- **Payout:** automático, proporcional al valor asegurado del Tótem al momento del evento.
  Cobertura máxima: 80% del valor del Tótem. Máx 1 payout cada 12 meses.
- **Mecanismo de reaseguro:** el Pool Totem Guard está respaldado por la póliza externa (Capa 1).
  Si un payout excede el pool interno, la póliza externa cubre la diferencia (hasta el límite contratado).

**Cascada ante exploit (concepto aprobado — detalle de cadena pendiente de diseñar):**
1. Guardian de Pausa se activa (2-de-3) → se congela el protocolo.
2. Post-mortem confirma exploit → se califica como evento cubierto.
3. **Primera pérdida → Pool Totem Guard** paga a usuarios asegurados (proporcional a su Tótem).
4. **Segunda pérdida → Póliza externa (O&M)** cubre el gap si el pool Totem Guard es insuficiente.
5. **La Reserva Sagrada NO se toca.** El principio "solo crece" es inviolable.
   El Seguro Anti-Exploit del Vault (ADR-015, 5%) queda como herramienta de última instancia
   disponible solo para el DAO en Etapa 4 — NO como mecanismo automático pre-DAO.

**Por qué este diseño:**
- El usuario no financia la póliza externa — sale de O&M (el protocolo se autoasegura).
- Totem Guard es opt-in — quien quiere protección extra la paga y sus primas alimentan el ecosistema.
- Paramétrico = sin burocracia, sin claims, pago automático. Simple para un joven de 15 años.
- Reaseguro cierra el círculo: pool interno no necesita ser gigante.
- **La Reserva Sagrada nunca se usa para cubrir pérdidas** — las dos capas de seguro existen
  precisamente para protegerla.
- Diferenciador: ningún protocolo DeFi en Solana ofrece seguro embebido con reaseguro real.

**PENDIENTE:** diseño detallado de la cadena de cascada (porcentajes, límites por capa, términos
de la póliza externa, interacción con ADR-015 en contexto DAO Etapa 4).

## ADR-029: Totem Guard → ASU + isotipo como base visual de Tótems + 120 misiones (2026-08-31) — Aprobado por Sebastián
El producto de seguro interno "Totem Guard" pasa a llamarse **ASU** (decisión personal del fundador,
nombre de marca). La mecánica no cambia (ADR-027). Cambios de naming:
- **Totem Guard** → **ASU** en toda documentación activa, contratos y UI.
- **Pool Totem Guard** → **Pool ASU**.
- La cascada ante exploit: Pool ASU → Póliza externa (O&M) → Reserva NO se toca.

**Isotipo V2 como base visual de Tótems:** el isotipo del proyecto (felino-bóveda-infinito
ciber-bioluminiscente, `brand/logos/ISOTIPO V2.png`) se usa como modelo base para los Tótems,
con variaciones de color, tamaño y efectos según categoría (Nativo/Universal/Estándar) y tier
(Bronce/Plata/Oro). Esto unifica la identidad visual: el Tótem ES el isotipo de LUKASH, no un
asset genérico. Los assets visuales se generarán con IA de imagen (Midjourney/Leonardo/Ideogram)
usando el isotipo como prompt base.

**Catálogo de 120 misiones de Aura** creado en `specs/08-catalogo-misiones-aura.md`:
7 senderos (A. Aprendiz 15 / B. Escuela Financiera 40 / C. Rastro Diario 12 / D. Rugido 18 /
E. Tótem 10 / F. Manada 12 / G. Estacionales 13). Organizados por nivel de usuario, etapa del
protocolo y tipo (one-time/diaria/semanal/mensual/evento). ~45 misiones activas desde Etapa 1,
~100 en Etapa 2A, 120 completas + rotativas mensuales.

## ADR-028: Narrativa central — Banco Central Optimizado (2026-08-31) — Aprobado por Sebastián
$LUKA como moneda transaccional con reserva propia replica el modelo de banco central pero
corrigiendo sus defectos fundamentales:

| Dimensión | Banco Central tradicional | LUKASH Protocol |
|---|---|---|
| **Emisión** | Moneda emitida contra deuda soberana | Supply fijo (10B), deflacionario hasta piso 3.3B |
| **Reserva** | Construida por deuda y política monetaria | Construida por transacciones reales (35% fees → Vault) |
| **Tendencia** | Inflacionaria por diseño (2-10%/año, peor en LatAm) | Deflacionaria: quema reduce supply, Vault crece |
| **Piso de precio** | No existe (la moneda puede devaluarse sin límite) | P_KASH = Vault / supply (piso auditable on-chain) |
| **Transparencia** | Opaca (decisiones de comité cerrado) | Smart contract auditable, Vault verificable 24/7 |
| **Gobernanza** | Centralizada en institución gubernamental | Guardian 2-de-3 → DAO (Etapa 4) |

**Implicación para inversores:** LUKASH no es "otro token DeFi" — es la versión optimizada de
la política monetaria, democratizada para 650M de latinoamericanos que viven la devaluación
como experiencia cotidiana. El mismo concepto (reserva respalda moneda), sin la deuda, sin la
inflación, con transparencia total. Es infraestructura financiera, no especulación.

**Uso en pitch:** esta narrativa debe ser el segundo slide del pitch deck (después del problema)
y el primer párrafo del litepaper. No presentar LUKASH como "crypto project" sino como
"optimized monetary infrastructure".
