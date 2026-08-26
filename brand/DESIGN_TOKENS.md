# LUKASH — Design Tokens & Sistema de UI

> Fuentes: `estilo visual lukash.md` (v1.0) + PRD LUKASH (v3.1, §1.3) + UI Kit v1.0 (§08).
> Estética: "Apple Finance" × Solana. Jungla digital mística, dark mode permanente, glassmorphism,
> partículas flotantes sutiles, efectos bioluminiscentes. Isotipo: felino-bóveda-infinito ciber-bioluminiscente.

## Paleta completa

| Token | Hex | Uso |
| --- | --- | --- |
| `--oro` | `#C9A84C` | **Oro LUKASH.** Acento primario, botones, bordes 1px, sellos. |
| `--oro-light` | `#E8C96A` | Oro claro (highlights, glow). |
| `--oro-dark` | `#9A7030` | Oro oscuro (sombras, estados pressed). |
| `--ambar` | `#E8940A` | **Ámbar Fuego.** CTA, bordes activos, Motor A / El Fuego. |
| `--ambar-light` | `#F4B340` | Ámbar claro. |
| `--verde` | `#3DAA6B` | **Verde Selva.** Alertas positivas, USDC/RWA, La Bóveda, estado B2. |
| `--verde-light` | `#5FCA8E` | Verde claro. |
| `--verde-dark` | `#1E6640` | Verde oscuro (fondo burbujas de usuario en chat). |
| `--obsidiana` | `#0A0A0C` | **Negro Obsidiana.** Fondo principal de la app. |
| `--carbon` | `#0F0F12` | Carbón. Éxito/datos confirmados, fondo de cards secundarias. |
| `--carbon-light` | `#1A1A1E` | Carbón claro (panel derecho, cards). |
| `--gris` | `#2C2C30` | Gris (bordes sutiles, deshabilitado). |
| `--gris-light` | `#555560` | Gris claro (texto secundario). |
| `--gris-ll` | `#888898` | Gris muy claro (placeholders, timestamps). |
| `--blanco` (Marfil Sagrado) | `#F0EAD6` | Texto principal sobre fondos oscuros. |
| `--blanco-dark` | `#C8C0A8` | Marfil apagado (texto terciario). |

Fondo: degradado `#0A0A0C → #1A1A1A`, low-key. Marco de diapositiva/card: 1px Oro (`rgba(oro, 0.22)`).

## Tipografía

| Uso | Fuente (fallback) | Notas |
| --- | --- | --- |
| Títulos / headings | **Cinzel Bold** (serif) | Majestuosidad. |
| Datos / labels técnicos | **Space Mono** (monospace) | Precisión blockchain. Labels 7-9px UPPERCASE, tracking .15em. |
| Cuerpo | **IBM Plex Sans** (sans-serif) | Legibilidad fintech. |

## Espaciado y radios (UI Kit §08)
- Spacing: `4 · 8 · 12 · 16 · 32 · 48` px.
- Border radius: `r4 · r8 · r12 · r16 · r24 · pill`. Cards principales 24px.

## Efecto bioluminiscente (para cNFT / Tótems)
```css
@keyframes bioGlow {
  0%, 100% { box-shadow: 0 0 20px rgba(var(--color), 0.6); }
  50%      { box-shadow: 0 0 45px rgba(var(--color), 1.0); }
}
```
Color según activo: naranja/ámbar = BTC, verde = SOL, azul = USDC. Duración 2–3s, infinite.

## Reglas de marca / UI (del PRD — importantes)
- **Dark mode permanente.** Nunca light mode.
- En la UI el Vault se llama **"La Reserva Sagrada"** (o "La Reserva"). **NUNCA "Vault"** de cara al usuario.
- La Reserva Sagrada es **colectiva del ecosistema**, nunca "tu reserva" personal.
- LUKAI habla como **shaman digital**: sabio, cercano, sin jerga blockchain (no dice "on-chain", "smart contract", "DEX", "tokenomics"; dice "la bóveda del ecosistema", "el sistema de puntos", "activos reales").
- Idioma: español latinoamericano. Tono: cercano, soberano.
- Sello recurrente (esquina inferior derecha): `MONTE CARLO VALIDATED — 0.0% RISK` en Cinzel Bold + isotipo en miniatura.

## Naming de producto (referencia — reconciliar con v4.2)
- **Tótems cNFT** (nombres del PRD v3.1, gran material de marca): Fuego Eterno (cBTC), Espíritu (JitoSOL), Agua Viva (mSOL), USDC Sagrado (USDC/Kamino), Tótem Negro (mixto premium). Tiers Bronce/Plata/Oro/Tótem Negro.
  - ⚠️ En v4.2 los cNFT se abstraen como Tótem Nativo / Tótem Universal / Tótem Estándar. Decisión pendiente: mapear los Tótems a los 3 tipos v4.2.
- **App = 6 pestañas**: Wallet (La Reserva Sagrada) · Manadas · Jungle Arena · Tótems cNFT · LUKAI · La Reserva.

## Assets disponibles
- `logos/` · `personajes/` · `conceptos-ui/` · `estilo-visual/` (guías + UI Kit PDFs) · `audio/` (voz LUKAI)
