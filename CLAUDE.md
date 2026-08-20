# CLAUDE.md — LUKASH Protocol

> Ecosistema financiero descentralizado sobre Solana. La puerta de entrada de LatAm
> a la infraestructura financiera on-chain. Tipo: Propio. Arquitecto: Sebastián Botero Pabón.
> Tesis: "Las finanzas del futuro, disponibles hoy para toda Latinoamérica."

## Propósito
$LUKA es un token deflacionario respaldado por una reserva creciente de activos duros
(el **Vault KASH Core**, 100% Solana-nativo) auditada on-chain. Cada transacción del
ecosistema alimenta el Vault, quema supply y recompensa a quien participa. No es un banco
alternativo: es acceso democratizado a la misma infraestructura (RWA/blockchain) que el
mundo institucional ya construye.

## Source of truth (leer ANTES de proponer cambios)
- `docs/protocolo/LUKASH_Protocolo_v4_2_FINAL.docx` — **arquitectura técnica DEFINITIVA**. Fuente de verdad.
- `docs/protocolo/LUKASH_BMC_v4_2_FINAL.docx` — modelo de negocio v4.2 (debe alinearse al protocolo).
- `extracted/Protocolo_v4.2_texto.txt` y `extracted/BMC_v4.2_FINAL_texto.txt` — versiones en texto plano para lectura rápida por agentes.
- `.claude/knowledge/key-decisions.md` — decisiones cerradas (ADRs). NO contradecir sin nuevo ADR.
- `.claude/knowledge/learned-rules.md` — reglas aprendidas.
- `audits/` — reportes de auditoría del protocolo (discrepancias detectadas).
- `brand/` — identidad visual. **El logo/isotipo es fundamental.** Ver `brand/DESIGN_TOKENS.md`.

## Arquitectura del protocolo (resumen operativo)
- **4 Motores** (regla universal 35/35/15/15 → Vault / Quema-LP / O&M / Staking):
  - **Motor A** (El Cazador, SOL, fee 4%/2.5%WL): trading en DEX. Activo desde TGE.
  - **Motor B** (El Motor Interno, $LUKA, 2.5%/1.5%WL): App. B0=quema directa (K<$25M), B2=recirculación (K≥$25M).
  - **Motor C** (El de Escala, USDC, 0.5%): pagos masivos. Fase 3.
  - **Motor D** (El Alma, multi, 4 capas 0%/1.5%/3-3.5%/1.5%/2%): Manadas, gaming, cNFTs, DeFi.
- **Vault KASH Core**: cBTC, SOL, LST (JitoSOL/mSOL), USDC reserva, USDC lending, oráculos. K_min=$25M activa Motor B2.
- **LUKAI**: orquestador de estados on-chain (v1.0, TGE) + interfaz IA conversacional (v2.0, Etapa 2A).
- **Aura** (antes "Jaguar Score"): reputación financiera on-chain, no transferible. Niveles Cub→Jaguar→Alpha→Emperor.
- **Jaguar Shield**: Circuit Breakers + Exit Fee + Anti-Whale + Timelock 48h + Tridente Multisig + oráculos redundantes.
- **Throttle dinámico dual**: Etapas 1-2 modula velocidad de quema; Etapa 3 (post-ENZ) invierte la lógica.
- **cNFT**: instrumentos financieros tokenizados (NO llaves de acceso). 3 tipos: Nativo / Jaguar Universal / Estándar.
- **Etapas**: 0 Pre-lanzamiento · 1 Génesis · 2A App/B0 · 2B Madurez/B2 · 3 Soberanía · 4 DAO.

## DECISIÓN DE MARCA CRÍTICA (sesión 2026-08-19)
- **"Aura" = rebrand de "Jaguar Score"** (ADR-002). En TODO código, UI y docs nuevos se usa **Aura**.
  La mecánica no cambia (niveles Cub→Emperor, on-chain, no transferible, decay 2%/sem tras 90d).
  Los documentos históricos v4.2 dicen "Jaguar Score" — al reescribir, migrar a Aura.
- Las "misiones para ganar Aura" = las **Misiones de Caza** de Jungle Arena (PENDIENTE de diseño detallado).

## Stack técnico (estándar DeFi Solana)
- **Smart contracts**: Rust + Anchor. **Primer milestone: Motores A/B/D + Vault + distribución 35/35/15/15 + Throttle en devnet.**
- **Frontend/SDK**: TypeScript.
- **Off-chain cache**: Supabase PostgreSQL.
- **Oráculos**: Pyth Network + Switchboard (redundancia, umbral desviación 2%).
- **Infra Solana**: Meteora (LP), Jupiter (swaps/routing), Jito (bundles privados anti-MEV), Sanctum (paridad LST), Kamino/Marginfi (lending).

## Reglas duras de implementación (seguridad DeFi — INNEGOCIABLES)
- SIEMPRE auditar access control antes de cualquier deploy.
- NUNCA deploy en mainnet sin testnet/devnet funcional 2+ semanas.
- SIEMPRE overflow checks en TODA operación aritmética (montos, fees, distribución).
- SIEMPRE timelocks (48h) en parámetros críticos: K_min, fees, thresholds Throttle, composición Vault.
- La distribución de fees debe ser ATÓMICA (una transacción) — sin riesgo de extracción parcial.
- Disclaimer on-chain obligatorio en metadata de cNFT: no garantiza rendimientos ni protección de capital.
- Vesting del equipo publicado on-chain ANTES del TGE. Bucket Equipo/Fundador 2% (200M), cliff 12m + lineal 48m (ADR-014).
- Auditoría Halborn/OtterSec pre-TGE es requisito NO negociable.

## Convenciones
- Conventional Commits. `.env.example` (nunca `.env`). ADRs. Español.
- `tasks/todo.md`: [ ] pending | [~] in progress | [x] done
- `tasks/lessons.md`: nunca borrar. Promoción a learned-rules si patrón 2+ veces.
- Progreso horizontal: esqueleto completo antes de profundizar.
- Registrar hitos en `BITACORA.md`.

## Guardrails de trabajo
1. **Verificar** el request. Preguntar si hay ambigüedad.
2. **Planificar**: pasos + "done cuando" (criterios verificables).
3. **Ejecutar** cambios mínimos.
4. **Verificar** contra los criterios. No auto-aprobarse.
