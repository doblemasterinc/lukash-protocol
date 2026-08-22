# Validación de los Motores — Simulación fiel al contrato (Milestone 1)

> Ing. Sebastián Botero Pabón · 2026-08-21 · LUKASH Protocol v4.3
> **Pregunta:** ¿el protocolo funciona como se planteó, motor por motor?
> **Método:** ejecutar la lógica REAL del smart contract (`contracts/playground/lib.rs`)
> bajo escenarios de mercado, estrés, sensibilidad, throttle y adopción, y verificar invariantes.
> **Código:** `simulations/` (engine.py = réplica fiel de lib.rs · economic.py · trajectory.py · suite.py).

---

## 0. Qué añade esta validación sobre el informe cuantitativo v4.2

El informe previo (`docs/analisis/`, `docs/protocolo/lukash_mc_v4.py`) validó el **diseño
económico** con un modelo en float. Esta suite valida dos cosas **adicionales y críticas para
el deploy en devnet**:

1. **Que el CÓDIGO implementa el diseño.** El motor `engine.py` es una réplica *línea por línea*
   de la aritmética entera de `lib.rs` (USD 6-decimales, división entera, dust a staking).
   Correr la simulación es equivalente a ejecutar el contrato. Sobre esa base se verifican los
   **invariantes duros** on-chain.
2. **Que las conclusiones del informe se sostienen con la lógica on-chain real** — no solo con
   el modelo float que hace atajos (fees continuos, sin conmutación B0/B2 exacta, sin cola entera).

**Provenance (ADR-008 / auditoría H10):** las cifras **absolutas** del Vault son de la familia
"modelo v4 optimista" (10–200× el modelo conservador v3.1 de $37.2M). **Lo válido para decisión
es lo estructural**: % de espiral, timing de B2, orden de escenarios, sensibilidades relativas.
Ninguna cifra absoluta debe publicarse sin etiquetar la fuente.

---

## 1. Veredicto

**El protocolo funciona como se planteó — con matices accionables para Milestone 2.**

- ✅ **La aritmética on-chain es exacta.** Todos los invariantes del contrato pasan (§2).
- ✅ **La tesis del lanzamiento agresivo se confirma cuantitativamente** (§3): campaña débil →
  44% de riesgo de espiral (trampa B0); campaña moderada o fuerte → **0%**.
- ✅ **Resiliencia estructural** ante shocks extremos (§5): crash BTC −80%, exploit 15% del Vault,
  retiro de LP → impactos de −0.1% a −3.8%. Espiral 0% en todos salvo el caso crónico.
- ✅ **El volumen es el driver #1 del Vault** (§4), muy por encima de fee, yield, throttle o K_min.
- ⚠️ **5 hallazgos a nivel de contrato** (§6): partes del diseño que aún NO viven en `lib.rs`
  (cap de quema, valuación por oráculo del switch, módulo contra-cíclico, modo ACELERADO inerte,
  Jaguar Shield) — candidatos de Milestone 2.

---

## 2. Invariantes del contrato (SIM 0) — TODOS OK

Verificados sobre trayectorias de 5 años en las 3 campañas:

| Invariante | Resultado |
| --- | --- |
| Distribución 35/35/15/15 suma exactamente el fee | ✅ error < 1e-10 (aritmética entera) |
| Composición del Vault = 100% (10000 bps) | ✅ |
| Conservación: LP+O&M+Staking = 65% de fees | ✅ error < 1e-9 |
| Simetría O&M == Staking (15/15) | ✅ exacta al micro-dólar |
| Supply nunca < 3.3B (piso ENZ) ni > 10B | ✅ |
| Precio de mercado ≥ P_KASH siempre (el muro) | ✅ |

**Lectura:** la distribución atómica es correcta y no pierde valor (el dust va a staking por
diseño). No hay extracción parcial posible. El piso P_KASH se respeta por construcción.

---

## 3. Monte Carlo por campaña (SIM 1) — la agresividad del lanzamiento es estructural

200 iteraciones × 3 campañas. **La agresividad de la campaña no es marketing: es lo que alimenta
el Vault y ejecuta la quema a tiempo para escapar de la trampa B0.**

| Campaña | Vault Y5 (mediana) | Espiral | Ruina | Switch B0→B2 (mediana) | P(B2 < Año 2) | Quema |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| **CONSERVADOR** (débil) | $90M | **44%** | 2% | día 1034 | 10% | 66% |
| **BASE** (moderada) | $389M | **0%** | 0% | día 448 | 98% | 66% |
| **AGRESIVO** (fuerte) | $1,882M | **0%** | 0% | día 195 | 100% | 66% |

**Alineación con el informe v4.2** (mediana ref: CONSERV $117M / BASE $483M / AGRESIVO $2.42B;
espiral ref 13.7% / 0% / 0%): mismo orden, mismo 0% en BASE/AGRESIVO, timing de B2 consistente.

**Diferencia informativa (no es error):** mi espiral CONSERVADOR (44%) es mayor que el 13.7% del
informe. La razón es un hallazgo de contrato (§6.4): uso la **composición FIJA del contrato**
(35% cBTC…), mientras el informe aplicaba el **módulo contra-cíclico de LUKAI** (rota a USDC en
bear). Ese módulo **no está en Milestone 1** → el contrato tal cual construido es *más frágil*
ante campañas débiles. **Refuerza la tesis:** con la implementación actual, lanzar débil es aún
más arriesgado de lo que el diseño idealizado sugería.

---

## 4. Qué palancas mueven el Vault (SIM 2 y SIM 4)

### 4.1 Sensibilidad OAT (SIM 2)
Un parámetro a la vez, 60 iters/punto, números comunes. Swing del Vault Y5 (mediana):

| # | Parámetro | Vault min | Vault max | **Swing** | Lectura |
| ---: | --- | ---: | ---: | ---: | --- |
| 1 | **Volumen (× base)** | $111M | $1,397M | **$1,287M** | Driver dominante. Cada punto de volumen pesa. |
| 2 | **Fee Motor A** (1.5–3.5%) | $215M | $493M | **$278M** | Palanca real, pero secundaria al volumen. |
| 3 | Yield APY (4–10%) | $351M | $358M | $7M | Marginal en 5 años. |
| 4 | Día launch App (30–180) | $353M | $355M | $2M | El protocolo es robusto a retrasos del launch. |
| 5 | K_min B2 ($10–35M) | $354M | $354M | **$0M** | **Neutro** — confirma que $25M no requiere ajuste fino. |

**Ranking: Volumen ≫ Fee A > Yield > Launch > K_min.** Coincide con el informe v4.2 (volumen = #1,
swing 3× el fee). **K_min ≈ $0 de swing es correcto, no un bug:** el timing B0→B2 solo decide si el
tramo LP del Motor B quema o recircula — **ninguno alimenta el Vault Core**, así que mover K_min no
cambia el Vault Y5. (El Asset Layer 35% es #2 en el informe; aquí no se varía porque es una
**constante inmutable** del contrato — no es una perilla de v1, por diseño.)

### 4.2 Throttle (SIM 4a) — no es palanca de crecimiento del Vault
6 configuraciones de umbrales (incluida la ACTUAL 0.50/0.80), números comunes:

> **Las 6 configuraciones dan exactamente $368M de Vault Y5.** El Throttle es **completamente
> irrelevante** para el Vault Core, porque solo gobierna el *timing* de quema del tramo LP del
> Motor B (quema ahora vs difiere) — y ese tramo **nunca toca el Vault**. Versión más fuerte del
> hallazgo del informe (<1%). **Recomendación: mantener 0.50/0.80 en v1** (no hay nada que optimizar
> del lado del Vault; su valor está en proteger el *precio*, no el Vault).

### 4.3 Adopción de usuarios (SIM 4b) — Motor D es amplificador, no motor
Barrido de usuarios al launch (1K→100K), números comunes:

| Usuarios | 1K | 5K | 20K | 50K | 100K |
| --- | ---: | ---: | ---: | ---: | ---: |
| Vault Y5 | $381M | $381M | $384M | $389M | $395M |

Señal limpia y monótona pero **pequeña** (+$14M de 1K a 100K). **El Motor A domina el Vault; el
Motor D (App) es un amplificador de escala, no la fuente principal del Vault en 5 años.** Coincide
con el informe (100K usuarios → +$26M). Implicación GTM: la App es crítica para *escala y comunidad*,
pero el Vault temprano depende del **volumen de trading del Motor A** (ver §3, §5).

---

## 5. Estrés extremo (SIM 3) — resiliencia confirmada

120 iteraciones/escenario, **números aleatorios comunes** (mismas sendas de mercado, distinto
shock) para deltas limpios.

| Escenario | Vault Y5 (mediana) | Δ vs BASE | Espiral | vs informe |
| --- | ---: | ---: | ---: | --- |
| BASE (sin estrés) | $384M | — | 0% | ref |
| Crash BTC −50% | $381M | −0.7% | 0% | ref −1.1% ✓ |
| Crash BTC −80% | $369M | −3.8% | 0% | ref −5.3% ✓ |
| Motor A a 0% (6 m) | $370M | −3.5% | 0% | ref −7.3% |
| **Motor A −70% permanente** | $121M | **−68.4%** | **5%** | ref −65.3% ✓ |
| Retiro LP 40% | $381M | −0.8% | 0% | ref −1.8% ✓ |
| Exploit 5% del Vault | $383M | −0.1% | 0% | ref −0.6% ✓ |
| Exploit 15% del Vault | $383M | −0.3% | 0% | ref −0.7% ✓ |

**Lecturas:**
- **El compuesto del Vault es el escudo.** Un exploit del 15% en el día 150 casi no mueve el
  Vault Y5 (−0.3%): a esa altura el capital es pequeño frente a 5 años de acumulación.
- **El único riesgo real es crónico, no agudo:** una caída *permanente* del 70% en el volumen
  del Motor A (−68%). No es una emergencia de contrato — es una señal de gobernanza/negocio
  (el protocolo depende de que el Motor A tenga volumen sostenido). Conecta con §3 y §4.
- Con la composición fija del contrato, ese caso crónico además introduce 5% de espiral (§6.4).

---

## 6. Hallazgos a nivel de contrato (Milestone 2)

Surgieron al construir el motor fiel. Ordenados por prioridad.

### 6.1 El modo ACELERADO del Throttle es inerte (prioridad: media)
`lib.rs:343` — `throttle_burn_bps(mode).min(BPS_DENOMINATOR)`. Como `BURN_ACCEL_BPS = 12_500`,
el `.min(10_000)` lo capa a 100%. **En ACELERADO la quema del tramo LP es idéntica a NORMAL**
(no puedes quemar más del tramo que existe en esa tx), y la cola diferida se drena al mismo
10%/semana en ambos modos. **La palanca de "quemar 125% en euforia" no produce ningún efecto**
en el contrato. Decisión para ti: (a) aceptar ACEL≡NORMAL (es seguro: no se puede sobre-quemar),
o (b) que ACELERADO también acelere el drenaje de la cola diferida (p.ej. 20%/semana).

### 6.2 El switch B0→B2 usa costo, no valor de mercado (prioridad: alta)
El contrato compara `vault_core_usd` (suma de fees depositados, **sin apreciar**) contra K_min.
El protocolo v4.3 §13 define K_min sobre el **valor de mercado** del Core (vía Pyth). La brecha
en la mediana es pequeña (switch a día 195 vs 200 en AGRESIVO), pero en un bull fuerte donde
cBTC/SOL se aprecian, el contrato conmutaría **tarde**. **Milestone 2: leer la valuación del
Vault por oráculo (Pyth/Switchboard) para la condición de switch**, no el costo acumulado.

### 6.3 No hay cap de quema diaria 1%/día (prioridad: alta)
El Blueprint v4.3 §13 exige "Cap burn diario 1% supply/día". `lib.rs` **no lo implementa**. Sin
él, a precios muy bajos (post-TGE) la quema en USD podría destruir demasiado supply de golpe. La
simulación aplica el cap en la capa económica; **el contrato debe incorporarlo** (con el exceso
diferido al día siguiente, como dice la spec).

### 6.4 Sin módulo contra-cíclico de LUKAI → composición fija más frágil (prioridad: media)
El contrato asigna el 35% Asset Layer siempre con la composición fija (cBTC 35 / SOL 15 / LST 20
/ USDC 25 / lending 5). El diseño contempla que **LUKAI rote a más USDC en BEAR** (módulo
contra-cíclico, v4.3 §5). Ese rebalanceo **no está en Milestone 1**. Efecto medido: la campaña
débil pasa de ~14% a 44% de espiral. **Milestone 2: rebalanceo dirigido por régimen** (solo
sobre entradas nuevas, sin tocar el Vault existente — como especifica el diseño).

### 6.5 Jaguar Shield (Anti-Whale, Exit Fee) aún no está en el contrato (prioridad: alta, ya planificada)
Correcto por ADR-012: el Jaguar Shield es Milestone 2. La simulación **no** pudo estresar
Anti-Whale ni Exit Fee porque no están implementados. Cuando se construyan, re-correr SIM 3 con
escenarios de ballena vendiendo y pánico coordinado.

### 6.6 (CORREGIDO 2026-08-21) Restauración del Jaguar Shield — ADR-015
En el análisis original de esta sesión, el hallazgo #6 decía "límite de retiro del Vault 5%/día".
**Ese concepto no existe en ninguna versión del Protocolo LUKASH** (v1.0 a v4.2). Arqueología
completa en `HISTORIAL_JAGUAR_SHIELD.md`. Fue un artefacto arrastrado del Informe Cuantitativo
que la primera versión de este reporte incorporó por error.

**Lo que sí queda por implementar del diseño real (ADR-015):**
- **Seguro Anti-Exploit** (producto financiero externo, NO lógica de contrato): cobertura hasta
  5% del Vault, máx 1 evento cada 12 meses, activo desde **Etapa 2B** (K ≥ $25M — condición
  on-chain). Del contrato solo necesitará una instrucción `receive_insurance_recovery` (entrada
  al Vault en caso de reembolso de la aseguradora, nunca salida). Auditor(a) a cotizar pre-TGE.
- **Tridente Multisig 3-de-3**: cablear el guard para las 3 operaciones críticas cuando existan
  (activar Capa 3 cBTC, cancelar Circuit Breaker LP, modificar K_min). Los 3 firmantes son
  decisión pendiente pre-TGE.
- **Circuit Breaker del Vault**: pausa de 24h ante variación negativa >10% en 1 hora.
- **Principio "el Vault no se toca"** hasta Etapa 4 (DAO), donde la comunidad redefine.

---

## 7. Recomendaciones consolidadas

| # | Acción | Origen | Milestone |
| --- | --- | --- | --- |
| R1 | Valuación del Vault por oráculo para el switch B0→B2 | §6.2 | 2 |
| R2 | Implementar cap de quema 1%/día con diferido | §6.3 | 2 |
| R3 | **Jaguar Shield seguridad del Vault** (ADR-015): Circuit Breaker 24h + Tridente 3-de-3 para Capa 3 + instrucción `receive_insurance_recovery` para el Seguro Anti-Exploit externo. **NO "5%/día"** (concepto inexistente en el diseño). | §6.6 corregido | 2 |
| R4 | Definir semántica de ACELERADO (inerte o drenaje acelerado) | §6.1 | 2 |
| R5 | Módulo contra-cíclico de LUKAI (rebalanceo por régimen) | §6.4 | 2 |
| R6 | Jaguar Shield: Anti-Whale + Exit Fee, luego re-estresar | §6.5, ADR-012 | 2 |
| R7 | **GTM:** priorizar campaña ≥ BASE y volumen sostenido del Motor A — es el driver estructural, no marketing | §3, §4, §5 | Etapa 0-1 |

---

## 8. Reproducibilidad

```bash
cd simulations
python suite.py all      # ~25 min; escribe out/ (JSON + PNG + CSV)
python suite.py inv      # solo invariantes (segundos)
```

Semillas fijas. Gráficas en `simulations/out/`. Detalle de arquitectura en `simulations/README.md`.

---
*Validación de motores — simulación fiel al contrato. Complementa (no reemplaza) el Informe de
Simulaciones Cuantitativas v4.2, que valida el diseño económico. Este documento valida que el
código Milestone-1 implementa ese diseño y dónde aún no lo hace.*
