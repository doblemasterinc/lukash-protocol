# Auditoría de Consistencia — Protocolo LUKASH v4.2 + BMC v4.2

> Fecha: 2026-08-19 · Auditor: Claude (AIRQUITECT) · Alcance: Protocolo v4.2 FINAL + BMC v4.2 FINAL
> Método: lectura íntegra de ambos documentos + verificación numérica de cada tabla + cruce Protocolo↔BMC.
> El Protocolo es la fuente de verdad; el BMC debe alinearse a él.

## Resumen ejecutivo

El diseño económico es **excepcionalmente sólido y maduro**. La mayoría de las tablas numéricas
cruzan bien entre ambos documentos (ver "Verificaciones OK"). Sin embargo hay **1 discrepancia
crítica** (la composición del Vault suma 105%) y varias inconsistencias menores de nomenclatura
y ambigüedades que conviene cerrar **antes de codificar los smart contracts**, porque afectan
directamente la lógica on-chain (distribución de fees, composición del Vault).

## Tabla de hallazgos

| # | Severidad | Ubicación | Discrepancia | Corrección sugerida |
| --- | --- | --- | --- | --- |
| H1 | 🔴 CRÍTICA | Protocolo §4.1 y §18 | La composición del Vault suma **105%**: cBTC 35 + SOL 15 + LST 20 + USDC Reserva 25 + USDC Lending 5 + Oráculos 5 = 105% | Rebalancear a 100%. Decisión de Sebastián (ver opciones abajo) |
| H2 | 🟠 ALTA | Protocolo §6.1/§6.5; BMC §5.2/§5.7 | El flujo atómico del Motor A compra solo 5 activos (cBTC 35·USDC 25·SOL 15·LST 20·lending 5 = 100%) y **omite el 5% de oráculos** (PYTH/JTO/JUP). No hay mecanismo definido para adquirirlos | Definir si los oráculos se financian del fee (y de qué bucket) o se adquieren aparte |
| H3 | 🟡 MEDIA | Protocolo §4.2 | El modelo de 3 capas del Vault lista **cBTC en Capa 2 (Trabajo Activo) Y en Capa 3 (Reserva Profunda 35%)** a la vez | Aclarar: ¿todo el cBTC es reserva profunda, o una parte trabaja? |
| H4 | 🟡 MEDIA | Protocolo §3.1 y §4.3 | Dos "70/30" distintos sobre bases distintas: (a) cada fee → 70% Core/30% Sociedad; (b) el yield del Vault → 70% reinversión/30% R_op. Fácil de confundir | Renombrar uno de los dos splits para desambiguar |
| H5 | 🟡 MEDIA | Protocolo §5 título vs cuerpo; BMC §07 | El componente se llama **"LUKAI v4.1"** pero sus versiones desplegables son **v1.0 y v2.0**. Versionado ambiguo | Fijar un esquema: p.ej. "LUKAI (release v1.0 orquestador / v2.0 IA)" sin el "v4.1" |
| H6 | 🟡 MEDIA | Protocolo §4.5 ("P_th") vs BMC §5.6/§10.1 ("P_KASH") | Mismo concepto (Precio KASH), dos símbolos distintos | Unificar a un solo símbolo (recomiendo **P_KASH**) |
| H7 | 🟢 BAJA | Protocolo §18 | Fee "Motor B — Etapa 1 = 2.5%", pero el Motor B **no se activa hasta Etapa 2A** (en Génesis solo hay Motor A) | Reetiquetar como "Motor B — desde Etapa 2A" |
| H8 | 🟢 BAJA | Protocolo §1.1 vs §18 | Capa 3B "opera idéntico al Motor C" pero su fee es 2% y el de Motor C es 0.5% | Aclarar que "idéntico" = mecánica de distribución, no la tasa |
| H9 | 🟢 BAJA | Protocolo §3 (capital 40/30/30) vs flujos (70/30 Core/Sociedad) | El Vault Core es 40% del capital inicial pero recibe 70% del componente KASH de cada fee | Nota aclaratoria: son bases distintas (capital semilla vs flujo de fees) |

## Detalle de hallazgos críticos y altos

### H1 — 🔴 La composición del Vault suma 105% (CRÍTICA)

Es la discrepancia más importante y afecta directamente al smart contract del Vault.

**Protocolo §4.1** ("Composición del Vault v4.2") y **§18** (Blueprint de parámetros) listan:

| Activo | Asignación |
| --- | ---: |
| cBTC (Bitcoin nativo) | 35% |
| SOL nativo | 15% |
| SOL/LST (JitoSOL+mSOL) | 20% |
| USDC Reserva Inmediata | 25% |
| USDC Lending | 5% |
| PYTH + JTO + JUP | 5% |
| **TOTAL** | **105%** ❌ |

Esto es imposible como asignación de portafolio. Además, el flujo operativo del Motor A
(§6.1, §6.5, y BMC §5.2/§5.7) usa una versión **diferente y de 100%** que **elimina los oráculos**:
`[35% cBTC · 25% USDC · 15% SOL · 20% LST · 5% lending]` = 100%.

**Interpretación:** los 5 activos "no-oráculo" ya suman 100% por sí solos. El 5% de oráculos
está "de más". Hay que decidir de dónde sale ese 5%.

**Opciones de corrección (requieren tu decisión):**
- **Opción A** — Bajar USDC Reserva de 25% → 20%, y meter oráculos 5%. (Reduce el buffer de emergencia.)
- **Opción B** — Bajar SOL/LST de 20% → 15% y meter oráculos 5%. (Reduce exposición con yield.)
- **Opción C** — Los oráculos NO son parte de la asignación del Vault (son infraestructura operativa
  pagada del O&M), y la composición oficial es la de 5 activos = 100%. (La más limpia; evita comprar
  PYTH/JTO/JUP como "reserva".)

> Recomiendo la **Opción C**: tratar PYTH/JTO/JUP como gasto de infraestructura (O&M), no como reserva
> de valor. Un token de gobernanza de oráculo no es "activo duro de respaldo". Así el Vault queda con
> 5 activos que suman 100% y coincide con lo que el Motor A realmente compra hoy en el flujo §6.1.

### H2 — 🟠 El Motor A nunca compra los oráculos

Consecuencia directa de H1: si los oráculos son 5% del Vault (§4.1) pero el flujo de compra del
Motor A (§6.1) no los incluye, **el Vault nunca los adquiere** por el mecanismo de fees. Queda un
activo "fantasma" en la tabla de composición sin ruta de adquisición. Se resuelve con la decisión de H1
(si eliges Opción C, desaparece; si A/B, hay que añadir el 4º/5º swap al flujo atómico del Motor A).

## Mapa Fase (inversión) vs Etapa (protocolo) — desambiguación

El proyecto usa dos ejes que es fácil confundir. No son lo mismo:

| Eje | Valores | Qué define |
| --- | --- | --- |
| **Etapa** (protocolo §12) | 0, 1, 2A, 2B, 3, 4 | Qué motores están activos y qué condiciones on-chain se cumplen |
| **Fase** (inversión §15) | 1, 2, 3 | Tramos de capital ($500K / $2M / $5M) y foco de desarrollo |
| **Estado del Motor B** | B0, B2 | B0=quema directa (K<$25M), B2=recirculación (K≥$25M). Es un estado, no una etapa |

Relación aproximada: Fase 1 ≈ Etapas 0-1 (+ prototipo App). Fase 2 ≈ Etapas 2A-2B. Fase 3 ≈ Etapa 3.
Etapa 4 (DAO) es post-Fase 3. **Recomiendo fijar esta tabla como canónica** en el protocolo para
evitar que un lector mezcle "Fase 2" con "Etapa 2".

## Inventario de PENDIENTES (del propio protocolo)

**Bloqueantes para smart contract cNFT:**
1. Mercado secundario cNFTs: fee de venta, herencia del modo de rendimiento, actualización de Aura al transferir.
2. Frecuencia de distribución Modo B: ¿semanal o mensual? ¿fija por instrumento o la elige el usuario?

**Pendientes Fase App:**
3. **Jungle Arena — mecánicas de juego detalladas** (niveles, misiones concretas, Bet & Win, economía sostenible). ← *lo que Sebastián quiere trabajar (ganar Aura)*
4. Manadas — parámetros de producto (tipos, límites de capital, gobernanza interna, árbitro LUKAI para disputas).
5. Go-to-market local por país (Colombia, México, El Salvador, Argentina, Brasil-PT, resto).
6. Yield sharing DeFi externo Capa 3 (gestora patrimonial descentralizada).
7. Aura (Jaguar Score) — ajuste de puntajes post-datos reales (parámetros de lanzamiento, ajustables por gobernanza con Timelock 48h).

## Verificaciones que PASARON ✅ (dan confianza)

- **Distribución 35/35/15/15**: suma 100% y se aplica consistente en los 4 motores y todas las capas del Motor D.
- **Distribución de supply §2.1**: 45+30+10+10+5 = 100% (4.5B+3B+1B+1B+0.5B = 10B). Correcto.
- **Arquitectura de capital §3**: LP 30 + Vault Core 40 + Vault Sociedad 30 = 100%. Correcto.
- **Precio KASH** (recalculado fila por fila): Año1 $20.9M/3.3B=$0.0063 (63x) · Año2 $29.6M=$0.0090 (90x) · Año3 $33.7M=$0.0102 (102x) · Año5 $37.2M=$0.0113 (113x). **Todas exactas.**
- **Puntajes de Aura/Jaguar Score**: los 9 valores de calibración (10/5/15/8/5/20/12/3/5-10-20) y los 4 niveles (Cub 0-499 / Jaguar 500-1999 / Alpha 2000-4999 / Emperor 5000+) son **idénticos** en Protocolo §7D y BMC §12.
- **Throttle**: umbrales (1.2x/0.8x/0.5x EMA30) y % de quema (125/100/60/25) y cola diferida (0/0/40/75) consistentes entre §7 y §18.
- **Jaguar Exit Fee**: 5%/3%/1% con condición dual (precio<0.7xEMA30 AND venta>0.3% supply/hora), 100% al Vault. Consistente §9/§18/BMC.
- **Anti-Whale**: 1-2%→3%, 2-5%→6%, >5%→10%. Consistente §9/§18.
- **Fees Motor D 4 capas**: Capa 0 (0%), Capa 1 (1.5%), Capa 2 (3%/3.5%), Capa 3A (1.5%), Capa 3B (2%). Consistente §1.1/§6.4/§18/BMC §5.5. Diferencial 0.5% $LUKA vs SOL/USDC verificado.
- **Jerarquía de fees por etapa**: 4%/2.5%→2.5%/1.5%→0.5% consistente (BMC separa 2A/2B, ambos a 2.5%/1.5%, coherente).
- **Gating cNFT Jaguar Universal**: "nivel Jaguar (≥500 pts)" idéntico en §7C.1, §7D.2 y BMC §12.
- **Monte Carlo**: $37.2M mediana, 0.0% ruina, ~1.4% espiral transición / <0.5% B2 consolidado, 86.1% P(B2<Año2), 53.6%→82.3% P(B2 Año1) sin/con 4 Frentes. Consistente en headers, §10, §17 y §21.
- **Inversión y CAC**: $500K/$2M/$5M y CAC $5-9/$3-5/$1 consistentes protocolo §15/§15.1 y BMC §9.2/§2.2.
- **Flujos Motor A/B/C**: cada distribución atómica suma 100% del fee. Correcto (salvo el tema de oráculos, H2).

## Recomendación

Ninguno de estos hallazgos invalida el protocolo — el diseño es robusto. **H1/H2 sí deben cerrarse
antes de escribir el smart contract del Vault** (la composición debe sumar 100% exacto en el código).
El resto son mejoras de claridad para la versión "v4.3 limpia" que servirá de base a la implementación.

Propongo: (1) decides H1 (recomiendo Opción C), (2) yo genero un **Protocolo v4.3 corregido** con
todas las tablas a 100%, nomenclatura unificada (Aura, P_KASH) y la tabla canónica Fase/Etapa, y
(3) con esa base cerrada arrancamos la spec de smart contracts del primer milestone.
