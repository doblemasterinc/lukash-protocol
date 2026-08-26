# LUKASH — Product Overview

> Resumen de producto. La especificación técnica exhaustiva vive en `docs/protocolo/` (fuente de verdad).
> Este archivo es el mapa de entrada para cualquiera que llegue al proyecto.

## Qué es
Ecosistema financiero descentralizado sobre Solana. Token $LUKA deflacionario respaldado por
el Vault KASH Core (reserva creciente de activos duros, auditada on-chain). Tesis: dar a 260M
de latinoamericanos excluidos acceso a la infraestructura financiera on-chain (RWA/blockchain)
que las instituciones grandes ya usan. "Las finanzas del futuro, disponibles hoy."

## Cómo captura valor (los 4 Motores, regla 35/35/15/15)
- **Motor A** (SOL): trading especulativo en DEX → llena el Vault. Desde TGE.
- **Motor B** ($LUKA): uso cotidiano en la App → quema (B0) o recircula (B2). Desde Etapa 2A.
- **Motor C** (USDC): pagos masivos con fee 0.5% → presión de compra. Fase 3.
- **Motor D** (multi, 4 capas): Manadas, gaming, cNFTs, DeFi. El alma cultural.

## Piezas clave
- **Vault KASH Core** — reserva 100% Solana-nativa. K_min=$25M activa Motor B2.
- **LUKAI** — orquestador de estados on-chain (v1.0) + IA conversacional (v2.0).
- **Aura** — reputación financiera on-chain, niveles Cub→Emperor.
- **Manadas** — natilleras/tandas del siglo XXI: ahorro comunitario con smart contracts.
- **Jungle Arena** — gaming financiero. Misiones de Caza → ganar Aura. *(Diseño pendiente.)*
- **cNFT** — instrumentos financieros tokenizados (3 tipos).
- **KASH Shield** — capa de seguridad (circuit breakers, exit fee, anti-whale, timelock, multisig).

## Segmentos
- **A (12-25)**: nativos de la atención. Entran por Jungle Arena. Gamificación + estatus.
- **B (25-45)**: buscadores de soberanía. Entran por LUKAI + respaldo real. Protección de inflación.

## Roadmap de construcción (Studio)
1. **[ACTUAL]** Setup + auditoría del protocolo → cerrar discrepancias (v4.3 limpia).
2. Diseño del juego / Misiones de Caza (Aura) + parámetros de Manadas.
3. **Primer milestone técnico**: smart contracts core en devnet (Motores A/B/D + Vault + Throttle).
4. LUKAI v1.0 orquestador + KASH Shield.
5. Prototipo App + Jungle Arena.
6. Auditoría Halborn/OtterSec → TGE.

## Estado
Discovery y diseño económico muy maduros (validados con Monte Carlo, 0.0% ruina). Lo abierto:
diseño de juego, parámetros de Manadas, go-to-market local, y la corrección de discrepancias
menores del protocolo (ver `audits/AUDITORIA_PROTOCOLO_v4.2.md`).
