# Reconciliación v3.1 (PRD/UI/estrategia) vs v4.2 (protocolo canónico)

> Fecha: 2026-08-19. El material de producto/UI/marca más detallado (PRD, UI Kit, estrategia) está
> basado en el **Protocolo v3.1** (≈10-mar). La lógica económica canónica es el **Protocolo v4.2** (16-mar).
> Este documento lista dónde divergen y qué decisión de marca/producto hace falta para el v4.3 limpio y la App.
> Documento vivo: se completa con los briefs de los revisores de app y narrativa.

## Decisiones de marca/producto que necesito de Sebastián

| # | Tema | v3.1 (PRD/UI/estrategia) | v4.2 (protocolo) | Recomendación |
| --- | --- | --- | --- | --- |
| D1 | **Ticker del token** | "$Lukas" / "$LKS" (PRD, UI Kit, tagline "pagamos con $Lukas") | **"$LUKA"** (consistente en todo v4.2; estrategia y análisis ya usan $LUKA) | **$LUKA** como canónico. Actualizar assets de marca viejos. ¿Confirmas? |
| D2 | **Niveles de Aura** | 5 niveles con nombres en español, y con umbrales inconsistentes entre sí: PRD Pestaña 3 (Cachorro 0-100 / Rastreador 101-300 / Guerrero 301-600 / Líder 601-900 / Jaguar 901+) vs UI Kit (Cachorro 0-199 / Rastreador 200-499 / Guerrero 500-999 / Líder 1000-1999 / Jaguar 2000+). Estrategia mencionaba Alpha/Glow/Cub | **4 niveles originales**: Cub 0-499 / Glow 500-1999 / Alpha 2000-4999 / Emperor 5000+ | **RESUELTO (ADR-005 actualizado):** 7 niveles: Cachorro→Rastreador→Cazador→Alfa→Emperador→Shamán→Titán. Ver abajo |
| D3 | **cNFT: naming** | "Tótems": Fuego Eterno (cBTC), Espíritu (JitoSOL), Agua Viva (mSOL), USDC Sagrado (USDC), Jaguar Negro (premium). Tiers Bronce/Plata/Oro/Jaguar Negro | 3 tipos: Nativo / Jaguar Universal / Estándar | Mantener los **nombres Tótem** (gran marca) como capa de UI sobre los 3 tipos técnicos v4.2 |
| D4 | **Nombre del Vault en UI** | "La Reserva Sagrada" (regla: NUNCA decir "Vault" al usuario) | "Vault KASH Core" (técnico) | **Mantener** "La Reserva Sagrada" en UI, "Vault/KASH Core" en contratos/docs técnicos |
| D5 | **Tagline** | "Ahorra mientras gastas. Crece mientras juegas. Construye el banco que LATAM nunca tuvo." | "Las finanzas del futuro, disponibles hoy para toda Latinoamérica." | Usar la v4.2 como principal; la v3.1 ("Ahorra mientras gastas, crece mientras juegas") sirve como **subtexto de producto/App** (describe bien el loop) |

### D2 — Niveles de Aura (RESUELTO — ADR-005 actualizado 2026-08-26)
**Decisión cerrada: 7 niveles** con nombres en español (marca, no se traducen):
`Cachorro (0-499) → Rastreador (500-1,499) → Cazador (1,500-2,999) → Alfa (3,000-4,999) → Emperador (5,000-9,999) → Shamán (10,000-24,999) → Titán (25,000+)`.
Gate cNFT Tótem Universal (≥500) preservado. Umbrales recalibrados para 7 niveles.

## Divergencias técnicas (el protocolo v4.2 gana — solo para trazabilidad)

| Tema | v3.1 | v4.2 (canónico) |
| --- | --- | --- |
| Composición del Vault | cBTC 40 / SOL 15 / LST 20 / USDC reserva 20 / lending 5 = 100% (consistente). PRD Wallet muestra otra simplificación: cBTC 40 / JitoSOL 35 / USDC 25 | cBTC 35 / SOL 15 / LST 20 / USDC reserva 25 / lending 5 = **100%** (tras quitar oráculos, ADR corregido). El v4.2 original sumaba 105% |
| Motor D | Fee único 1.5% | 4 capas (0% / 1.5% / 3-3.5% / 1.5% / 2%) |
| LUKAI | v3.1 | Releases v1.0 (orquestador) / v2.0 (IA) |
| Reputación | "Jaguar Score" / "JS" | **Aura** (rebrand, ADR-002) |
| Inversión por fase | (mayor) | $500K / $2M / $5M |
| Activos mencionados | algunos docs citan LINK | Solo Solana-nativos (sin LINK, sin bridge) |

**Nota histórica útil:** la composición del Vault en v3.1 (cBTC 40) **sí sumaba 100%**. El error de 105% se introdujo en v4.2 al bajar cBTC 40→35, subir USDC reserva 20→25 y añadir oráculos 5%. Nuestra corrección (quitar oráculos, dejar USDC reserva en 25) preserva la intención de v4.2 (más buffer USDC, menos cBTC) y vuelve a 100%.

## Insumos de estrategia/GTM reutilizables (del revisor de estrategia)
- **North Star de adquisición:** CAC objetivo ~$3 (vs $5-15 neobanco, $150-350 TradFi). Apps móviles = canal más barato.
- **Dolor LATAM cuantificado:** Colombia 95.8% tiene depósito pero solo 35.5% accede a crédito; el problema no es "abrir cuentas" sino **utilidad del capital** ("Impuesto Invisible de la Exclusión").
- **Gen Z desintermediada:** 76% busca consejo financiero en TikTok/IG; #FinTok >1.4B views. LUKAI reemplaza al finfluencer azaroso.
- **Retención:** gamificación sube engagement 18-35%; visualizar progreso (Aura) → +23% ahorro; retener es 5x más rentable que adquirir.
- **Métrica moderna:** medir **CPW (Coste de Adquisición de Billetera)** y LTV on-chain, no impresiones.
- **Infra de growth medible:** Galxe/Zealy/Layer3 (quests con filtro anti-bot), Formo/Spindl (atribución CPW), Nansen/MetaCRM (segmentar ballenas), Farcaster Frames + Lens (DeSoc).
- **Aura por contribución real** (no spam): recompensar contenido educativo, reporte de bugs, onboarding de embajadores; medir con Soulbound/NFT dinámico que evoluciona. Encaja como motor de Aura.
- **Mecánicas virales concretas:** "Spot the Whale", "Meme Wars", "The First Hunt", "Burn Stream" 24/7, "Vault Challenge", referidos con leaderboard (5→50k, 20→250k+NFT, 100→founders call), raids "Proof of Roar" lun/mié/vie.
