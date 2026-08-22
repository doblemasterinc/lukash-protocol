# Key Decisions — LUKASH Protocol

> ADRs = decisiones cerradas. No contradecir sin un nuevo ADR que lo reemplace.
> Los ADR-P## capturan decisiones ya consolidadas en el Protocolo v4.2 (fuente de verdad).
> Los ADR-0## son decisiones tomadas en las sesiones de trabajo del Studio.

## ADR-001: Protocolo v4.2 es la arquitectura definitiva (2026-08-19)
El `LUKASH_Protocolo_v4_2_FINAL.docx` (carpeta "Back up conversación 16-03", más reciente
que el de raíz) es la fuente técnica de verdad. El BMC debe alinearse a él, no al revés.
Versiones anteriores (v1–v4.1) se conservan en `docs/protocolo/` solo como historial.

## ADR-002: "Aura" reemplaza a "Jaguar Score" (2026-08-19) — Aprobado por Sebastián
El sistema de reputación financiera on-chain pasa a llamarse **Aura**. Es un rebrand puro:
la mecánica NO cambia (niveles Cub 0-499 / Jaguar 500-1999 / Alpha 2000-4999 / Emperor 5000+,
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

## ADR-005: Aura con 5 niveles narrativos (2026-08-19) — Aprobado por Sebastián
Cachorro (0-499) → Rastreador (500-1999) → Cazador (2000-4999) → Alfa (5000-9999) → Jaguar (10000+).
Nombres finales (2026-08-19): el pináculo es "Jaguar" (te conviertes en el jaguar, cierra el arco cachorro→jaguar).
Localización ES/EN/PT: Cachorro/Cub/Filhote · Cazador/Hunter/Caçador · "Jaguar" no se traduce.
Preserva los umbrales v4.2 (500/2000/5000) y añade el pináculo. Gate del cNFT Jaguar Universal en Rastreador
(≥500). Diseño completo en `specs/01-aura-jungle-arena.md`.

## ADR-006: Liberación del Vault Sociedad — regla única Jaguar Lock (2026-08-19) — Aprobado por Sebastián
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
anti-banco/agravio. **Conservar el jaguar y TODA la identidad visual** (no era el problema) reencuadrando su
significado: de "tótem sagrado del pueblo" → "el que asciende/caza/gana" (fuerza individual + pertenencia +
orgullo pan-americano). "Soberanía" siempre como soberanía PERSONAL, no política. Posicionamiento = mezcla de
3 ejes con jerarquía: (1) Dueño de tu ascenso, (2) Tu manada, (3) Tu reserva a la vista; "el banco que vas a
construir" se reserva para Fase 3. Audiencia = todos los menores de 45 (Segmentos A y B), no solo jóvenes.
Operar en **ES/EN/PT** (términos de marca no se traducen: LUKA/KASH/Aura/Jaguar/Manada/LUKAI/Reserva).
Detalle y taglines en `brand/POSICIONAMIENTO_NARRATIVA.md`. Pendiente menor: unificar nombres de los 5 niveles
de Aura (presentación: Cachorro/Rastreador/Guerrero/Líder/Jaguar vs spec: .../Cazador/Jaguar Maduro/Jaguar Sabio).

## ADR-012: Corrección del Anti-Whale + Exit Fee 5% Génesis (2026-08-20) — Aprobado por Sebastián
**Exit Fee:** se mantiene en **5% para el Génesis** (Et.1), 3%/1% en Et.2/3. Aclaración: el "100% al Vault" es
el DESTINO del fee, no una penalización del 100% (el fee es 5/3/1%). Solo se activa en pánico real (precio
<0.7×EMA30 AND venta >0.3% supply/hora) y exime a holders comprometidos.
**Anti-Whale (corregido):** (1) aplica **solo a ventas/transferencias, NO a compras** (quieres ballenas
comprando); (2) el umbral se mide por **% del pool de liquidez, no del supply** — la métrica de supply era
brutal al inicio (1% supply ≈ $10K al TGE, ahogaba el volumen de lanzamiento) y laxa después. Tiers 3/6/10%
sobre el excedente, 100% al Vault. (3) Fix del nombre de exención: "Emperor/Jaguar Maduro" → "Jaguar" (nivel
máx de Aura). Aplicado en Protocolo v4.3 (C10). Se implementa en el Jaguar Shield (Milestone 2).

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

## ADR-015: Jaguar Shield — restauración del Seguro Anti-Exploit + Tridente 3-de-3 + auditor desacoplado (2026-08-21) — Aprobado por Sebastián
Consolida 4 decisiones tras arqueología del historial v1.0→v4.2 (ver `audits/HISTORIAL_JAGUAR_SHIELD.md`):

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

**(5) Mint del token: Token-2022 con Transfer Hook aprobado (2026-08-21) — Aprobado por Sebastián.** El token de mainnet será **Token-2022** (no SPL clásico) para que el Jaguar Shield (Anti-Whale + Jaguar Exit Fee) se aplique **en cada transferencia on-chain**, imposible de evadir vendiendo en otro DEX. Sin esto, el value prop anti-dump quedaría vacío. **Implicaciones operativas:**
- El mint devnet actual (`2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`) es SPL clásico y **se mantiene solo para pruebas Milestone 1**. El mint de mainnet será nuevo (Token-2022).
- Validar antes del TGE: compatibilidad Meteora (LP), Jupiter (routing), CEXs objetivo. Solana Foundation ha empujado adopción Token-2022 desde 2024 y el soporte hoy es amplio (Jupiter y Meteora sí soportan).
- El transfer hook implementa: Jaguar Exit Fee dual (precio<0.7×EMA30 AND venta>0.3% supply/hora) + Anti-Whale por % del pool (ADR-012) + verificación de exenciones (ver punto 6).

**(6) Exenciones canónicas del Jaguar Shield (Anti-Whale + Exit Fee) — de v4.1 §9.1-9.2, ratificadas:**
- **Swaps internos del Motor D** (operaciones dentro del protocolo, no son "ventas de verdad").
- **Staking activo** (holders comprometidos).
- **LP Comprometido en lock activo** (proveen liquidez, no la sacan).
- **LP Fundador 365d** (lock máximo, incentivo estructural).
- **Market Makers registrados en el Tridente Multisig** (registro explícito on-chain, no auto-declarado — el MM debe estar aprobado por Tridente 3-de-3 para figurar en la lista de exentos).
- **Nivel Jaguar de Aura** (Aura ≥10,000, el pináculo — ADR-005 y C10). Antes se llamaba "Emperor"; migrado a "Jaguar" por ADR-002/012.
- **KOLs no tienen exención Anti-Whale/Exit Fee.** El mecanismo que los alinea es distinto: reciben tokens **vesteados** (ADR-011) desde el bucket Marketing/CEX 8%, y el vesting on-chain les impide dumpear (aunque el Anti-Whale sí les aplicaría si lograran vender por encima del umbral, lo cual es improbable con vesting escalonado). Esa es la razón por la que en el manual `specs/05` los KOLs se pagan en tokens vesteados, nunca cash — el vesting hace redundante una exención explícita.

**(7) Auditoría externa vía subsidios — ratificado.** No comprometemos gasto directo. La ruta primaria (ya documentada en `contracts/AUDIT_READINESS.md`) es: (a) herramientas gratis pre-cotización (Sec3 X-Ray, Trident fuzzing, clippy); (b) **subsidio Areta $1M** vía Colosseum fast-track; (c) grants Solana Foundation / Superteam Instagrants; (d) recién si nada anterior alcanza, boutique paga $5-20K o Immunefi bug bounty. **Sin ADR de contratación hasta cotización real.**

Aplicado: parche Protocolo v4.3 §9 · corrección `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` (eliminado el "5%/día" inventado) · `tasks/todo.md` actualizado.

## ADR-016: Drenaje de cola en todos los modos + hard-stop ENZ (2026-08-22) — Aprobado por Sebastián
**Evolución consciente** del diseño original (v2.1→v4.3 decían "al regresar a NORMAL" / "bono de deflación futura"). Razón del cambio: en un bear prolongado (2+ años), la cola se acumula tanto que al normalizar el drenaje de 10%/sem tardaría meses en limpiarla — riesgo de acumulación excesiva. La quema inmediata por tx (25-100% del tramo LP) ya ocurre en todos los modos; lo nuevo es que la cola también drena siempre. Escala geométrica: ACEL 25%/sem · NORMAL 10% · CONS 5% · DEF 2%. El "bono de deflación futura" sigue existiendo (la cola crece en bear porque el inflow supera el drenaje), solo es de menor magnitud. **Hard-stop ENZ:** toda la maquinaria de quema se apaga definitivamente cuando `supply ≤ 3.3B` (alineado con todas las versiones del protocolo). La cola se congela con su saldo residual (destino = DAO Etapa 4). Supply NUNCA baja de 3.3B. Spec `07-d`.

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
0%, nivel Cub), Jaguar Universal (Capas 1/3A, 1.5%, gated por Aura ≥500), Estándar SOL/USDC
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
