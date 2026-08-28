# Diseño: AURA + JUNGLE ARENA — Sistema de Reputación, Tótems, Manadas y Juego

> v2.0 · 2026-08-28 · Base: Protocolo v4.2 (mecánica) + ADR-022 a 026 (sesión 18).
> Objetivo del loop (los 4 que definió Sebastián): **educación financiera + retención diaria + adquisición viral + volumen económico.**
> Regla de oro: el Aura NO se compra; se gana con participación real. Las recompensas se pagan en $LUKA
> desde los pools de **Marketing/Staking**, NUNCA del Vault KASH (el Vault solo crece).

---

## 1. AURA — "El Rugido de la Manada"

Aura es la reputación financiera on-chain del usuario: su historial de caza en la selva. Mecánica
heredada de v4.2 (inalterada):
- **On-chain, no transferible, acumulativa.** Vinculada a la wallet.
- **Decae 2%/semana tras 90 días de inactividad.** Se detiene al retomar actividad.
- **El nivel usa el Aura máximo histórico** — los niveles alcanzados no retroceden por decaimiento.
- Calibrada en tiempo real por LUKAI según participación en el ecosistema (Motor D).

### 1.1 Los 7 niveles (arco narrativo cachorro → titán)

| Nivel | Aura | Narrativa | Beneficios desbloqueados |
| --- | --- | --- | --- |
| **Cachorro** | 0–499 | Acabas de llegar a la selva | Acceso básico a servicios Motor D Capa 2. Manadas estándar. **Tótem Nativo Bronce** (desbloqueado por misión educativa, 1.5%). Misiones del Sendero del Aprendiz. |
| **Rastreador** | 500–1,499 | Aprendiste a leer las huellas | **Desbloquea Tótem Universal → DeFi Capa 1** (gate ≥500, igual que v4.2). **Puede mintear Avatar como NFT completo** (milestone). Voto en parámetros Motor D (DAO). Acceso prioritario a nuevos cNFT. |
| **Cazador** | 1,500–2,999 | Cazas por ti mismo | LP LUKASH 90d con +12% rewards. Visibilidad en ranking de Manadas. Acceso anticipado a integraciones Capas 3A/3B. **Puede crear Sellos de Manada (líder).** Puede liderar Manada. |
| **Alfa** | 3,000–4,999 | Proteges a tu manada | Gobernanza avanzada. LP Fundador 365d con +20% rewards. Multiplicador de Aura ×1.10. |
| **Emperador** | 5,000–9,999 | Tu rugido se escucha lejos | Override Anti-Whale automático. Multiplicador de Aura ×1.15. Acceso anticipado a instrumentos premium. |
| **Shamán** | 10,000–24,999 | Guías a la manada con sabiduría | Gobernanza premium (peso extra). Acceso a instrumentos **RWA y T-bills tokenizados** (Fase 3). Multiplicador de Aura ×1.25. Skin cNFT legendario. |
| **Titán** | 25,000+ | Eres leyenda de la selva | Multiplicador de Aura ×1.30. Exención total Anti-Whale y Exit Fee. Puede fundar "Dinastías" (federación de Manadas). Máximo estatus del ecosistema. |

> **Cambio v2.0 (ADR-023):** Capa 0 (staking gratis, 0%) eliminada. Cachorro accede al Tótem Nativo
> con fee 1.5% a través de una misión educativa, no directamente. No existe staking sin Tótem.

### 1.2 Calibración base (cómo se gana Aura — heredado de v4.2 §7D.1)

| Acción | Aura | Nota |
| --- | ---: | --- |
| Holding activo cNFT Tótem Universal | 10 / sem | Máximo incentivo de holdeo premium |
| Holding activo cNFT Tótem Nativo | 5 / sem | Staking básico via Tótem |
| Participación en Manada activa (Capa 2) | 15 / evento | Por transacción completada en Manada |
| Apuesta Bet & Win completada (Capa 2) | 8 / evento | Independiente del resultado |
| Acceso a evento o marketplace (Capa 2) | 5 / evento | Por uso de servicios sociales |
| **Completar misión educativa (Jungle Arena)** | **20 / misión** | **Máxima ponderación — premia el aprendizaje** |
| **Completar lección de educación financiera** | **5 / lección** | **4 tracks en paralelo** |
| Duelo de Tótems (participación) | 12 / duelo | Cohesión grupal |
| Staking activo $LUKA (mín 7d via Tótem) | 3 / día | Requiere Tótem Nativo, no holding suelto |
| LP Comprometido 30 / 90 / 365d | 5 / 10 / 20 / sem | Escalonado por lock |
| Predicción Relámpago (streak) | 3 / día | Streak activa multiplicador |

Los multiplicadores de nivel (×1.10 Alfa, ×1.15 Emperador, ×1.25 Shamán, ×1.30 Titán) aplican sobre estas ganancias.
**PENDIENTE (ajustable por gobernanza, Timelock 48h en Etapa 2A):** valores de lanzamiento, se recalibran con datos reales.

---

## 2. TÓTEMS — Instrumentos Financieros como cNFTs (ADR-023, ADR-024)

Un Tótem es un cNFT (Metaplex comprimido, ~$0.001/mint) que **es** la posición de staking del usuario.
No hay staking sin Tótem. Al depositar $LUKA en un Tótem, ese depósito genera rendimientos del 15%
Staking de todos los motores (A/B/C/D) proporcionalmente al capital comprometido.

### 2.1 Categorías de Tótems

| Categoría | Fee | Activo | Gate de Aura | Disponible |
| --- | --- | --- | --- | --- |
| **Nativo** | 1.5% | $LUKA | Ninguno (misión educativa) | Etapa 1 (App launch) |
| **Universal** | 1.5% | $LUKA | ≥500 (Rastreador) | Etapa 2A |
| **Estándar** | 2% | SOL/USDC/etc | Por definir | Etapa 3 |

### 2.2 Tiers visuales por monto de depósito

Cada categoría tiene 3 tiers. Misma mecánica, mismo fee, mismo rendimiento proporcional.
La diferencia es estética (color/aspecto del cNFT) y de capital comprometido:

| Tier | Nativo ($LUKA) | Universal ($LUKA) | Estándar |
| --- | --- | --- | --- |
| **Bronce** | 10K | 50K | Por definir |
| **Plata** | 50K | 200K | Por definir |
| **Oro** | 100K | 500K | Por definir |

- Montos de referencia, ajustables pre-TGE.
- El usuario escoge su compromiso. Se puede depositar más después para subir de tier.
- El tier es aspiracional y coleccionable — un Tótem Oro se ve diferente de uno Bronce.

### 2.3 Mecánica del fee

Cada transacción dentro de un Tótem (depósito, retiro, transferencia) paga el fee de su Capa.
Ese fee se distribuye **35/35/15/15** (Vault + LP/Burn + O&M + Staking) por Motor D.
El 15% Staking alimenta los rendimientos de TODOS los holders de Tótems, proporcional al capital.

### 2.4 Tótem Nativo gated por misión educativa (ADR-024)

El primer Tótem Nativo NO se compra desde un menú. Se desbloquea al completar las primeras
misiones del Sendero del Aprendiz:
1. LUKAI explica qué es un Tótem, cómo funciona, qué genera.
2. Quiz interactivo para verificar comprensión.
3. Al final de la misión se ofrece la compra en 3 tiers (Bronce/Plata/Oro).
4. El usuario elige su nivel de compromiso y mintea su primer Tótem Nativo.

---

## 3. AVATAR — Identidad del Usuario (ADR-024)

El Avatar es la representación visual e identidad del usuario en la plataforma. Es **diferente**
de los Tótems (instrumentos financieros). Un usuario tiene UN Avatar y puede tener MUCHOS Tótems.

### 3.1 Ciclo de vida del Avatar

| Etapa | Tipo | Costo | Características |
| --- | --- | --- | --- |
| **Creación** | In-app (off-chain) | Gratis | Se crea al registrarse. Evoluciona visualmente con el Aura (Cachorro → Titán). |
| **Mint NFT** | NFT completo (Metaplex, no comprimido) | Fee en $LUKA (por definir) | Se desbloquea tras misión milestone (Aura ≥500 o misión específica). Identidad on-chain. |
| **Evolución** | NFT con metadata dinámica | Automático | El Aura se refleja en el NFT. Un Avatar de Shamán tiene metadata diferente al de Cachorro. |

- **Un Avatar por usuario, único, transferible** (después del mint).
- Representación visual en Duelos, Manadas, leaderboards y marketplace.
- Mercado secundario: un Avatar con historial Shamán o Titán tiene valor intrínseco (reputación incluida).

---

## 4. SELLOS DE MANADA — cNFT de Membresía (ADR-025)

El **Sello de Manada** es un cNFT creado por el líder de una Manada para representar y distribuir
membresía/acceso. El nombre visible lo define el creador; el sistema lo llama "Sello" internamente.

### 4.1 Requisitos

- **Crear un Sello:** Aura ≥ Cazador (1,500+).
- **Comprar/recibir un Sello:** sin requisito de Aura.

### 4.2 Tipos de Manada que el Sello habilita

| Tipo | Descripción | El Sello = |
| --- | --- | --- |
| **Vaca** | Ahorro grupal para una meta (fin de año, vacaciones) | Tu cuota |
| **Fondo** | Crowdlending entre miembros | Tu participación |
| **Negocio** | Crowdfunding para un proyecto | Tu stake en el proyecto |
| **Evento** | Ticket de acceso verificable on-chain, revendible | Tu entrada |
| **Club** | Membresía abierta con marketplace interno, descuentos, votación | Tu carnet |

### 4.3 Configuración del Sello por el líder

El líder configura: tipo, cupo máximo, aporte mínimo/máximo, duración, meta de capital, arte del Sello.
Todos los fees pasan por **Motor D Capa 2** (3% en $LUKA / 3.5% en SOL) → distribución 35/35/15/15.
LUKAI actúa como árbitro de disputas (P2P entre miembros — el protocolo NO presta ni capta).

### 4.4 Manadas desde el día 1

Las Manadas son funcionalidad core desde el lanzamiento de la App/Web. No requieren Etapa 2 ni
instrumentos externos. Motor B (en $LUKA) y Motor D comparten la misma lógica de distribución
35/35/15/15 con B0/B2; Manadas operan bajo Motor D Capa 2 con fees de 3%/$LUKA.

---

## 5. JUNGLE ARENA — El Mapa de Caza

Pantalla-juego (Pestaña 3 de la App). Un **mapa de senderos** estilo videojuego donde cada nodo es una
misión. Nodos completados en oro brillante, activo pulsando, bloqueados en gris. LUKAI actúa como
chamán-guía y comentarista. Economía **no inflacionaria**: valor entre participantes + quemas del Motor B,
sin emisión de recompensas fuera de los pools Marketing/Staking ya asignados.

### 5.1 Cuatro senderos = los 4 objetivos

| Sendero | Objetivo primario | Qué contiene | Motor |
| --- | --- | --- | --- |
| **Sendero del Aprendiz** | Educación plataforma | Misiones learn-to-earn: LUKASH, Tótems, Manadas, Motores. Desbloquea primer Tótem Nativo. | Motor D Capa 2 |
| **Escuela Financiera** | Educación general | 4 tracks paralelos: finanzas personales, ciudadano, cripto, LUKASH. Lecciones cortas tipo TikTok. | Motor D Capa 2 |
| **Rastro Diario** | Retención | Loop diario: login, racha ("Vigilia del Santuario"), caza del día | Motor D Capa 2 |
| **Rugido de la Manada** | Viral + Volumen | Raids sociales (Proof of Roar), referidos, Duelos, misiones transaccionales, retos virales | Motor A/B/D |

### 5.2 Catálogo de Misiones de Caza (valores de lanzamiento)

**Sendero del Aprendiz (educación plataforma) — Aura alto, one-time:**
| # | Misión | Pasos | Aura | Desbloquea |
| --- | --- | --- | ---: | --- |
| A1 | **Primeras Huellas** | Quiz de onboarding: ¿qué es LUKASH y la Reserva Sagrada? | +20 | Sendero Rastro Diario |
| A2 | **La Reserva Sagrada** | Aprende cómo la Reserva respalda a $LUKA + haz tu 1er aporte (mín 1 $LUKA) | +20 | — |
| A3 | **Tu Primer Tótem** | LUKAI explica qué es un Tótem, cómo funciona, qué genera. Quiz + ofrece compra de Tótem Nativo (Bronce/Plata/Oro) | +30 | **Tótem Nativo** (1.5% fee) |
| A4 | **El Rugido del Ahorro** | Simulador: interés compuesto y hold con tu Tótem | +20 | — |
| A5 | **Cazar sin Miedo** | Módulo de riesgo/volatilidad + entiende tu rendimiento de Tótem | +50 | — |
| A6 | **El Puente DeFi** | Aprende Capa 1 + mint tu 1er Tótem Universal (requiere Rastreador ≥500) | +50 | DeFi Capa 1 |
| A7 | **Forma tu Manada** | Crea o únete a tu primera Manada. LUKAI explica tipos y Sellos. | +30 | Funciones de Manada |

**Escuela Financiera (4 tracks en paralelo, ADR-026) — repetible:**
| Track | Contenido (fácil → experto) | Aura/lección |
| --- | --- | ---: |
| **Finanzas personales** | Ahorro, presupuesto, deuda, fondo de emergencia, inversión, planificación fiscal | +5 |
| **Finanzas del ciudadano** | Bancos, tasas, inflación, impuestos, política monetaria, macroeconomía | +5 |
| **Cripto y blockchain** | Bitcoin, wallets, exchanges, seguridad, DeFi, smart contracts, auditoría | +5 |
| **LUKASH** | Reserva Sagrada, Tótems, Manadas, Motores, Capas, DAO | +5 |

Formato: lecciones cortas (2-3 min), tipo TikTok (video + reto), generadas/personalizadas por LUKAI.
Streaks por días consecutivos. LUKAI detecta nivel del usuario en cada track y sugiere la próxima lección.

**Rastro Diario (retención) — repetible, Aura bajo, con rachas:**
| # | Misión | Aura | Nota |
| --- | --- | ---: | --- |
| D1 | **Rondar el territorio** (login diario + revisar la Reserva Sagrada) | +2 | Base de la racha |
| D2 | **Caza del día** (1 transacción o aporte) | +5 | Alimenta Motor B |
| D3 | **Racha "Vigilia del Santuario"** (7 / 30 / 90 días seguidos de hold) | +15 / +50 / +120 | Recompensa por consistencia |
| D4 | **Alimenta el Fuego** (participar cuando el dashboard "El Fuego" está activo) | +8 | Vincula al Throttle/quema |

**Rugido de la Manada (viral + volumen) — social y grupal:**
| # | Misión | Aura | Objetivo |
| --- | --- | ---: | --- |
| R1 | **El Primer Rugido / Proof of Roar** (contenido verificable sobre LUKASH, validado por LUKAI) | +30 | Viral |
| R2 | **Trae a la Manada** (referido que llega a Rastreador) | +40 / referido | Adquisición (CAC ~$1) |
| R3 | **Duelo de Tótems** (tu criatura vs otra, stats basados en actividad real, apuesta en $LUKA) | +12 / duelo | Volumen + cohesión |
| R4 | **Conquista de Territorio** (tu Manada reclama y defiende una zona con Stake de Batalla) | +25 | Volumen (bloquea $LUKA) |
| R5 | **Caza Mayor** (misión transaccional: volumen acumulado del mes) | +30 | Volumen Motor A/B |
| R6 | **Spot the Whale / Meme Wars** (retos de comunidad semanales) | +10 | Viral, engagement |

### 5.3 Juegos de la Jungle Arena (ADR-026)

| Juego | Mecánica | Motor/Fee |
| --- | --- | --- |
| **Duelo de Tótems** | Tu criatura vs otra. Stats basados en actividad real (Aura, holding, misiones). Apuesta en $LUKA. | Motor D Capa 2, 3% |
| **Caza del Tesoro** | LUKAI esconde drops en el mapa. Pistas diarias. Consume Energía. | Energía (Capa 2) |
| **Conquista de Territorio** | Manadas reclaman zonas del mapa con Stake de Batalla temporal. | Motor D Capa 2 |
| **Predicción Relámpago** | ¿Sube o baja? 1 predicción diaria gratuita. Streak multiplicador. | Sin fee (engagement) |
| **Misiones de LUKAI** | Retos personalizados diarios: ahorra, invita, aprende, comparte. | Varía por misión |
| **Retos Virales / Proof of Roar** | Contenido verificable sobre LUKASH → validado por LUKAI → Aura. | Sin fee (adquisición) |
| **Battle Pass ("Temporada de Caza")** | Track gratuito + premium. Misiones estacionales mensuales. Rewards en $LUKA + skins. | Premium: fee en $LUKA |

### 5.4 El loop (diario → semanal → temporada)

- **Diario:** Rastro Diario (D1–D4) + 1 lección de Escuela Financiera + Predicción Relámpago + Energía para jugar. Genera el hábito.
- **Semanal:** misiones del Rugido (R1, R3, R6) + progreso en el Sendero del Aprendiz. Duelos de Tótems. Misiones de LUKAI.
- **Temporada de Caza (mensual):** Battle Pass con leaderboard de Aura por usuario y por Manada.
  Recompensas de temporada en $LUKA desde el pool **Marketing/Staking** + skins de cNFT + badges. El
  leaderboard **celebra contribución** (aprendizaje, Manada, aportes), no solo volumen.

### 5.5 Economía del juego (sinks y fuentes)

- **Energía** (crédito interno **no-transferible**, sink de tokens): se compra con $LUKA como servicio
  Motor D Capa 2 → distribución **35/35/15/15** (v4.2). Cada acción de la Arena (atacar/defender/cazar)
  consume Energía. Es el sumidero que da valor real al juego sin inflar recompensas.
- **Stake de Batalla:** para Conquista de Territorio / Duelos, la Manada bloquea $LUKA temporalmente →
  genera fee Motor D + presión de hold.
- **Bet & Win:** apuestas de habilidad (no azar). Fee de entrada Motor D Capa 2 (3% $LUKA) + rake sobre el
  pozo, **distribuido 35/35/15/15**. Otorga +8 Aura por apuesta liquidada.
- **Geo-Drops / Presas:** LUKAI lanza recompensas ($LUKA/NFT desde pools Marketing/Staking) en coordenadas o
  eventos; reclamarlas exige Energía/Stake. Motor de eventos y volumen.
- **Battle Pass Premium:** compra mensual en $LUKA que desbloquea el track premium con misiones y rewards
  exclusivas. Fee distribuido 35/35/15/15.

### 5.6 Anti-abuso (para que el Aura signifique algo)
- Aura solo por acciones verificables on-chain o validadas por LUKAI (Proof of Roar).
- Cooldowns por misión repetible; tope diario de Aura por fuente.
- Naturaleza Soulbound: el Aura no se transfiere ni se vende.
- Misiones sociales requieren verificación (no basta "postear"); referidos cuentan solo al llegar a Rastreador.
- Decay por inactividad (2%/sem tras 90d) evita farmear y abandonar.

---

## 6. Resumen de cNFTs en el ecosistema

| cNFT | Tipo | Cantidad por usuario | Propósito |
| --- | --- | --- | --- |
| **Tótem Nativo** | cNFT (comprimido) | Múltiple | Staking en $LUKA (Capa 1, 1.5%). 3 tiers. |
| **Tótem Universal** | cNFT (comprimido) | Múltiple | DeFi Capa 1 premium (1.5%). Aura ≥500. 3 tiers. |
| **Tótem Estándar** | cNFT (comprimido) | Múltiple | Multi-activo (SOL/USDC, 2%). Etapa 3. 3 tiers. |
| **Sello de Manada** | cNFT (comprimido) | Múltiple | Membresía, acceso, participación en Manada. Custom por líder. |
| **Avatar** | NFT completo (Metaplex) | **Uno** | Identidad on-chain. Metadata dinámica (Aura). Transferible. |

---

## 7. Cómo cada objetivo queda cubierto

| Objetivo de Sebastián | Cubierto por |
| --- | --- |
| **Educación financiera** | Sendero del Aprendiz (A1–A7) + Escuela Financiera (4 tracks paralelos) + LUKAI como tutor chamán. Formato TikTok, divertido, desde 13 años. |
| **Retención diaria** | Rastro Diario (D1–D4), rachas "Vigilia del Santuario", Predicción Relámpago, Energía, decay que premia volver, Battle Pass mensual. |
| **Adquisición viral** | Rugido de la Manada: Proof of Roar (R1), referidos (R2), Duelos de Tótems (R3), retos virales, Meme Wars. CAC ~$1. |
| **Volumen económico** | Misiones transaccionales (R4, R5), Bet & Win, Stake de Batalla, Energía (compra con $LUKA), Sellos de Manada (fees Capa 2) → todo alimenta Motores A/B/D y la Reserva Sagrada. |

---

## 8. Pendientes de este diseño (para cerrar antes de contratos de la App)
- Valores exactos de Energía (costo en $LUKA por acción) y topes diarios de Aura por fuente.
- Reglas de Duelos de Tótems (formato, duración, criterio de victoria, cálculo de stats).
- Tamaño y cadencia de los pools de recompensa de Temporada (desde Marketing/Staking).
- Arte visual de Tótems por tier (Bronce/Plata/Oro) y por categoría (Nativo/Universal/Estándar).
- Diseño visual del Avatar y su evolución por nivel de Aura.
- Fórmula exacta del `Aura = f(volumen, hold, IA, victorias, puntualidad, misiones)` con pesos.
- Fee del mint del Avatar NFT (ADR-024: por definir).
- Montos de Tótem Estándar (SOL/USDC) — Etapa 3.
- Contenido de los 4 tracks de la Escuela Financiera (lecciones, niveles, progresión).
- Diseño del Battle Pass mensual (track gratuito vs premium, rewards, misiones estacionales).
- Reglas de Predicción Relámpago (streak bonuses, fuente de datos de precio).
