# LUKASH — Suite de simulación de motores (contract-faithful)

Simulaciones que responden **¿el protocolo funciona como se planteó, motor por motor?**
conduciendo la **lógica real del smart contract** (`../contracts/playground/lib.rs`, v4.3)
bajo escenarios de mercado, estrés, sensibilidad, throttle y adopción.

## Por qué es distinta de las simulaciones previas

El informe cuantitativo v4.2 (`docs/analisis/`, `docs/protocolo/lukash_mc_v4.py`) validó el
**diseño económico** con un modelo en float. Esta suite valida dos cosas **adicionales**:

1. **Que el CÓDIGO implementa el diseño** — la aritmética entera exacta de `lib.rs`
   (distribución 35/35/15/15, split Core/Sociedad 70/30, Throttle 4 modos, cola diferida,
   switch B0→B2) se ejecuta tal cual on-chain, y se verifican los **invariantes duros**.
2. **Que las conclusiones estructurales del informe se sostienen con la lógica on-chain real**
   (0% espiral con campaña ≥moderada, volumen = driver #1, throttle 0.50/0.80 correcto).

## Arquitectura

| Módulo | Rol |
| --- | --- |
| `engine.py` | **Réplica fiel de `lib.rs`** — mismas constantes y aritmética entera USD 6-dec. `process_fee`, distribución, Throttle, cola, `switch_motor_b`, `execute_deferred_burn`, Jaguar Lock. NO añade nada que el contrato no haga. |
| `economic.py` | Capa que el contrato NO tiene (off-chain/oráculo): Markov + ciclos halving, precios BTC/SOL, precio $LUKA = **P_KASH × prima**, vesting, adopción. Portada de `lukash_mc_v4.py`. |
| `trajectory.py` | Una trayectoria de 5 años: conduce `engine.py` con `economic.py`. Detecta espiral (trampa B0) y ruina. |
| `scenarios.py` | 3 campañas de lanzamiento: CONSERVADOR / BASE / AGRESIVO. |
| `suite.py` | Las 4 simulaciones del informe + invariantes. |
| `market.py`, `run.py` | **(descartados)** primer intento con impacto AMM — el precio colapsaba al piso. Reemplazados por el modelo prima. |

## Uso

```bash
cd simulations
python suite.py inv        # invariantes del contrato (rápido)
python suite.py mc         # Monte Carlo 3 campañas
python suite.py sens       # sensibilidad OAT
python suite.py stress     # estrés (8 escenarios)
python suite.py throttle   # throttle + usuarios
python suite.py all        # todo (~30-40 min; escribe out/)
```

Salidas en `out/`: JSON por SIM + PNG + CSV.

## Provenance (ADR-008 / auditoría H10)

Las cifras **absolutas** del Vault pertenecen a la familia "modelo v4 optimista" (10–200× el
modelo conservador v3.1 de $37.2M). **Lo válido para decisión es lo estructural**: % espiral,
timing B2, orden de escenarios, sensibilidades relativas. No publicar cifras absolutas sin
etiquetar la fuente.

## Fidelidad: qué SÍ y qué NO está en el contrato Milestone-1

El motor fiel expuso qué partes del diseño **aún no viven en el contrato** (candidatos Milestone 2):

- **Cap de quema 1%/día** (spec §13): NO en `lib.rs`. La suite lo aplica en la capa económica.
- **Valuación por oráculo del Vault** para el switch B0→B2: el contrato compara `vault_core_usd`
  (fees depositados, sin apreciar) contra K_min; el diseño mide K por valor de mercado (Pyth).
- **Módulo contra-cíclico de LUKAI** (rebalanceo USDC según régimen): NO en el contrato →
  la composición fija hace la campaña débil más frágil (ver hallazgos).
- **Jaguar Shield activo** (Anti-Whale, Exit Fee): NO en Milestone 1 (ADR-012, va en Milestone 2).
- **Yield del Vault**: lo aplica el keeper off-chain, no el contrato.
