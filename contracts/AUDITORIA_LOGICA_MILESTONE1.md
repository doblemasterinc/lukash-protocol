# Auditoría de lógica — Contrato Milestone 1 (post-compilación)

> 2026-08-19 · Compila ✅ ("Build successful" en Solana Playground). Esta auditoría revisa que la LÓGICA
> coincida con el Protocolo v4.3, no solo que compile. Alcance: núcleo económico (Etapas 1-2).

## Veredicto
**La lógica del núcleo es correcta. No hay bugs críticos.** Hace exactamente lo que debe para lo que este
milestone prueba. Hay 1 ajuste opcional que vale la pena y varios puntos que son de milestone 2 / etapa 3.

## ✅ Verificado correcto (coincide con el protocolo)
- **Fees por etapa/motor/capa:** Motor A 4%/2.5%WL (Et.1) → 2.5%/1.5%WL (Et.2) · Motor B 2.5%/1.5%WL (desde 2A) · Motor C 0.5% (Et.3) · Motor D capas 0%/1.5%/3%-3.5%/1.5%/2%. Todos exactos (§13).
- **Distribución atómica 35/35/15/15** con **invariante verificado** (la suma exacta debe igualar el fee; el resto va a staking para no perder centavos).
- **Vault:** split de fee **70% Core / 30% Sociedad** + composición **cBTC 35 / SOL 15 / LST 20 / USDC res 25 / lending 5 = 100%**. Validado que suma 100% en `initialize`.
- **B0/B2:** Motor A siempre quema; Motor B/D queman en **B0** (modulado por el Throttle) o **recirculan** en **B2**.
- **Throttle:** umbrales correctos (P>1.2× ACEL · 0.8-1.2× NORMAL · 0.5-0.8× CONS · <0.5× DEF) y % de quema (100/60/25) con el resto a la cola diferida.
- **Jaguar Lock** ($30M Core o 12 meses) y **switch a B2** (K ≥ $25M) — correctos.
- **Timelock 48h**, **pausa (circuit breaker)** y **control de acceso** (`has_one = authority`) — correctos.
- **Overflow checks** (`checked_*` / u128 intermedio) en toda la aritmética.

## 🟡 Ajustes (opcionales — no rompen nada)
1. **Cola de quema diferida sin drenar (el único que recomiendo hacer ahora).** El Throttle acumula quemas
   diferidas en modo CONSERVADOR/DEFENSIVO, pero **no hay instrucción que las ejecute** cuando el precio se
   normaliza (protocolo: ≤10%/semana). Falta `execute_deferred_burn`. Es un mecanismo real del protocolo;
   añadirlo completa el Throttle. **~15 líneas.**
2. **ACCEL 125% → simplificado a 100%.** En modo acelerado el protocolo quema 125% (extra de la cola); el
   scaffold lo capa a 100% (no se puede quemar más que el tramo en el modelo por-fee). Simplificación
   documentada; se completa en milestone 2.
3. **Motor C (Etapa 3):** su tramo LP es "inyección al LP", no quema; está contabilizado como quema. Menor y
   de Etapa 3 (fuera del alcance de este milestone).
4. **Convergencia de fees en Etapa 3 (0.5%):** el código mantiene Motor A/B en 2.5% en stage 3; el protocolo
   dice que converge a 0.5%. Definir/implementar en milestone 2.

## 🔵 Simplificaciones intencionales (por diseño, milestone 2)
- Montos como **notional USD** (se prueba la lógica aislada; sin transferencias reales SPL/Jupiter/quema on-chain).
- `process_fee` **sin gate de acceso** (en producción lo protege el movimiento real de tokens: no puedes "fingir" un fee sin mover tokens).
- **Yield del Vault / R_op (70/30)** no implementado (es un flujo aparte, off-chain/oráculo — no parte del reparto de fees).
- Staking (15%) y O&M (15%) incrementan contadores; el movimiento real de tokens es milestone 2.

## Recomendación
El núcleo está **correcto y listo** para lo que este milestone valida. **No es obligatorio corregir nada ahora.**
Si quieres dejarlo redondo, el único ajuste que vale la pena hoy es añadir **`execute_deferred_burn`** (#1).
Los demás son de milestone 2 / Etapa 3. Puedo darte el archivo corregido para re-pegar y re-buildear.
