# Isotipo LUKASH V1 (Line Art / Marca)

> Versión canónica. Se usa en: logotipo, pecho de personajes, sellos, favicon, marca de agua.
> Archivo definitivo: `brand/logos/lukash protocol isotipo.png` — **USAR ESTE PNG DIRECTAMENTE.**
> Fecha: 2026-09-02

## USO CORRECTO

**NO regenerar con prompt de texto.** El isotipo ya existe y es definitivo.
Este documento sirve para:
1. **Descripción textual de referencia** cuando subas el PNG a una IA junto con otro prompt
   (ej: "el personaje lleva en el pecho este símbolo [imagen adjunta]")
2. **Briefing para diseñadores humanos** que necesiten reproducirlo en vector/SVG
3. **Documentación interna** de los elementos y colores exactos

### Cómo usar en cada IA generativa:
- **Midjourney**: sube `lukash protocol isotipo.png` como imagen + tu prompt → usa `--cref` o `--sref`
- **ChatGPT/DALL-E**: sube la imagen + di "incluye este símbolo exacto"
- **Flux/Leonardo**: usa como ControlNet reference o image-to-image
- **Diseñador humano**: entrega este documento + el PNG como referencia

---

## PROMPT PRINCIPAL (copiar tal cual)

```
A perfectly symmetrical sacred emblem viewed from the front, centered on a pure black background (#0A0A0C). The emblem depicts a stylized big cat head (jaguar/panther hybrid) rendered in clean, continuous golden line art (#C9A84C) with consistent stroke weight throughout.

THE FELINE HEAD: Frontal view, bilateral symmetry. Two pointed triangular ears with inner detail lines. The forehead has a subtle inverted triangle formed by the brow lines. Almond-shaped eyes angled slightly upward, hollow (no pupils, just the outline shapes). A defined broad nose bridge narrowing to a small triangular nose tip. The muzzle widens below the nose into two curved cheek/jaw lines.

THE INFINITY SYMBOL: Flowing directly from the lower jawline of the feline, an infinity symbol (∞) forms the bottom half of the emblem. The two loops of the infinity are smooth, rounded, and slightly thick — they connect seamlessly to the jaw contour as if the feline's chin morphs into the infinity. The crossover point of the infinity sits at the exact vertical center of the overall emblem.

THE VAULT CIRCLE: At the exact crossover point of the infinity symbol, a perfect circle (the vault). Inside it, a smaller concentric circle creating a ring shape — like a vault door or portal viewed from the front. This circle interrupts the infinity crossover naturally, as if the infinity loops emerge from the vault.

THE GREEN AURA: Behind the entire golden emblem, a soft ethereal glow in bright green (#00FF88 to #3DAA6B gradient). This glow forms a subtle diamond/rhombus shape radiating outward from the emblem center. The glow is NOT a solid shape but a soft radial luminescence with higher opacity near the emblem and fading to transparent at the edges. Small bright sparkle points (4-pointed stars) appear at the cardinal points of the green aura (top, bottom, left, right).

BACKGROUND: Pure dark background (#0A0A0C). Very subtle, thin circuit board trace lines in slightly lighter dark gray (#1A1A1E to #2C2C30) extend horizontally from the left and right sides of the composition — straight lines with 90-degree turns, resembling PCB traces. These traces are barely visible, adding technological depth without competing with the emblem.

STYLE: Clean vector-like line art, flat 2D, no gradients on the golden lines themselves. The gold is solid and uniform. The ONLY depth comes from the green glow behind. Minimalist but sacred/mystical feeling. The overall composition resembles a coat of arms or sacred seal. No texture on the lines, no 3D shading, no metallic reflection.

RENDERING: Digital illustration, ultra-clean edges, perfect symmetry, centered composition. Square format (1:1 aspect ratio). The emblem occupies approximately 70% of the canvas height.

DO NOT INCLUDE: Any text, watermarks, signatures. No additional decorative elements beyond what is described. No gradients on the gold lines. No 3D effects on the emblem itself. No realistic fur or animal features — this is a stylized symbol, not a realistic animal.
```

---

## DESGLOSE DE ELEMENTOS (para verificación)

| # | Elemento | Verificar que... |
|---|---|---|
| 1 | Cabeza felina | Es FRONTAL, simétrica, line art dorado, orejas triangulares puntiagudas |
| 2 | Ojos | Almendrados, huecos (solo contorno), inclinados ligeramente hacia arriba |
| 3 | Nariz | Triangular pequeña, puente ancho que se estrecha |
| 4 | Símbolo infinito (∞) | Fluye del mentón, loops suaves y redondeados, conecta orgánicamente con la mandíbula |
| 5 | Círculo central (bóveda) | Perfecto, concéntrico doble, en el punto de cruce del infinito |
| 6 | Color oro | #C9A84C uniforme, sin gradientes, sin brillo metálico |
| 7 | Aura verde | Detrás del emblema, glow suave #00FF88, forma diamante sutil, sparkles en 4 puntos |
| 8 | Fondo | Negro puro #0A0A0C, trazas de circuito muy sutiles en gris oscuro |
| 9 | Estilo | Line art 2D plano, vectorial, limpio, sin texturas |

## PALETA EXACTA

- Líneas del emblema: `#C9A84C` (Oro LUKASH)
- Glow principal: `#00FF88` → `#3DAA6B` (gradiente radial)
- Sparkles: `#E8C96A` (Oro claro)
- Fondo: `#0A0A0C` (Negro Obsidiana)
- Trazas circuito: `#1A1A1E` → `#2C2C30` (apenas visibles)

## VARIANTES SUGERIDAS

- **Monocromático oro**: sin aura verde, solo oro sobre negro (para fondos oscuros simples)
- **Invertido**: oro sobre blanco (para documentos impresos)
- **Favicon**: solo la cabeza + infinito, sin aura, sin circuitos, fondo transparente
- **Sello pequeño**: isotipo en miniatura dentro de un círculo con borde oro 1px
