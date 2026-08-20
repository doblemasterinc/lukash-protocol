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

## ADR-011: Lanzamiento community-only (fair-launch) como ruta primaria; MM/influencers para después (2026-08-19)
Con poco capital: **Ruta A = fair-launch comunidad-only** (liquidez propia lockeada/quemada en Meteora + volumen
100% orgánico + anti-dump: vesting on-chain, Exit Fee, Anti-Whale). El **Market Maker se quita de todo material
público** (indicación de Sebastián) y queda como **Ruta B (opcional, para cuando haya tracción/capital)**, con
directorio real y contactos documentados. KOLs = hispanos de educación financiera pagados en **tokens vesteados,
nunca cash**. Manual completo (ambas rutas + directorios MM e influencers + best practices) en
`specs/05-manual-lanzamiento.md`. Contratos en un solo archivo para compilar en Playground: `contracts/playground/`.
Trade-off honesto: sin MM el volumen depende 100% de la comunidad — más limpio pero más exigente (ver auditoría integral).

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
