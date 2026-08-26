# Diseño: AURA + JUNGLE ARENA — Sistema de Reputación y Juego

> v1.0 · 2026-08-19 · Base: Protocolo v4.2 (mecánica) + Síntesis de Discovery + decisión de 5 niveles.
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

### 1.1 Los 5 niveles (arco narrativo cachorro → sabio)

| Nivel | Aura | Narrativa | Beneficios desbloqueados |
| --- | --- | --- | --- |
| **Cachorro** | 0–499 | Acabas de llegar a la selva | Acceso básico a servicios Motor D Capa 2. Manadas estándar. cNFT Nativo $LUKA (0%). Misiones del Sendero del Aprendiz. |
| **Rastreador** | 500–1,999 | Aprendiste a leer las huellas | **Desbloquea cNFT Tótem Universal → DeFi Capa 1** (gate ≥500, igual que v4.2). Voto en parámetros Motor D (DAO). Acceso prioritario a nuevos cNFT. |
| **Cazador** | 2,000–4,999 | Cazas por ti mismo | LP LUKASH 90d con +12% rewards. Visibilidad en ranking de Manadas. Acceso anticipado a integraciones Capas 3A/3B. Puede liderar Manada. |
| **Alfa** | 5,000–9,999 | Proteges a tu manada | Gobernanza avanzada. LP Fundador 365d con +20% rewards. Override Anti-Whale automático. Multiplicador de Aura ×1.15. |
| **Glow** | 10,000+ | Eres leyenda de la selva | Gobernanza premium (peso extra). Acceso a instrumentos **RWA y T-bills tokenizados** (Fase 3). Multiplicador de Aura ×1.30. Skin cNFT legendario. Puede fundar "Dinastías" (federación de Manadas). |

> **Compatibilidad con v4.2:** los umbrales 500 / 2,000 / 5,000 se preservan exactos (Cachorro=Cub,
> Rastreador=Glow, Cazador=Alpha, Alfa=Emperor). Solo se añade el pináculo **Glow (10,000+)**,
> que hereda los beneficios de RWA/T-bills. Ningún gate de cNFT existente se rompe.

### 1.2 Calibración base (cómo se gana Aura — heredado de v4.2 §7D.1)

| Acción | Aura | Nota |
| --- | ---: | --- |
| Holding activo cNFT Tótem Universal | 10 / sem | Máximo incentivo de holdeo premium |
| Holding activo cNFT Nativo o Estándar | 5 / sem | Holding comprometido en cualquier instrumento |
| Participación en Manada activa (Capa 2) | 15 / evento | Por transacción completada en Manada |
| Apuesta Bet & Win completada (Capa 2) | 8 / evento | Independiente del resultado |
| Acceso a evento o marketplace (Capa 2) | 5 / evento | Por uso de servicios sociales |
| **Completar misión educativa (Jungle Arena)** | **20 / misión** | **Máxima ponderación — premia el aprendizaje** |
| Duelo de Manadas (participación) | 12 / duelo | Cohesión grupal |
| Staking activo $LUKA (mín 7d) | 3 / día | Requiere staking, no solo holding |
| LP Comprometido 30 / 90 / 365d | 5 / 10 / 20 / sem | Escalonado por lock |

Los multiplicadores de nivel (×1.15 Alfa, ×1.30 Glow) aplican sobre estas ganancias.
**PENDIENTE (ajustable por gobernanza, Timelock 48h en Etapa 2A):** valores de lanzamiento, se recalibran con datos reales.

---

## 2. JUNGLE ARENA — El Mapa de Caza

Pantalla-juego (Pestaña 3 de la App). Un **mapa de senderos** estilo videojuego donde cada nodo es una
misión. Nodos completados en oro brillante, activo pulsando, bloqueados en gris. LUKAI actúa como
chamán-guía y comentarista. Economía **no inflacionaria**: valor entre participantes + quemas del Motor B,
sin emisión de recompensas fuera de los pools Marketing/Staking ya asignados.

### 2.1 Tres senderos = los 4 objetivos

| Sendero | Objetivo primario | Qué contiene | Motor |
| --- | --- | --- | --- |
| **Sendero del Aprendiz** | Educación | Misiones learn-to-earn: quizzes, simuladores, "gestiona tu Vault Personal" | Motor D Capa 2 |
| **Rastro Diario** | Retención | Loop diario: login, racha ("Vigilia del Santuario"), caza del día | Motor D Capa 2 |
| **Rugido de la Manada** | Viral + Volumen | Raids sociales (Proof of Roar), referidos, Duelos de Manada, misiones transaccionales | Motor A/B/D |

### 2.2 Catálogo de Misiones de Caza (valores de lanzamiento)

**Sendero del Aprendiz (educación) — Aura alto, one-time:**
| # | Misión | Pasos | Aura | Desbloquea |
| --- | --- | --- | ---: | --- |
| A1 | **Primeras Huellas** | Quiz de onboarding: ¿qué es LUKASH y la Reserva Sagrada? | +20 | cNFT Nativo (Tótem Bronce) |
| A2 | **La Reserva Sagrada** | Aprende cómo la Reserva respalda a $LUKA + haz tu 1er aporte (mín 1 $LUKA) | +20 | Sendero Rastro Diario |
| A3 | **El Rugido del Ahorro** | Simulador: interés compuesto y hold | +20 | — |
| A4 | **Cazar sin Miedo** | Módulo de riesgo/volatilidad + configura tu 1er staking | +50 | Activa staking del usuario |
| A5 | **El Puente DeFi** | Aprende Capa 1 + mint tu 1er cNFT Tótem Universal (requiere Rastreador) | +50 | DeFi Capa 1 |

**Rastro Diario (retención) — repetible, Aura bajo, con rachas:**
| # | Misión | Aura | Nota |
| --- | --- | ---: | --- |
| D1 | **Rondar el territorio** (login diario + revisar la Reserva) | +2 | Base de la racha |
| D2 | **Caza del día** (1 transacción o aporte) | +5 | Alimenta Motor B |
| D3 | **Racha "Vigilia del Santuario"** (7 / 30 / 90 días seguidos de hold) | +15 / +50 / +120 | Recompensa por consistencia |
| D4 | **Alimenta el Fuego** (participar cuando el dashboard "El Fuego" está activo) | +8 | Vincula al Throttle/quema |

**Rugido de la Manada (viral + volumen) — social y grupal:**
| # | Misión | Aura | Objetivo |
| --- | --- | ---: | --- |
| R1 | **El Primer Rugido / Proof of Roar** (contenido verificable sobre LUKASH, validado por LUKAI) | +30 | Viral |
| R2 | **Trae a la Manada** (referido que llega a Rastreador) | +40 / referido | Adquisición (CAC ~$1) |
| R3 | **Duelo de Manadas** (competencia de rendimiento/actividad entre Manadas) | +12 / duelo | Volumen + cohesión |
| R4 | **Conquista de Territorio** (tu Manada reclama y defiende una zona con Stake de Batalla) | +25 | Volumen (bloquea $LUKA) |
| R5 | **Caza Mayor** (misión transaccional: volumen acumulado del mes) | +30 | Volumen Motor A/B |
| R6 | **Spot the Whale / Meme Wars** (retos de comunidad semanales) | +10 | Viral, engagement |

### 2.3 El loop (diario → semanal → temporada)

- **Diario:** Rastro Diario (D1–D4) + Energía para jugar. Genera el hábito.
- **Semanal:** misiones del Rugido (R1, R3, R6) + progreso en el Sendero del Aprendiz. Duelos de Manada.
- **Temporada de Caza (mensual/trimestral):** ligas con leaderboard de Aura por usuario y por Manada.
  Recompensas de temporada en $LUKA desde el pool **Marketing/Staking** + skins de cNFT + badges. El
  leaderboard **celebra contribución** (aprendizaje, Manada, aportes), no solo volumen.

### 2.4 Economía del juego (sinks y fuentes)

- **Energía** (crédito interno **no-transferible**, sink de tokens): se compra con $LUKA como servicio
  Motor D Capa 2 → distribución **35/35/15/15** (v4.2). Cada acción de la Arena (atacar/defender/geo-drop)
  consume Energía. Es el sumidero que da valor real al juego sin inflar recompensas.
- **Stake de Batalla:** para Conquista de Territorio / Duelos, la Manada bloquea $LUKA temporalmente →
  genera fee Motor D + presión de hold.
- **Bet & Win:** apuestas de habilidad (no azar). Fee de entrada Motor D Capa 2 (3% $LUKA) + rake sobre el
  pozo, **distribuido 35/35/15/15** (NO el viejo 30/30/40). Otorga +8 Aura por apuesta liquidada.
- **Geo-Drops / Presas:** LUKAI lanza recompensas ($LUKA/NFT desde pools Marketing/Staking) en coordenadas o
  eventos; reclamarlas exige Energía/Stake. Motor de eventos y volumen.
- **cNFT evolutivo ("Estatus de Depredador"):** representación visual del nivel de Aura (skins por nivel:
  Cachorro → … → Glow legendario). Niveles altos dan multiplicadores de staking y descuentos de fee.

### 2.5 Anti-abuso (para que el Aura signifique algo)
- Aura solo por acciones verificables on-chain o validadas por LUKAI (Proof of Roar).
- Cooldowns por misión repetible; tope diario de Aura por fuente.
- Naturaleza Soulbound: el Aura no se transfiere ni se vende.
- Misiones sociales requieren verificación (no basta "postear"); referidos cuentan solo al llegar a Rastreador.
- Decay por inactividad (2%/sem tras 90d) evita farmear y abandonar.

---

## 3. Cómo cada objetivo queda cubierto

| Objetivo de Sebastián | Cubierto por |
| --- | --- |
| **Educación financiera** | Sendero del Aprendiz (A1–A5), +20/misión, máxima ponderación. LUKAI como tutor chamán. |
| **Retención diaria** | Rastro Diario (D1–D4), rachas "Vigilia del Santuario", Energía, decay que premia volver. |
| **Adquisición viral** | Rugido de la Manada: Proof of Roar (R1), referidos con leaderboard (R2), Duelos (R3). CAC ~$1. |
| **Volumen económico** | Misiones transaccionales (R4, R5), Bet & Win, Stake de Batalla, Energía (compra con $LUKA) → todo alimenta Motores A/B/D y la Reserva. |

---

## 4. Pendientes de este diseño (para cerrar antes de contratos de la App)
- Valores exactos de Energía (costo en $LUKA por acción) y topes diarios de Aura por fuente.
- Reglas de Duelos de Manada (formato, duración, criterio de victoria).
- Tamaño y cadencia de los pools de recompensa de Temporada (desde Marketing/Staking).
- Curva de skins del cNFT evolutivo por nivel.
- Fórmula exacta del `Aura = f(volumen, hold, IA, victorias, puntualidad, misiones)` con pesos.
