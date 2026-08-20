# Auditoría Complementaria — LUKASH (barrido de todo el corpus)

> Fecha: 2026-08-19. Complementa `AUDITORIA_PROTOCOLO_v4.2.md` (consistencia interna) y
> `DIVERGENCIAS_v3.1_vs_v4.2.md` (reconciliación de versiones). Aquí van los hallazgos surgidos
> de revisar el resto de la carpeta: simulaciones, auditorías previas v3, y la doc de la App.

## 🔴 H10 (CRÍTICA) — Los números "Monte Carlo Validated" no provienen del modelo v4.2

Es el hallazgo más importante de todo el barrido y afecta directamente la **credibilidad ante inversores**.

Existen **tres simulaciones distintas**, no una:
1. `LUKASH_MonteCarlo_RiskAssessment_v3.1.docx` — **de aquí salen TODOS los números publicados** en el Protocolo/BMC v4.2: Vault P50 $37.2M (P25-P75 $18.9M-$67.1M), ruina 0.0%, espiral 1.4%, P(B2<Año2) 86.1%, KASH $0.0113 (113x).
2. `lukash_mc_v4.py` + `Informe montecarlo 3000 simulaciones v4.2.docx` — el modelo **más reciente y "riguroso"** (Markov 3 regímenes, ciclos de halving). Corrió **3.000 trayectorias reales** (aunque el código declara N=5000).
3. `LUKASH_Informe_simulaciones cuantitativas.pdf` — 4 modelos de sensibilidad/estrés (~35.000 trayectorias).

**El problema:** el modelo v4.2 (el más nuevo) **contradice al alza** los números publicados, y el propio informe lo admite:

| Métrica | Publicado (proviene de v3.1) | Código v4.2 BASE |
| --- | --- | --- |
| Vault Año 5 | $37.2M | **$483M** |
| Precio (múltiplo) | 113x | **23,887x** |
| P(B2 < Año 2) | 86.1% | **100%** |
| Espiral de muerte | 1.4% | **0%** |

Solo coinciden K_min ($25M), el horizonte (1.825 días) y la etiqueta "5.000 iteraciones" (que además no cuadra: se corrieron 3.000).

**Además:** el **Sharpe 2.34** y el **drawdown -8.2%** que aparecen en el PRD/UI **no son reproducibles** por `lukash_mc_v4.py` (la función `stats()` no los calcula). Origen no trazable — deben verificarse aparte o retirarse.

**Por qué los números v4.2 son irreales (limitaciones del modelo):**
- **Reflexividad circular**: precio→volumen→fees→Vault→P_KASH→precio, con momentum compuesto `(p/p_tge)^0.15` → explica los 23,887x.
- **Ruina 0% y espiral 0% son estructurales, no empíricos**: el Vault *nunca decrece* por construcción del código (los buckets guardan USD invertido y solo se multiplican por apreciación); la espiral exige precio <50% del pico por 60 días seguidos, casi imposible con esa apreciación.
- **Calibración optimista**: volumen pico $12M-$60M/día (WIF/JTO sostenido), hasta 3.5M usuarios, y solo el 10% de la presión vendedora "impacta".

**Recomendación (importante antes de usar en material de inversión):**
1. **Decidir qué modelo es el oficial.** Recomiendo quedarse con los números **conservadores v3.1** ($37.2M, 113x, 0.0%, 1.4%) porque son defendibles; los del v4.2 son insosteniblemente optimistas.
2. **Etiquetar la fuente**: el documento debe decir explícitamente "cifras del modelo conservador RiskAssessment v3.1" — no atribuirlas al modelo v4.2.
3. **Recalibrar `lukash_mc_v4.py`** para que el Vault pueda decrecer (drawdown real de activos) y la presión vendedora sea realista, y **re-correr** con N=5000 real. Solo entonces las cifras serán auditables.
4. **Retirar o recalcular Sharpe 2.34 / drawdown -8.2%** hasta poder reproducirlos.

## Riesgos ABIERTOS heredados de las auditorías previas v3 (no resueltos por v4.2)

De la Evaluación Institucional [A3], Stress-Test [A4] y Análisis Estratégico [A2]:

| Riesgo | Estado en v4.2 | Severidad |
| --- | --- | --- |
| **Auditorías de smart contracts NO hechas** (Ottersec/Halborn citadas como meta) + sin seguro anti-exploit activo | Abierto (bloqueante institucional) | 🔴 |
| **Oráculo único (Pyth)** para Throttle + switch B0→B2 + contra-cíclico, sin fallback formal a Switchboard/multisig | Abierto (el protocolo menciona redundancia Pyth+Switchboard pero no está implementada) | 🟠 |
| **Correlación ~90% del Vault** (BTC 35 + SOL 15 + LST 20 caen en bloque en crash sistémico) | Abierto (composición no cambió) | 🟠 |
| **Vesting seed obligatorio on-chain** + reducir circulación TGE de 45% a 25-30% | Abierto (45% sigue circulante desde TGE) | 🟠 |
| **Inconsistencia burn/staking en modo DEFENSIVO** [A4 §6.4]: con θ=0.25 el ratio se invierte (1 quemado : 1.71 redistribuidos), reduciendo la deflación justo cuando más se necesita | No confirmado si el rediseño de Motor D lo corrigió | 🟡 |
| **Dependencia de volumen especulativo** Motor A (70-80% del ingreso Fase 1); frágil si cae bajo $3M/día | Mitigado parcial (4 Frentes, MM) pero estructural | 🟡 |
| **Narrativa/posicionamiento** [A2]: dependencia del relato "memecoin", riesgo de parecer 3 productos | Mejorado en v4.2 (nueva tesis RWA) pero a vigilar | 🟡 |

**Nota:** las auditorías A1 (CFO) y A4 son marcadamente favorables; las críticas duras y accionables están en **A3 (Institucional)** y **A2 (Estratégico)**. v4.2 SÍ resolvió de fondo la crítica más citada (circularidad B2↔R_op, al sacar la R_op del flujo normal del Motor B) y adoptó la estructura de inversión en 3 tramos condicionados.

## Divergencia adicional en el modelo de fees de la App

La doc de la App (FASE 2/3) describe un **"Fee de Etapa" único** (5.5% / 3.5% / 1.5% según market cap, con split 2.5% KASH / 1% LP / 1% Staking / 1% O&M) — que **NO coincide** ni con el Motor D de 4 capas de v4.2 ni con la regla 35/35/15/15. Es doc pre-v4.2. Al construir la App, **la lógica de fees debe seguir v4.2**, no la de FASE 2/3.

## Prioridad de cierre antes de codificar
1. 🔴 H1 (Vault 105%→100%) — **ya decidido** (quitar oráculos).
2. 🔴 H10 (provenance Monte Carlo) — decidir modelo oficial + etiquetar. **No usar cifras v4.2-code en material de inversión.**
3. 🟠 Riesgos institucionales (auditoría de contratos, oráculo fallback, vesting seed) — son condiciones de TGE, no bloquean el desarrollo en devnet pero sí el lanzamiento.
