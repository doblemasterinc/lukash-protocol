# Síntesis de Discovery — LUKASH

> Consolidado del barrido de TODO el corpus (estrategia, app, simulaciones, auditorías, narrativa).
> Objetivo: material listo para diseñar **el juego (Jungle Arena)**, el **sistema Aura** y la **App**.
> La lógica económica sigue el Protocolo v4.2 (fuente de verdad). Esto complementa, no reemplaza.

---

## 1. Universo narrativo y voz de marca

**Mundo:** una selva digital mística donde el dinero es "fuerza vital" que se escapa "entre las garras
como niebla". El usuario entra como **cachorro a merced del viento** y madura hasta **Jaguar Sabio /
dueño de su propio banco** — arco de soberanía financiera. La **Reserva Sagrada** (bóveda BTC/SOL/USDC)
es el santuario. Metáfora rectora: **cazar solo vs. cazar en Manada**.

**Giro narrativo oficial:** de "anti-banco" a **"La Reserva del Pueblo"** — construcción colectiva, no
confrontación (`NARRATIVA_REVISADA_PEOPLE_RESERVE.md`). Arquetipo de marca: **"El Jaguar Sabio"** —
educativo, confiado pero humilde, pro-pueblo, nunca "get rich quick".

**Personajes:**
- **LUKAI** — IA guía. Jaguar de obsidiana, ojos ámbar. Habla como **chamán digital**: pausado, sin jerga cripto. Firma: *"Respira profundo, Joven Jaguar."*
- **Los Jaguares** — usuarios. Progresión visible ligada a Aura.
- **Las Manadas** — grupos que ahorran/invierten/compiten juntos. *"El tiempo de cazar solo ha terminado."*
- **EROSAI / $EROS** — ⚠️ NO es personaje de LUKASH. Es un proyecto precursor del fundador (agente poeta) del que LUKASH heredó el mecanismo de token con reserva multidivisa. ADN de origen, no lore de usuario. **Confirmar con Sebastián si quiere darle algún rol.**

**Frases-firma reutilizables:** *"Quien siembra con paciencia, cosecha libertad."* · *"En la manada pagamos con Lukas, pero crecemos con la Soberanía."* · *"¿Ruges con nosotros?"* · *"La manada avanza junta. 🐆"* · Taglines: "La Reserva del Pueblo" · "De la manada, para la manada, por la manada."

---

## 2. Sistema Aura (rebrand de Jaguar Score) — insumos de diseño

**Concepto narrativo:** Aura = **"el resplandor del Jaguar"** — reputación visible que se gana cazando
bien y cuidando a la Manada, no comprando. On-chain, no transferible (mantiene la mecánica v4.2).

**Fórmula compuesta propuesta** (combina lo que ya definía la doc de la App): 
`Aura = volumen transaccional + tiempo de hold + uso de LUKAI + victorias en Arena + puntualidad en Vacas + misiones educativas completadas`. Se mapea a los umbrales v4.2.

**Niveles — 3 esquemas de naming en la mesa** (misma estructura v4.2, decisión de marca D2):
| Umbral (v4.2) | v4.2 original | Opción hispana (4) | Opción arco narrativo (5)* |
| --- | --- | --- | --- |
| 0-499 | Cub | Cachorro | Cachorro |
| 500-1999 | Jaguar | Jaguar | Rastreador |
| 2000-4999 | Alpha | Alfa | Cazador |
| 5000+ | Emperor | Emperador | Jaguar Maduro |
| (5º nivel) | — | — | Jaguar Sabio |
\* la opción de 5 niveles obliga a recalibrar umbrales y gates del cNFT.

**Regla de oro del sistema de recompensas (inviolable):** el Aura y las recompensas se pagan **siempre
en $LUKA desde los pools de Marketing/Staking, NUNCA del Vault KASH**. El Vault solo crece.

---

## 3. Jungle Arena — catálogo de mecánicas de juego

De la doc de la App (FASE 2/3, plan maestro) + estrategia. **Insumos, no decisiones cerradas:**

**Tres arquetipos de misión (para ganar Aura):**
- **(a) Sociales/virales — "Proof of Roar":** retos verificados por LUKAI (hilos en X/TikTok, comentarios, "Golden QR" / stickers físicos con foto). Mecánicas concretas: *Spot the Whale, Meme Wars, The First Hunt, raids coordinados lun/mié/vie*.
- **(b) Transaccionales:** volumen, tiempo de hold ("Vigilia del Santuario" = proteger la Reserva), rachas de ahorro, uso de LUKAI.
- **(c) Educativas (Learn-to-Earn):** quizzes y simuladores ("gestiona tu Vault Personal"). Máxima ponderación de Aura (+20 en v4.2). Enfocado Gen Z/Alpha.

**Mecánicas de gaming avanzadas (de la doc de App):**
- **cNFT evolutivo ("Estatus de Depredador"):** minteado al registro (Bubblegum), evoluciona con el `User_Activity_Score` → skins, multiplicadores de staking, descuentos en fee de IA. Es la **representación visual del nivel de Aura**.
- **Sistema de Energía (sink de tokens):** crédito interno no-transferible, se compra con $LUKA (split 30/30/40); cada acción consume Energía.
- **Stake de Batalla:** para atacar/defender territorios la Manada bloquea $LUKA → genera fee.
- **Mapa Global de Caza + Geo-Drops:** LUKAI lanza "Presas" (recompensas $LUKA/NFT desde pools Marketing/Staking) en coordenadas.
- **Conquista de Territorios:** Manadas reclaman zonas y cobran % de fees de la transaccionalidad ahí.
- **Duelos y torneos** entre Jaguares y entre Manadas; LUKAI narra como chamán-comentarista; leaderboard que **celebra contribución, no solo números**.

**Widgets de contenido/urgencia:** "Burn Stream" 24/7 (supply baja / Vault sube), "Vault Challenge", "Inverse Calculator". Alinean con el dashboard v4.2 (La Bóveda / El Fuego / La Cola).

---

## 4. Manadas — parámetros de producto (insumos)

- **Membresía vía cNFT** (identidad + score + llave de acceso). **Fuerza de Manada** = suma del poder de los cNFTs + $LUKA holdeado por miembros.
- **Roles/gobernanza:** creación/join de squad; "Embajadores de Jungla" (influencers no-financieros que crean su Manada y compiten por el Vault de Manada más grande).
- **Servicios (Motor D Capa 2 en v4.2):** Vacas (crowdfunding social, LUKAI redacta el contrato P2P, **10% de la operación regresa al Vault** como fee de protección), Crowdlending, Marketplace interno, Fondos/Seguros colectivos, Eventos.
- **Niveles de Manada (doc vieja):** Cachorro (2-5) → Jauría (6-20) → Legado (21+). Mínimo 2 miembros para activarse.
- **Beneficio de reputación:** Aura alto de miembros → menos colateral en préstamos, mejores rendimientos.

---

## 5. Arquitectura de la App (reconciliada con v4.2)

**6 pestañas canónicas (PRD v4.2/UI):** Wallet ("La Reserva Sagrada") · Manadas · Jungle Arena · Tótems cNFT · LUKAI · La Reserva.

**Módulos clave (de la doc de App — a mapear sobre las 6 pestañas):**
- **Wallet multi-activo (Gateway):** $LUKA, SOL, USDC, cBTC. Todo movimiento paga el fee de etapa con swap atómico a $LUKA vía Jupiter. **$LUKA es la única unidad de cuenta** (prohibido mostrar precios en FIAT/otras cripto en UI).
- **Jaguar Chat:** mensajería P2P cifrada con comandos en burbuja (`/enviar`, `/cobrar`, `/vaca`) que ejecutan smart contracts.
- **Las Vacas / Oracle Bets / Marketplace:** crowdfunding social, apuestas P2P con escrow (rake 2-5%), marketplace con escrow-árbitro.
- **LUKAI ejecutor:** LUK Pro (adultos/PYMES: contador ISO 20022, pagos programados) y LUK Fun (jóvenes: retos de ahorro).
- **Módulo Universal Fee Extractor:** motor on-chain que intercepta toda transferencia SPL, calcula el fee según etapa, swapea a $LUKA y distribuye (Burn/Vault/LP/Staking). Batching hasta ~20 SOL equiv. **Es el corazón técnico del primer milestone.**
- **KYC por tiers** (redes → on-chain identity → scoring crediticio) y **backend ISO 20022** (XML en el `memo`).

---

## 6. Go-to-Market y crecimiento (insumos de estrategia)

- **North Star:** CAC objetivo ~$3 (vs $5-15 neobanco, $150-350 TradFi). App móvil = canal más barato.
- **Dolor LATAM:** el problema no es "abrir cuentas" (Colombia 95.8% tiene depósito) sino la **utilidad del capital** — solo 35.5% accede a crédito ("Impuesto Invisible de la Exclusión").
- **Gen Z desintermediada:** 76% busca consejo financiero en TikTok/IG; #FinTok >1.4B views. LUKAI reemplaza al finfluencer azaroso.
- **Métrica moderna:** medir **CPW (Coste de Adquisición de Billetera)** y LTV on-chain, no impresiones.
- **Infra de growth medible:** Galxe/Zealy/Layer3 (quests anti-bot), Formo/Spindl (atribución CPW), Nansen/MetaCRM (ballenas), Farcaster Frames + Lens (DeSoc), agregador Firefly.
- **Pirámide de influencers 1+20+100+1000** remunerados con tokens (vesting), no pagos únicos.
- **Referidos con leaderboard:** 5→50k, 20→250k+NFT, 100→llamada con founders.
- **Hub regulatorio:** El Salvador (licencia DASP, 0% capital gains) como lanzamiento; alinear con MiCA (reserva 1:1) y sandbox SFC Colombia / Bre-B.

---

## 7. Restricciones de diseño (reglas de oro)
1. Recompensas SIEMPRE en $LUKA desde Marketing/Staking — **nunca del Vault KASH**.
2. $LUKA es la única unidad de cuenta en la UI.
3. "La Reserva Sagrada" en UI, nunca "Vault".
4. LUKAI nunca usa jerga blockchain.
5. La lógica de fees sigue v4.2 (Motor D 4 capas + 35/35/15/15), no el "Fee de Etapa único" de la doc vieja.
6. Aura no se compra; se gana con participación real (anti-spam, estilo Soulbound).
