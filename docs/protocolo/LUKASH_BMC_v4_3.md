# LUKASH PROTOCOL

## BUSINESS MODEL CANVAS

### v4.3 — Las Finanzas del Futuro, Disponibles Hoy para Toda Latinoamérica

Identidad Cultural · Experiencia Financiera Gamificada · Manadas como Economía Comunitaria · Todo LATAM + Brasil + España

Contratos v10 desplegados en Solana Devnet · Fase 1: $500K

Ing. Sebastián Botero Pabón | Agosto 2026

**CONFIDENCIAL · USO EXCLUSIVO PARA INVERSORES**

| Verificación | Contratos Devnet | Guardian de Pausa | KASH Shield |
|---|---|---|---|
| Smart contracts v10, 2066+ líneas Anchor/Rust | Program ID `AmRW...Cuy` activo en Solana devnet | 2-de-3, scope pausa/veto, sunset Etapa 3 (ADR-022) | Circuit Breakers + Anti-Whale + Exit Fee + Transfer Hook (Token-2022) |

---

## SOBRE ESTE DOCUMENTO — CAMBIOS v4.2 → v4.3

El BMC v4.3 consolida las decisiones estratégicas adoptadas entre agosto 19 y agosto 28 de 2026 (ADR-017 a ADR-027, más las consolidaciones ADR-P01 a ADR-P08). Todos los cambios reflejan decisiones cerradas y aprobadas por el fundador.

| Área | v4.2 | v4.3 | Justificación |
|---|---|---|---|
| **Rebrand completo (ADR-017)** | Terminología "jaguar" en seguridad, productos, narrativa y niveles | Eliminación total de la palabra. KASH Shield, KASH Exit Fee, KASH Lock, Tótems, LUKASH Pay/Chat/Games, LUKAI. Niveles Aura en español: Cachorro→Rastreador→Cazador→Alfa→Emperador→Shamán→Titán. Narrativa: Camino del Rugido, El Rugido de la Manada | Asociaciones con retórica populista en LATAM (economías jaguar). La identidad visual del felino se conserva; el problema era la palabra |
| **Aura — 7 niveles (ADR-002, ADR-005)** | "Jaguar Score" con 4 niveles (Cub/Jaguar/Alpha/Emperor) | **Aura** con 7 niveles (Cachorro 0-499 / Rastreador 500-1499 / Cazador 1500-2999 / Alfa 3000-4999 / Emperador 5000-9999 / Shamán 10000-24999 / Titán 25000+). Gate del Tótem Universal en Rastreador (≥500). Exención Anti-Whale/Exit Fee en Titán (≥25000) | Progresión más granular. Nombres en español son marca, no se traducen |
| **Presupuesto $500K (ADR-018)** | App 50% / Liquidez 25% / Marketing 15% / Legal 10% | Tech 35% ($175K) / Liquidez 20% ($100K) / Marketing+MM 30% ($150K) / Legal 10% ($50K) / Reserva 5% ($25K) | Simulaciones muestran 44% espiral de muerte con campaña débil vs 0% con moderada/agresiva. Marketing subdimensionado en v4.2 |
| **Estrategia MM/KOL (ADR-019)** | MM como asociación clave desde TGE | TGE sin MM (Ruta A community-only) → MM contingente mes 2-3 si volumen <$500K/día por 5d consecutivos → CEX mes 4+. KOLs hispanos en $LUKA vesteado. Regla: tokens vesteados = servicios; KASH Sociedad = solo capital/inversores | Fair launch creíble. MM solo si el mercado lo requiere |
| **Rondas escalonadas (ADR-020)** | Sin estructura formal de rondas | Seed ángeles $500K/5-8% (~$6-10M valoración) → Estratégica fintech $1-2M/5-8% (~$15-25M). KASH Sociedad: ~17% fundador / 5-8% seed / 5-8% estratégica / ~6-13% reserva | Ángeles invierten en visión; fintechs invierten en tracción demostrada |
| **Identidad fundador (ADR-021)** | Sin definición explícita | Pseudónimo conocido. KYC para inversores, alias para comunidad, doxx público evaluado mes 6-12 | Seguridad personal en LatAm + flexibilidad regulatoria pre-framing |
| **Vault composición (ADR-P01)** | Incluía oráculos como 5% del Vault (sumaba 105%) | cBTC 35%, SOL 15%, SOL/LST 20%, USDC reserva 25%, USDC lending 5% (suma 100%). Oráculos = infraestructura O&M, no reserva | Corrección de la discrepancia del 105% |
| **Token-2022 (ADR-015-5)** | Sin especificación de estándar de token | Token-2022 con Transfer Hook para mainnet. Anti-Whale + KASH Exit Fee en cada transferencia on-chain, imposible de evadir | Sin Transfer Hook, el value prop anti-dump quedaría vacío |
| **Equipo/Fundador 2% (ADR-014)** | Sin fila explícita de equipo en la tabla de supply | 200M tokens (2%), cliff 12m + lineal 48m. Marketing/CEX reducido de 10% a 8%. Economía de fundador en 3 capas: O&M / Equipo / Vault Sociedad | Alineación + gobernanza + upside. Resuelve el hueco de la tabla |
| **Tótems (ADR-P05, ADR-023, ADR-024)** | "cNFTs Jaguar" con tipos poco diferenciados. Capa 0 con 0% fee | **Capa 0 eliminada.** 3 tipos con tiers visuales (Bronce/Plata/Oro por monto): Tótem Nativo (Capa 1, 1.5%, gated por misión educativa), Tótem Universal (1.5%, gate Aura≥500), Tótem Estándar (2%, Etapa 3). Staking solo existe a través de Tótems | Cada depósito alimenta la Reserva desde la primera interacción. UX simplificada: el usuario solo conoce Tótems, no "staking" |
| **Avatar NFT (ADR-024)** | No existía separación Avatar/Tótem | Avatar = identidad del usuario (uno por persona). Empieza off-chain (gratis), evoluciona con Aura. Se puede mintear como NFT completo (Metaplex) tras misión milestone. Diferente de Tótem (instrumento financiero, cNFT, múltiples por usuario) | Separar identidad de instrumentos. Mercado secundario: Avatar con historial alto tiene valor intrínseco |
| **Sellos de Manada (ADR-025)** | No existían | Nueva categoría de cNFT. Creados por líder de Manada (Aura ≥ Cazador) para membresía/acceso. 5 tipos: Vaca, Fondo, Negocio, Evento, Club. Fees Motor D Capa 2 (3%/$LUKA). Manadas activas desde lanzamiento App | Las Manadas son el espíritu comunitario del proyecto desde día 1 |
| **Guardian de Pausa (ADR-022)** | Tridente Multisig 3-de-3 con poderes amplios | Renombrado Guardian de Pausa. Solo puede pausar y vetar (no mover fondos ni modificar parámetros). 2-de-3 (tolera pérdida de un firmante). Sunset automático en Etapa 3 o por voto DAO | Reduce superficie regulatoria. El protocolo no depende de 3 personas para operar |
| **Educación + Gaming (ADR-026)** | Jungle Arena con 3 senderos | 4 senderos (+ Escuela Financiera con 4 tracks paralelos). Educación multi-track: finanzas personales, ciudadano, cripto, LUKASH. Gaming: Duelos de Tótems, Predicción Relámpago, Battle Pass mensual. Formato TikTok. Gameplay primero, educación como consecuencia | Divertido para un joven de 13 años y para un adulto. No quizzes aburridos |
| **Totem Guard (ADR-027)** | No existía seguro individual | Seguro paramétrico opt-in para Tótems. Dos capas: (1) Póliza externa financiada por O&M, (2) Pool Totem Guard financiado por micro-primas del usuario. Cascada: Totem Guard → Póliza externa → Reserva NO se toca. Payouts automáticos por oráculo (sin claims) | La Reserva Sagrada nunca se usa para cubrir pérdidas. Diferenciador: ningún protocolo Solana ofrece seguro embebido |
| **KASH Shield (ADR-015)** | Anti-Whale por % del supply | Anti-Whale por % del pool de liquidez (no supply). Exit Fee 5%/3%/1%. Circuit Breaker. Seguro Anti-Exploit (5% Vault, herramienta DAO Etapa 4). Guardian de Pausa 2-de-3 | El umbral por supply era brutal al inicio ($10K al TGE) y laxo después |
| **Lanzamiento faseado (ADR-009)** | Sin secuencia explícita | F0 comunidad → F1 solo cripto → F2 App → F3/F4 instrumentos. NUNCA vender retornos. Marketing vende identidad/misión/transparencia | Mantiene perfil de utilidad (no security) |

---

## 01 PROPUESTA DE VALOR

### Las Finanzas del Futuro · Disponibles Hoy · Para Toda Latinoamérica

El sistema financiero global está migrando a blockchain. No es una predicción — es un proceso en marcha. BlackRock, Franklin Templeton, Siemens, Apollo: las instituciones más grandes del planeta ya tokenizaron activos reales en cadena. LUKASH no llega a competir con la banca tradicional: llega a ser la primera puerta de entrada de 650 millones de latinoamericanos a esa nueva infraestructura financiera — construida sobre Solana, respaldada por activos duros, y diseñada para la cultura y la economía de la región.

### 1.1 El Contexto — La Mayor Migración de Capital de la Historia

La tokenización de activos del mundo real (RWA) es el cambio estructural más importante en finanzas desde la creación de los fondos mutuos. No es especulación. Es infraestructura. Y ya está ocurriendo.

| Indicador | Dato verificado |
|---|---|
| Mercado RWA tokenizado en 2022 | $5,000 millones — experimentos institucionales (Venturebloxx / RWA.xyz) |
| Mercado RWA tokenizado a mediados de 2025 | $24,000 millones — crecimiento de 380% en tres años (RedStone, Gauntlet, RWA.xyz, junio 2025) |
| Proyección BCG para 2030 | $16 billones en activos tokenizados. Standard Chartered proyecta $30 billones para 2034 |
| Quiénes ya lo hacen hoy | BlackRock (fondo BUIDL $2,900M on-chain), Franklin Templeton (fondos tokenizados), Siemens (bono €300M on-chain), Apollo (crédito privado tokenizado). No es el futuro — es 2025 |
| Claridad regulatoria global 2025 | EEUU: Ley GENIUS para activos tokenizados. Hong Kong: Ordenanza de Stablecoins agosto 2025. Europa: MiCA activo. El entorno regulatorio global converge hacia blockchain |
| La brecha en LatAm (Global Findex 2025 + Credicorp IIF 2024) | 7 de cada 10 adultos en LatAm tiene una cuenta digital. Pero solo 3 de cada 10 accede a instrumentos financieros reales que le permiten crecer. Brecha activa: ~260 millones de personas |

> **LA TESIS:** Si el sistema financiero global está migrando a blockchain, y 260 millones de latinoamericanos están excluidos del sistema financiero avanzado, la oportunidad es construir la puerta de entrada a esa migración para esos 260 millones. Eso es LUKASH — las finanzas del futuro, disponibles hoy.

### 1.2 El Estándar KASH — La Reserva Soberana que No Para de Crecer

El corazón del protocolo: cada transacción construye una reserva de activos duros reales. Bitcoin nativo. Dólares digitales. SOL con rendimiento. Verificable por cualquier persona en tiempo real. Sin intermediarios. Sin papeles. Sin promesas.

> **PRECIO KASH (P_th) = Valor Total del Vault / Supply Circulante**
>
> El piso de valor que el protocolo garantiza con activos reales. Crece con cada transacción. Verificable on-chain en tiempo real.

| Escenario | Vault Mediana (5 años) | Rango P10–P90 | Espiral | Múltiplo precio |
|---|---|---|---|---|
| **Conservador** (fair-launch community-only) | $90M | $38M – $326M | 40% | 4,381x |
| **Base** (fair-launch + 3 KOLs vesteados) | $388M | $197M – $1,146M | 0% | 22,799x |
| **Agresivo** (KOLs virales + Manadas activas) | $1,882M | $741M – $5,705M | 0% | 107,126x |

*Fuente: Monte Carlo v4.3 (200 iteraciones × 3 campañas × 5 años, ago 2026). Motor fiel al contrato lib.rs v10.2. Parámetros: ADR-016 (cola drena todos modos), ADR-018 (Vault ini $100K), ADR-019 (TGE sin MM), ADR-023 (Capa 0 eliminada). Los valores absolutos son del modelo optimista (ADR-008/H10) — lo válido para decisiones es lo estructural: % espiral, orden de escenarios, sensibilidades relativas. Contratos v10.2 desplegados en Solana devnet verifican la mecánica on-chain.*

`[Cambio v4.3: Composición del Vault corregida — ADR-P01. cBTC 35%, SOL 15%, SOL/LST 20%, USDC reserva 25%, USDC lending 5%. Suma 100%. Oráculos reclasificados como infraestructura O&M, no reserva]`

### 1.3 La Experiencia — Las Finanzas del Futuro No Parecen Banco

La banca tradicional genera fricción: formularios, filas, rechazo, opacidad. Las finanzas del futuro generan motivación: progresión, recompensas, comunidad, transparencia total. LUKASH construye esa experiencia sobre una infraestructura blockchain real.

`[Cambio v4.3: Rebrand completo — ADR-017. Toda la terminología actualizada]`

| Elemento gamificado | Lo que parece | Lo que es realmente |
|---|---|---|
| **Aura** (niveles Cachorro→Titán) | Un sistema de logros como en un videojuego | Historial financiero on-chain no transferible. Define acceso a tasas e instrumentos. Nadie puede quitárselo al usuario |
| Jungle Arena — Misiones de Caza | Un juego de habilidades financieras | Educación financiera progresiva. Cada misión enseña conceptos reales: ahorro, riesgo, liquidez, DeFi. El aprendizaje desbloquea instrumentos de mayor rendimiento |
| La Bóveda / El Fuego / La Cola | Un marcador que toda la comunidad sigue en vivo | Dashboard del Vault en lenguaje humano. Crea urgencia de participación genuina, verificable on-chain |
| **Tótem Universal** #1247 | Un NFT coleccionable con identidad visual única | Instrumento financiero tokenizado on-chain — equivalente a un CDT o participación en fondo. APY variable, transferible en mercado secundario |
| Duelos de Manadas | Un torneo de equipos con puntuación | Ranking de rendimiento financiero entre comunidades. Motor viral: competición entre pares con stakes financieros reales |
| Bet & Win $LUKA | Apuestas de habilidad entre usuarios | Mercado de inversión activa gamificado. Gestión de riesgo con piel en el juego. Motor D Capa 2 |

### 1.4 $LUKA como Identidad Cultural — Pagar es un Acto de Pertenencia

En el ecosistema de Manadas, pagar con $LUKA no es elegir un medio de pago. Es una declaración: soy parte de este ecosistema, confío en este protocolo, y mis transacciones lo fortalecen. El diferencial de 0.5% de descuento (Motor D Capa 2) no es la razón — es la confirmación de que tomó la decisión correcta.

`[Cambio v4.3: Rebrand — ADR-017. Tótems reemplazan "cNFTs Jaguar". Aura reemplaza "Jaguar Score". LUKASH Pay/Chat/Games reemplazan los antiguos nombres]`

> **LOS CUATRO ACTOS DE IDENTIDAD $LUKA**
>
> 1. **Pago en $LUKA (Capa 2 Motor D):** "Elijo fortalecer el protocolo del que soy parte. Mis transacciones queman $LUKA o fortalecen la liquidez."
>
> 2. **Holding del Tótem Universal:** "Tengo un instrumento de inversión on-chain — mi Aura lo prueba. Suma 10 pts/semana automáticamente."
>
> 3. **Participación activa en Manada:** "Mi comunidad económica existe on-chain, con contratos inteligentes y sin banco intermediario."
>
> 4. **Aura nivel Emperador (5,000+ pts):** "Construí una reputación financiera que ningún intermediario me puede dar ni quitar. Accedo a instrumentos RWA y T-bills tokenizados."

### 1.5 Las Manadas — La Economía Comunitaria del Futuro On-Chain

En Latinoamérica existen décadas de tradición de ahorro colectivo informal: las "natilleras" en Colombia, las "tandas" en México, las "juntas" en Perú. LUKASH no inventa este comportamiento — lo lleva a la infraestructura financiera del futuro: contratos inteligentes, auditoría on-chain, acceso a activos tokenizados globales.

| Servicio de Manada | Problema real que resuelve | Cómo funciona on-chain |
|---|---|---|
| Crowdfunding y Crowdlending | El préstamo al vecino o al emprendedor local sin banco, sin calificación externa, sin fianza | Contrato inteligente + Aura como historial. LUKAI como árbitro. Liquidación automática |
| Fondos y Seguros Colectivos | La "vaca" de emergencias — cuando alguien tiene una cirugía de urgencia o pierde el empleo | Fondo on-chain con condiciones claras y ejecución automática por oráculos. Sin burocracia |
| Marketplaces Internos | La economía circular del barrio: el panadero, el mecánico, el maestro usando $LUKA con descuentos | $LUKA como moneda interna. Cada transacción genera fees Motor D Capa 2 para el Vault |
| Instrumentos DeFi Colectivos | Acceso grupal a instrumentos inaccesibles individualmente: Tótems de $10,000 divididos entre 20 personas | Tótem compartido. Rendimiento distribuido proporcionalmente. Motor D Capas 1/3A/3B |
| Tokenización RWA (Fase 3) | Primera hipoteca colectiva descentralizada de LatAm. 10 familias compran una propiedad tokenizada juntas | Motor C (USDC) + Motor D Capas 3A/3B. SPVs on-chain. Rendimiento distribuido entre miembros |

> Las Manadas son la natillera del siglo XXI: la misma lógica de confianza comunitaria de siempre, ahora con contratos inteligentes, rendimiento real on-chain, y acceso a la economía financiera global de los activos tokenizados.

### 1.6 Los Cuatro Motores — La Máquina de Captura de Valor

Cuatro motores se activan secuencialmente y se retroalimentan. Cada uno captura valor de un segmento distinto de usuario y lo convierte en capital permanente para la Reserva Soberana. La regla 35/35/15/15 es universal, automática e inamovible.

| Motor | Divisa | Fee | Identidad — qué captura y cómo lo convierte en valor soberano |
|---|---|---|---|
| Motor A | SOL | 4% / 2.5% WL | **EL CAZADOR:** Captura el capital especulativo del mercado cripto. Cada trade en el DEX convierte SOL en Bitcoin, SOL/LST y USDC permanentes en el Vault. El FOMO del mercado financia la Reserva Soberana |
| Motor B | $LUKA | 2.5% / 1.5% WL | **EL MOTOR INTERNO:** Cada transacción cotidiana en la App quema $LUKA (B0) o fortalece la liquidez (B2). Usar el protocolo lo hace más fuerte. El usuario y el sistema ganan juntos |
| Motor C | USDC | 0.5% | **EL MOTOR DE ESCALA:** Las finanzas del día a día. El café, la nómina, la renta. Con fee invisible del 0.5%, cada pago cotidiano genera presión de compra constante sobre $LUKA. Volumen masivo sobre supply fijo = precio estructuralmente creciente |
| Motor D | $LUKA / Multi | 0%→3.5% | **EL ALMA DEL PROTOCOLO:** Las Manadas, el gaming, los Tótems, los eventos, el DeFi externo. Donde $LUKA es identidad cultural, no solo activo financiero. Capa 0: base silenciosa. Capa 2: corazón cotidiano. Capas 3A/3B: puente con el mundo DeFi tokenizado |

### 1.7 Posicionamiento Competitivo — El Error que LUKASH Corrige

> La lección de OlympusDAO, Axie Infinity y Stepn es idéntica: los tres proyectos fallaron por el mismo error estructural — emisión inflacionaria sin piso de respaldo real. LUKASH es la intersección corregida de los cuatro modelos más poderosos del ecosistema cripto y neobancario.

| Modelo Referencia | Lo que hace bien | Error estructural fatal | Corrección en LUKASH |
|---|---|---|---|
| OlympusDAO (OHM) | Reserve-backed token con piso de valor matemático | Emisión inflacionaria (rebase). Sin deflación real | Burn algorítmico desde Motor A. El supply solo baja |
| Axie Infinity | Gaming financiero con economía real entre pares | Emisión ilimitada de recompensas. Espiral inflacionaria terminal | Jungle Arena: economía entre participantes + quemas Motor B. Sin emisión inflacionaria |
| Stepn | Play-to-Earn con activo físico tangible | Emitía tokens para pagar recompensas. Colapso inevitable | Vault como reserva que impide el colapso por diseño matemático |
| Xade / Valora | Neofinanzas para LatAm con interfaz simple | Sin Vault propio. Retención débil. Sin gamificación real | LUKAI + Aura + Manadas = retención estructural medible |

### 1.8 Filosofía — Las Finanzas del Futuro, Hoy

> LUKASH es una Academia de Soberanía Financiera.
>
> El volumen agresivo del Motor A captura capital especulativo.
>
> La identidad cultural del Motor D lo convierte en comunidad real.
>
> La experiencia gamificada de Jungle Arena lo convierte en educación financiera.
>
> Las Manadas lo convierten en economía comunitaria on-chain.
>
> El Estándar KASH lo convierte en reserva soberana permanente.
>
> El éxito se mide en Soberanía Financiera Validada: Aura alta + tenencia de Tótems activos + participación en Manadas + Vault creciendo.
>
> Las finanzas del futuro ya están siendo construidas por BlackRock, Franklin Templeton y los grandes gestores del mundo. LUKASH construye la misma infraestructura para los 260 millones de latinoamericanos que aún están fuera.

---

## 02 SEGMENTOS DE CLIENTES

### Todo LATAM + Brasil + España — 650M de Personas, Una Sola Infraestructura

El protocolo opera con dos segmentos psicográficos que convergen en la plataforma, desplegados en una geografía de prioridad escalonada que cubre todo el continente latinoamericano, Brasil, y la diáspora hispanohablante en España.

### 2.1 Dos Segmentos — Una Plataforma

`[Cambio v4.3: Rebrand — Aura reemplaza "Jaguar Score". KASH Shield reemplaza "Jaguar Shield". Tótems reemplazan "cNFTs Jaguar"]`

| Parámetro | Segmento A — Nativos de la Atención (12–25 años) | Segmento B — Buscadores de Soberanía (25–45 años) |
|---|---|---|
| Motivación Principal | Estatus digital, pertenencia a manadas, entretenimiento y acceso temprano a creación de riqueza | Protección contra inflación latinoamericana, pagos transfronterizos, acceso a instrumentos financieros reales que les permitan crecer |
| Barrera de Entrada | Falta de capital inicial y desconocimiento técnico de DeFi/blockchain | Desconfianza histórica en cripto y falta de tiempo para navegar complejidad técnica |
| Solución LUKASH | Jungle Arena, airdrops por hitos, Aura como currículum financiero digital on-chain | Interfaz LUKAI conversacional; respaldo real del Estándar KASH auditado on-chain; acceso a tokenización RWA en Fase 3 |
| Palanca de Retención | Gamificación profunda, narrativa de manada con lealtad emocional y estatus en Jungle Arena | Seguridad institucional del KASH Shield y transparencia total del Vault Core verificable on-chain |
| Canal Primario | Viralidad en redes, KOLs con KASH Lock, Jungle Arena peer-to-peer | LUKAI conversacional, App con dashboard de Vault, reportes on-chain verificables |
| Producto Central (Tótem) | Tótem Nativo $LUKA (Capa 0): staking sin fee. Tótem Universal cuando la Aura lo habilite | Tótem Estándar SOL/USDC (Capa 3B) como puerta de entrada. Upgrade a Tótem Universal con el tiempo |

### 2.2 Mapa Geográfico — Prioridades y CAC por Mercado

> **CAC DE REFERENCIA:** Nubank reportó $7 CAC en 2023–2024 con 80–90% de clientes llegando por referido orgánico. Cada Manada tiene un CAC de ~$1 por miembro adicional una vez que el líder está activo.

| Prioridad | País / Región | CAC Et.1 | CAC Et.2 viral | Estrategia |
|---|---|---|---|---|
| 1 | Colombia | $5–$8 | $2–$4 | Hub legal y de conocimiento. Sandbox SFC. Nequi y Daviplata como referencia de adopción digital. Lanzamiento primario |
| 1 | México | $5–$8 | $2–$4 | 130M personas. Nubank ya validó el mercado. Mayor ecosistema joven de LatAm. Lanzamiento simultáneo con Colombia |
| 2 | El Salvador + Centroamérica | $1–$3 | $1–$2 | Hub regulatorio CNAD activo. Baja competencia fintech. CAC extremadamente bajo por ser primeros. Centroamérica: Costa Rica, Guatemala, Honduras, Panamá desde El Salvador |
| 2 | Argentina | $3–$5 | $2–$4 | 45M personas. Alta adopción digital. Protección contra inflación = driver perfecto para el Vault. Expansión Fase 2 |
| 2 | Perú + Chile + Ecuador | $4–$6 | $2–$4 | IIF Credicorp 2024 muestra crecimiento consistente en inclusión. Perú y Ecuador: alta base de población sub-bancarizada. Chile: mayor sofisticación = Segmento B fuerte |
| 3 | Brasil | $5–$8 | $3–$5 | 200M personas. Mercado más grande de LatAm. Requiere localización en portugués. Nubank validó que el modelo funciona. Prioridad 3 por requerimiento de idioma, no por falta de potencial |
| 3 | Venezuela + Bolivia + Paraguay | $2–$4 | $1–$3 | Alta motivación por inflación e inestabilidad monetaria. Riesgo regulatorio variable. Entrada con cuidado estratégico |
| 4 | España (diáspora) | $20–$40 | $10–$20 | 4.5M latinoamericanos residentes. Motor principal: remesas transfronterizas. Alto LTV por volumen de remesas. Nicho específico Fase 3 |

### 2.3 Funnel de Soberanía — Trayectoria del Usuario

`[Cambio v4.3: Lanzamiento faseado — ADR-009. F0→F1→F2→F3/F4. NUNCA vender retornos]`

| Fase | Nombre | Acción Central | Motor Activo | Resultado |
|---|---|---|---|---|
| Fase 1 | Entrenamiento | Simulador, onboarding educativo por LUKAI, Jungle Arena, primeros Tótems Nativos | Motor A (DEX) | Aura construida. Airdrops por hitos. Identidad financiera digital. Primer Tótem emitido |
| Fase 2 | Transaccional | Pagos en $LUKA, Manadas activas, Bet & Win, Tótems DeFi, Duelos de Manadas | Motor B + Motor D | Ahorro comunitario activo. $LUKA como medio de pago e identidad. Instrumentos Tótem de mayor rendimiento |
| Fase 3 | Soberanía | Pagos en stablecoin, DeFi externo Capas 3A/3B, dividendos del Vault, acceso a RWA tokenizados | Motor C + ENZ + Capas 3A/3B | Distribución de yield desde mes 13. LTV máximo. $LUKA como acción preferente |

### 2.4 Costo de Salida Psicológico — El Foso de Retención

La Aura acumulada representa la reputación financiera digital verificable on-chain del usuario. Abandonar el protocolo implica perder ese historial construido. Los instrumentos Tótem de mayor rendimiento están gated por el nivel de Aura — el aprendizaje es un activo escaso con valor económico real.

Este mecanismo genera retención orgánica sin incentivos artificiales de corto plazo ni dependencia de soporte humano.

> PENDIENTE FASE APP — Segmentación local por país: Go-to-market específico: regulación local, localización de App, partnerships con neobancos regionales (Nequi, Daviplata, Yape). Brasil requiere versión en portugués.

---

## 03 CANALES DE DISTRIBUCIÓN Y ACCESO

### Ecosistema Solana · Wallet Interactiva · Nodos Sociales · Dashboard Público

### 3.1 Infraestructura de Red — Solana como Ferrocarril

La red Solana proporciona liquidación con finalidad de sub-segundo y costos de transacción de $0.00025–$0.0005 USD. La arquitectura es 100% Solana-nativa: cBTC (cbBTC nativo sin Wormhole), JitoSOL, mSOL y pools en Meteora — eliminando riesgo de bridge en todos los activos del Vault.

`[Cambio v4.3: Token-2022 con Transfer Hook — ADR-015-5. El mint de mainnet será Token-2022 para que el KASH Shield (Anti-Whale + KASH Exit Fee) se aplique en cada transferencia on-chain, imposible de evadir]`

### 3.2 Canales de Distribución

`[Cambio v4.3: KOLs con KASH Lock (rebrand). Estrategia MM/KOL en 3 fases — ADR-019]`

| Canal | Función | Etapa de Activación | CAC Relativo | Motor Alimentado |
|---|---|---|---|---|
| Jungle Arena | Onboarding gamificado. Simulador de Aura. Misiones educativas. Puerta de entrada para Segmento A | Fase 1 | $5–$9 (campaña inicial) | Motor D Capa 2 |
| App LUKASH + LUKAI v4.1 | Canal transaccional principal. Manadas, pagos, Tótems, servicios Motor D. Dashboard La Bóveda/El Fuego/La Cola | Fase 2 | $3–$5 (viral maduro) | Motor B + Motor D |
| Dashboard Público — La Bóveda | Narrativa de progreso colectivo hacia K_min $25M. +15% volumen orgánico en meses 3–12 | Desde TGE | $0 — coordinación social | Motor A |
| KOLs — KASH Lock | Red de creadores hispanos con vesting en $LUKA por hitos de volumen. Sin dumping inmediato. Pagados en tokens vesteados, nunca cash (ADR-019) | Fase 1–2 | $0 directo (vesting por volumen) | Motor A + Motor D |
| Manadas — Líderes de Manada | Cada líder de Manada adquiere miembros adicionales con CAC marginal ~$1 con flywheel activo | Fase 2 | ~$1 por miembro adicional | Motor D Capa 2 |
| DEX (Meteora / Jupiter) | Canal de trading externo. Motor A principal | Desde TGE | N/A — captura, no adquisición | Motor A |
| CEX (Fase 2+ / mes 4+) | Listados en exchanges centralizados. MEXC/Gate.io. Financiado por wallet Marketing/CEX (8% supply) | Fase 2+ (mes 4+) | Cubierto por asignación supply | Motor A (amplificado) |

### 3.3 Proof of Roar — Viralidad Auditable

> **PROOF OF ROAR — Mecanismo de Viralidad Auditable**
>
> Todo contenido viral sobre el protocolo puede verificarse on-chain para recibir recompensas.
>
> Líderes de Manada con incentivos alineados al largo plazo.
>
> Educación obligatoria: los Tótems de mayor rendimiento están gated por Aura.
>
> CAC proyectado: $5–$9 (Et.1) → $3–$5 (Et.2 viral) → $1 o menos (orgánico objetivo).
>
> Dashboard La Bóveda / El Fuego / La Cola: urgencia de participación genuina verificable on-chain.

---

## 04 RELACIONES CON CLIENTES

### Jungle Arena · Gamificación Financiera · Retención Orgánica

El modelo de relación opera sin dependencia de soporte humano. LUKAI v4.1 actúa como el punto de contacto primario — educativo, operativo y emocional. Los Tótems son el vehículo de la relación financiera duradera.

### 4.1 Métricas de Retención Validadas

| +23% | 5x | 3.2x | $3–$5 |
|---|---|---|---|
| Más ahorro con Aura visible (referencia ecosistemas DeFi) | Retención vs. adquisición de usuario nuevo | Engagement en ecosistemas descentralizados vs. aplicaciones tradicionales | CAC proyectado Etapa 2 (flywheel activo) vs $5–$15 neobanco |

### 4.2 Modelo de Relación por Etapa del Funnel

| Etapa | Tipo de Relación | Mecanismo de LUKAI | Tótem en esta Etapa | Indicador de Éxito |
|---|---|---|---|---|
| Entrenamiento | Mentor–Aprendiz | Onboarding conversacional guiado. Sendero del Aprendiz + Escuela Financiera (4 tracks). Jungle Arena como tutor gamificado | Tótem Nativo (1.5%) desbloqueado por misión educativa (Bronce/Plata/Oro). Avatar off-chain evoluciona con Aura | Aura inicial. Primer Tótem adquirido via misión |
| Transaccional | Compañero de Manada | Orquestación B0/B2. Notificaciones de Vault. Alertas de Throttle. Recomendaciones de instrumentos por Aura. Sellos de Manada para participar en Vacas/Fondos/Eventos | Tótem Universal (gate Aura ≥500). Avatar minteado como NFT completo. Totem Guard opt-in. Manadas activas con Sellos | Volumen en Motor D activo. Manada con 3+ miembros activos |
| Soberanía | Socio del Protocolo | Dashboard de dividendos. Reporte de P_KASH. Votación de gobernanza (Etapa 4). Acceso a instrumentos RWA | Tótem Estándar (multi-activo, 2%). Instrumentos DeFi externos | Distribución de yield desde mes 13. LTV máximo |

---

## 05 FUENTES DE INGRESOS

### Los Cuatro Motores · Su Identidad · Sus Flujos · La Sinergia

Los cuatro motores económicos son mecanismos de captura de valor con identidades distintas que se activan secuencialmente y se retroalimentan. La regla 35/35/15/15 los unifica a todos. La R_op no interviene en el flujo operativo normal del Motor B0 ni B2 — la recuperación del precio en B2 ocurre por inercia estructural de los mecanismos existentes.

### 5.1 Jerarquía de Fees por Etapa de Madurez

| Etapa | Fee Base | Fee WL | Motor Principal | Descripción Estratégica |
|---|---|---|---|---|
| Etapa 1 — Génesis | 4.0% | 2.5% | Motor A | Deflación agresiva. Acumulación máxima del Vault. Bootstrapping desde TGE. Referente: $WIF |
| Etapa 2A — App / B0 | 2.5% | 1.5% | A + B0 + D (4 capas) | App en adopción activa. Vault en trayectoria hacia K_min $25M. Motor D 4 capas activas |
| Etapa 2B — Madurez / B2 | 2.5% | 1.5% | A + B2 + D (4 capas) | K(t) >= $25M on-chain. Motor B2 activo (tramo LP recircula). Referente: $BONK ciclo completo |
| Etapa 3 — Soberanía | 0.5% | 0.5% (convergencia) | A + B2 + C + D | Supply fijo 3.3B. ENZ activa. $LUKA como acción preferente con dividendos reales |

### 5.2 Motor A — El Cazador: Capturando el Capital Especulativo

Cada trade en el DEX convierte SOL en activos duros permanentes en el Vault. El especulador recibe exposición al token. El protocolo recibe la reserva. El FOMO del mercado financia la Reserva Soberana.

> **FLUJO COMPLETO MOTOR A — Ejemplo: 100 SOL compran $LUKA en DEX (fee 4% = 4 SOL)**
>
> BÚFER: SOL acumulados hasta ~$500 equivalente → 3 swaps fraccionados vía Jito bundle privado
>
> DISTRIBUCIÓN ATÓMICA:
>
> - 1.40 SOL (35%) → Jupiter → activos KASH → VAULT CORE [35%→cBTC · 25%→USDC · 15%→SOL · 20%→LST · 5%→lending]
> - 1.40 SOL (35%) → Jupiter compra $LUKA → BURN PERMANENTE [presión de compra + supply baja]
> - 0.60 SOL (15%) → Wallet O&M
> - 0.60 SOL (15%) → Jupiter compra $LUKA → Contrato de Staking
>
> EFECTOS NETOS: Supply $LUKA baja | Vault sube | Precio con soporte sube | Stakers recompensados

### 5.3 Motor B — El Motor Interno: Usar el Protocolo lo Fortalece

Cada transacción cotidiana en la App genera un fee en $LUKA. En B0 quema directamente. En B2 fortalece la liquidez del pool. En ambos estados, el Asset Layer compra Hard Assets. El Asset Layer NUNCA quema. La R_op NO interviene en el flujo normal del Motor B — ni en B0 ni en B2.

| Tramo (35/35/15/15) | Estado B0 (K < $25M) — Deflación activa | Estado B2 (K >= $25M) — Estabilidad activa |
|---|---|---|
| Liquidity Layer (35%) | BURN permanente a dirección null. Sin USDC. Sin R_op. La quema es directa y gratuita para el sistema | Recirculación al pool de liquidez (NO quema). Sin R_op. El precio se recupera por inercia estructural: Motor A + KASH Exit Fee + Anti-Whale + cola diferida |
| Asset Layer (35%) | Compra Hard Assets (cBTC/SOL/LST). NUNCA quema. Sin R_op | Compra Hard Assets (cBTC/SOL/LST). NUNCA quema. Sin R_op. La R_op se acumula idle del yield del Vault — no es necesaria para la operación cotidiana |
| Reward Layer (15%) | $LUKA directo a stakers activos | $LUKA directo a stakers activos |
| O&M Layer (15%) | Swap inmediato a SOL/USDC → Wallet O&M | Swap inmediato a SOL/USDC → Wallet O&M |

> **MECANISMOS DE INERCIA ESTRUCTURAL EN B2** — Sin tocar el Vault ni la R_op:
>
> 1. Motor A activo todos los días: mientras haya cualquier volumen de trading, el 35% Asset Layer compra Hard Assets y el 35% LP crea presión compradora sobre $LUKA. No para.
> 2. KASH Exit Fee (5%/3%/1%): las ventas en pánico generan un fee que va al 100% al Vault Core. El pánico mismo fortalece la reserva.
> 3. Anti-Whale: los fees escalonados de transacciones grandes van al Vault Core.
> 4. Cola diferida: las quemas acumuladas durante CONSERVADOR y DEFENSIVO en B0 se ejecutan al 10%/semana al normalizar el precio — deflación adicional en el momento de recuperación.
> 5. Arbitraje natural: cuando el precio de mercado cae muy por debajo del Precio KASH, cualquier inversor racional compra $LUKA barato. El mercado lo ejecuta solo.

### 5.4 Motor C — El Motor de Escala: Las Finanzas del Día a Día

En Fase 3 (Supply fijo 3.3B, ENZ activa), cada pago cotidiano en USDC genera presión de compra constante sobre $LUKA con fee de 0.5%. Volumen masivo sobre supply fijo = precio estructuralmente creciente.

> **FLUJO COMPLETO MOTOR C — Usuario paga 200 USDC (fee 0.5% = 1 USDC)**
>
> - 0.35 USDC (35%) → directo al Vault KASH (refuerza buffer USDC Capa 1)
> - 0.35 USDC (35%) → Jupiter compra $LUKA → inyección al LP [presión de compra real]
> - 0.15 USDC (15%) → Wallet O&M
> - 0.15 USDC (15%) → Jupiter compra $LUKA → distribuye a stakers
>
> En Fase 3, cada transacción cotidiana genera presión de compra neta positiva sobre $LUKA con supply fijo.

### 5.5 Motor D — El Alma: $LUKA como Identidad Cultural (4 Capas)

> ACTUALIZACIÓN CRÍTICA v4.2 vs v3.0: En v3.0 el Motor D tenía un fee único del 1.5%. En v4.2 opera con 4 capas diferenciadas. El diferencial de 0.5% entre pagar en $LUKA vs SOL/USDC es constante — no es penalización, es premio.

`[Cambio v4.3: Tótems — ADR-P05. 3 tipos: Tótem Nativo (Capa 0, 0%), Tótem Universal (Capas 1/3A, 1.5%, gate Aura≥500), Tótem Estándar (Capa 3B, 2%)]`

| Capa | Servicio | Fee $LUKA | Fee SOL/USDC | Identidad y función estratégica |
|---|---|---|---|---|
| Capa 0 | Staking $LUKA · Pools internos | 0% | N/A | La base silenciosa. Exención absoluta y permanente. El rendimiento proviene del 15% de staking de los otros motores — sin doble cobro |
| Capa 1 | DeFi interno premium — instrumentos $LUKA | 1.5% | Solo $LUKA | El club privado. Acceso gated por Tótem Universal (gated por Aura ≥500 — nivel Rastreador). APY variable. Sin garantía de capital |
| Capa 2 | Servicios cotidianos: Crowdfunding · Crowdlending · Fondos · Eventos · Marketplaces · Jungle Arena · Bet & Win | 3% | 3.5% | El corazón de la comunidad. Donde $LUKA es identidad cultural. Diferencial 0.5% premia usar $LUKA. Principal motor de adopción masiva |
| Capa 3A | DeFi externo en $LUKA — Kamino, Marginfi, RWA tokenizados | 1.5% | Swap previo sin cargo + 1.5% | El puente soberano hacia el mundo DeFi tokenizado. El rendimiento externo va 100% al usuario |
| Capa 3B | DeFi externo SOL/USDC — Tótem Estándar | N/A | 2% | La puerta de entrada para usuarios sin $LUKA. Opera como Motor C. Menor fricción para el Segmento B |

### 5.6 La Sinergia entre Motores — El Ciclo Virtuoso

- Motor A captura capital externo → Vault crece → P_KASH sube → atrae más compradores → más volumen Motor A.
- Motor D genera actividad cotidiana → alimenta Motor B → $LUKA se quema o recircula → precio mejora → más incentivo para participar en Motor D.
- Motor C en Fase 3 (supply fijo 3.3B) → compra $LUKA constantemente → presión de compra neta positiva → P_KASH sube → dividendos del Vault crecen.
- Aura gatea acceso a instrumentos premium Motor D Capa 1 → participar más da acceso a más beneficios → más fees → Vault más fuerte.

> **RESULTADO:** Cada interacción humana con el protocolo — desde el trade más especulativo hasta el pago del café más mundano — termina en el mismo lugar: la Reserva Soberana crece, $LUKA es más escaso, y el usuario que participó tiene más que cuando empezó.

### 5.7 Distribución Universal — Regla 35/35/15/15

| Motor | Divisa | 35% Vault / Asset Layer | 35% LP / Quema | 15% O&M + 15% Staking |
|---|---|---|---|---|
| Motor A (SOL) | SOL | → activos KASH → Vault Core [35%cBTC·25%USDC·15%SOL·20%LST·5%lending] | → compra $LUKA → BURN + presión | → Wallet O&M / → $LUKA → Staking |
| Motor B — B0 ($LUKA) | $LUKA | → Asset Layer: Hard Assets. Sin R_op. NUNCA quema | → BURN directo a null. Sin R_op | → Swap SOL/USDC O&M / → $LUKA stakers |
| Motor B — B2 ($LUKA) | $LUKA | → Asset Layer: Hard Assets. Sin R_op. La R_op se acumula idle del yield | → Recirculación LP (NO quema). Precio por inercia estructural | → Swap SOL/USDC O&M / → $LUKA stakers |
| Motor C (USDC) | USDC | → directo al Vault (buffer USDC Capa 1) | → compra $LUKA → LP | → Wallet O&M / → $LUKA stakers |
| Motor D Capa 0 | N/A | EXENTO TOTAL | EXENTO TOTAL | EXENTO TOTAL |
| Motor D Capas 1/2/3A ($LUKA) | $LUKA | → Asset Layer: Hard Assets | B0: BURN / B2: Recirculación LP | → Swap O&M / → $LUKA stakers |
| Motor D Capa 3B (SOL/USDC) | SOL/USDC | → Vault directo (buffer USDC o Hard Assets) | → $LUKA LP/quema | → O&M / → $LUKA stakers |

### 5.8 Proyecciones de Ingresos por Etapa

| Métrica | Rango Mínimo | Rango Sostenible | Rango Optimista |
|---|---|---|---|
| **ETAPA 1 — Motor A principal (30 días)** | | | |
| Volumen Diario | $5M | $35M | $120M |
| Fees (30d) | $6M | $42M | $144M |
| Aporte Vault (35%x70%) | $1.47M | $10.29M | $35.28M |
| Tokens Quemados (30d) | 480M $LUKA | 1.8B $LUKA | 3.2B $LUKA |
| **ETAPA 2 — Motor A + B + D (mensual)** | | | |
| Volumen (DEX + App) | $250M | $1,200M | $3,500M |
| Fees Motor A+B | $6.25M | $30M | $87.5M |
| Motor D adicional al Vault | $625K | $3M | $8.75M |
| **ETAPA 3 — Motor C + D (mensual)** | | | |
| Transacciones Mensuales | 100M | 300M | 550M |
| Volumen Mensual (USD) | $300M | $900M | $1,650M |
| Fees Motor C | $1.5M | $4.5M | $8.25M |
| Motor D adicional | $750K | $2.25M | $4.12M |

### 5.9 Rendimientos del Vault — Regla 70/30

| Capital del Vault | Yield LST 20% (6–8%) | Yield USDC Lending 5% (4–8%) | Yield Total Anual | Aporte R_op (30%) |
|---|---|---|---|---|
| $10M | $120K–$160K | $20K–$40K | $140K–$200K | $42K–$60K |
| $25M (K_min) | $300K–$400K | $50K–$100K | $350K–$500K | $105K–$150K |
| $50M | $600K–$800K | $100K–$200K | $700K–$1M | $210K–$300K |

El 70% del yield del Vault se reinvierte automáticamente para crecimiento compuesto. El 30% alimenta la Reserva Operativa en USDC (R_op), que se acumula como reserva de estabilización pero no interviene en el flujo operativo normal del Motor B0 ni B2. En B0 el tramo LP quema directamente sin necesitar USDC. En B2 el precio se recupera por inercia estructural. La redención soberana queda reservada como opción del DAO en Etapa 3.

### 5.10 Distribución Perpetua — Mes 13+

A partir del hito de activación del KASH Lock (KASH Core alcanza $30M o 12 meses, lo que ocurra primero), el 30% de los fees destinados al componente KASH Sociedad se vuelven líquidos y directos a perpetuidad — flujo de caja constante respaldado por utilidad real, sin dilución de los socios fundadores.

---

## 06 ACTIVIDADES CLAVE

### Orquestación de Estados · Gestión del Vault · KASH Shield · Throttle Dual

### 6.1 Orquestación de Estados por LUKAI v4.1

| Actividad | Descripción |
|---|---|
| Conmutación B0 ↔ B2 | Monitoreo de K(t) en tiempo real vía Pyth. Conmutación automática al cruzar K_min = $25M. El tramo LP pasa de quemar a recircular. Sin intervención humana |
| Routing Tótem por Capa Motor D | Identifica tipo de Tótem y activo para aplicar la capa correcta (0/1/2/3A/3B) con su fee y distribución específicos. Un solo cobro al usuario |
| Rebalanceo del Vault | Compra gradual de Hard Assets con el 35% del Asset Layer. El Vault existente no se toca — solo nuevas entradas |
| Curaduría de instrumentos Tótem | Evalúa riesgo, liquidez y rendimiento histórico. Solo habilita instrumentos que superan el filtro LUKAI. Actualiza APY variable on-chain |
| Calibración de Aura | Actualización en tiempo real según participación activa en Motor D. Capa 0 exenta |

### 6.2 Throttle Dinámico — Dos Algoritmos Independientes

> **DISTINCIÓN CRÍTICA:** Son dos algoritmos con lógicas inversas.
>
> **Etapas 1–2 (pre-ENZ):** Controla la VELOCIDAD DE QUEMA del tramo LP según precio vs EMA30. Protege la R_op en bear markets.
>
> **Etapa 3 (post-ENZ):** Controla la ESTRATEGIA DE COMPRA DE HARD ASSETS con el yield. BULL acumula USDC. BEAR compra Hard Assets agresivamente con el USDC acumulado.
>
> **IMPORTANTE:** En B2 el Vault NO se toca para defender el precio de $LUKA. La defensa opera a través de los flujos de fees (Motor A + KASH Exit Fee + Anti-Whale + cola diferida) — no del Vault existente ni de la R_op.

`[Cambio v4.3: Drenaje de cola en todos los modos + hard-stop ENZ — ADR-016. Escala geométrica: ACEL 25%/sem · NORMAL 10% · CONS 5% · DEF 2%. Supply NUNCA baja de 3.3B]`

### 6.3 Gestión Contra-Cíclica del Vault — Etapas 1 y 2

| Régimen | Activos Volátiles (nuevas entradas) | USDC (nuevas entradas) | Lógica Estratégica |
|---|---|---|---|
| BEAR Confirmado (EMA30 < EMA90 BTC) | 70% | 30% | Acumulación contra-cíclica — compra activos duros baratos para el próximo bull |
| NEUTRAL | 75% | 25% | Proporción estándar. Sin sesgo direccional |
| BULL Confirmado (EMA30 > EMA90 BTC) | 40% | 60% | Construye USDC para el próximo ciclo bear. +22–23% en Vault final vs. proporciones fijas |

---

## 07 RECURSOS CLAVE

### Vault Soberano · LUKAI · Sistema Tótem · Aura · Smart Contracts

`[Cambio v4.3: Rebrand completo — ADR-017. Aura reemplaza "Jaguar Score". Tótems reemplazan "cNFTs Jaguar". KASH Shield reemplaza "Jaguar Shield". Token-2022 con Transfer Hook — ADR-015-5]`

| Recurso | Tipo | Función Estratégica | Mecánica de Seguridad | Diferenciador Clave |
|---|---|---|---|---|
| Vault KASH Core (Reserva Soberana) | Capital Real | Reserva de respaldo inamovible. Determina P_KASH y activa Motor B2. No se toca para defender precio — la inercia estructural es suficiente | 3 capas: Liquidez Inmediata (25% USDC), Trabajo Activo (cBTC+SOL+LST), Reserva Profunda (cBTC 35%). Guardian de Pausa 2-de-3 (ADR-022) | 100% Solana-nativa. 0% riesgo bridge. Contratos v10 verificados en devnet |
| LUKAI v4.1 — Orquestador IA | Inteligencia | Orquestador de estados + IA conversacional. Calibra Aura. Módulo contra-cíclico. Curaduría Tótem | Pyth + Switchboard (redundancia dual). Throttle automático. Sin intervención humana en funciones críticas | v1.0 para TGE (on-chain). v2.0 (IA conversacional) Etapa 2A |
| Sistema Tótem — Instrumento Financiero | Producto Digital | Instrumento de inversión del usuario. APY variable, dos modos de rendimiento, transferible. Gated por Aura | Disclaimer obligatorio on-chain. APY actualizable por LUKAI. Curaduría previa | No existe equivalente en Solana: instrumento financiero + gamificación + Aura en un solo token |
| Aura — Red de Reputación | Capital Social | Currículum financiero on-chain. Genera Costo de Salida Psicológico | On-chain, no transferible. Decaimiento 2%/semana tras 90d inactividad. 7 niveles: Cachorro→Titán | El usuario no abandona su reputación. Retención orgánica real |
| Smart Contracts en Solana (Token-2022) | Infraestructura | PDA de control de supply. Distribución atómica de fees. ENZ automática. KASH Lock. Transfer Hook para Anti-Whale + KASH Exit Fee | Auditoría externa pre-TGE. Multisig para emergencias. Timelock 48h. Token-2022 con Transfer Hook imposible de evadir | Distribución atómica en una transacción — sin riesgo de extracción parcial |
| Pool de Liquidez LP (Meteora) | Liquidez de Mercado | Soporte de trading en DEX. Motor A principal | Circuit Breaker 3 niveles. LP Comprometido reduce free-float vulnerable | LP Comprometido 30d/90d/365d con rewards +5%/+12%/+20% |

---

## 08 ASOCIACIONES CLAVE

### Market Makers · KOLs KASH Lock · Infraestructura Solana · Regulatorio · Auditorías

`[Cambio v4.3: Estrategia MM/KOL en 3 fases — ADR-019. MM contingente, no desde TGE. KOLs hispanos en $LUKA vesteado. Auditor externo desacoplado — ADR-015-3. Rondas escalonadas — ADR-020]`

| Tipo | Socios Objetivo | Rol | Estructura de Alineación |
|---|---|---|---|
| **Market Makers (MM) — Contingente** | Kairon Labs, Gravity Team (Solana) | Fase 1: SIN MM (Ruta A community-only). Fase 2 (mes 2-3): MM contingente si volumen <$500K/día por 5d consecutivos. Modelo loan + call option (1-3% supply inventario) | Compatible con fair launch. Gate de activación por volumen. Presupuesto $40-60K del bucket Marketing+MM |
| **KOLs — KASH Lock** | Creadores financieros hispanos: Hugo Botto, Catalina Castro, Daniel Muvdi, Criptolawyer | Canal de adquisición primario. Educación del Funnel de Soberanía en su audiencia | KASH Lock: tokens $LUKA con vesting cliff 6mo + 12mo lineal, por hitos de volumen generado. Pagados en tokens, nunca cash. Del bucket Marketing/CEX (8% supply) |
| **Auditorías de Seguridad** | Auditor(a) externa a cotizar pre-TGE (Halborn, OtterSec, Sec3, Zellic, Neodyme o similar) | Auditoría pre-TGE. Seguro Anti-Exploit (5% Vault) requiere auditoría activa | Ruta primaria: subsidios (Areta $1M vía Colosseum, grants Solana Foundation/Superteam). Boutique paga $15-25K como fallback |
| **Inversores Seed (ADR-020)** | Ángeles cripto-nativos | $500K por 5-8% del KASH Sociedad. Valoración implícita ~$6-10M. Pre-TGE | Invierten en visión y diseño. KASH Lock con vesting |
| **Inversores Estratégicos (ADR-020)** | Fintechs LatAm, cooperativas, remesadoras | $1-2M por 5-8% del KASH Sociedad. Valoración implícita ~$15-25M. Mes 6-12 post-tracción | Invierten en tracción demostrada. Revalorización entre rondas como incentivo early-stage |
| **Protocolos RWA** | Maple Finance, Centrifuge, Ondo Finance | Activos tokenizados del mundo real para Manadas en Fase 3 | Integración Motor C y Capas 3A/3B Motor D. Las Manadas invierten colectivamente |
| **Infraestructura Solana** | Pyth, Switchboard, Jupiter SDK, Meteora, Kamino/Marginfi, Sanctum, JitoSOL | Datos de precio TWAP, swaps atómicos, yield USDC ocioso, monitor paridad LSTs | Redundancia dual en oráculos (umbral 2%). Sin single point of failure |
| **Hub Regulatorio Estratégico** | El Salvador (CNAD), Sandbox SFC Colombia, Fintech México 2025–2030 | Marco regulatorio favorable. Capital mínimo $2,000. 0% impuestos cripto en El Salvador. Hub Centroamérica | Estructura legal por jurisdicción. Sandbox como canal de validación institucional |
| **Neobancos LatAm (Fase 3)** | Nequi, Daviplata, Yape, Mercado Pago | Rampas fiat on/off para Motor C. Canal de distribución en su base de usuarios | Integración API para pagos USDC. Canal de distribución, no competencia directa |

---

## 09 ESTRUCTURA DE COSTOS

### Optimización Radical · Seguridad Matemática · CAC Viral

### 9.1 Comparativa de Eficiencia Operativa

| Indicador | Banca Tradicional | Neobanco Digital Líder | LUKASH (Proyectado) |
|---|---|---|---|
| CAC — Etapa 1 (campaña activa) | $150–$350 USD | $5–$15 USD (Nubank: $7 en 2024) | $5–$9 USD con KOLs + comunidad orgánica |
| CAC — Etapa 2 (flywheel viral activo) | N/A | $7 USD (Nubank CAC estable) | $3–$5 USD con Manadas activas |
| CAC — marginal orgánico (objetivo) | N/A | <$1 USD (Nubank: 80–90% referidos) | $1 o menos — cada Manada reduce CAC marginal a ~$1/miembro |
| Costo por Transacción | $20–$65 USD (SWIFT internacional) | $0.01–$0.05 USD | $0.00025–$0.0005 USD (Solana) |
| Soporte al Cliente | Call center + oficinas | Equipo humano digital | LUKAI v4.1 IA conversacional 24/7. Costo = $0 por usuario |
| Infraestructura de Red | Física + Digital. Alto CAPEX | Digital. Servidores cloud | Smart contracts en Solana. Costo marginal → $0 por escala |

### 9.2 Presupuesto de Inversión — Fase 1 Optimizada + Fases 2-3

`[Cambio v4.3: Redistribución presupuesto $500K — ADR-018. Nueva distribución: Tech 35% / Liquidez 20% / Marketing+MM 30% / Legal 10% / Reserva 5%. Simulaciones muestran 44% espiral de muerte con campaña débil vs 0% con moderada/agresiva]`

| Categoría | Monto | % | Componentes Clave |
|---|---|---|---|
| **App/Tech** | $175,000 | 35% | Contratos Anchor/Rust (70% avanzados). Auditoría boutique $15-25K (Sec3, Zellic, Neodyme). Frontend MVP LUKASH App. LUKAI v1.0 (orquestador on-chain). KASH Shield completo |
| **Liquidez Inicial** | $100,000 | 20% | Pool $LUKA/SOL en Meteora con LP lock permanente. Liquidez propia (Ruta A community-only). Sin dependencia de MM externo al inicio |
| **Marketing + MM Contingente** | $150,000 | 30% | KOLs hispanos ($LUKA vesteado, $0 cash). Plataformas orgánicas. Community manager. Ads. MM contingente $40-60K (activar solo si volumen <$500K/día por 5d). CEX listing $30K (MEXC/Gate.io) |
| **Legal** | $50,000 | 10% | Abogado cripto. Entidad El Salvador (CNAD ~$7,500). Framing regulatorio por jurisdicción. Estructura SAFE + token warrant para Seed |
| **Reserva Operativa** | $25,000 | 5% | O&M fundador meses 1-6. Emergencias. Buffer de contingencia |
| **TOTAL FASE 1** | **$500,000** | **100%** | |

| Fase | Inversión | Foco | Componentes Clave |
|---|---|---|---|
| Fase 2 | $2,000,000 USD | Product & Scale (400–500K usuarios) | LUKAI v2.0 (IA conversacional, $900K). Adquisición de usuarios mixta pagada/viral ($700K — CAC efectivo $3–$5 con flywheel). Jungle Arena completa ($200K). Manadas con productos específicos. Integraciones Kamino / Tensor / Phantom. OpEx KASH Shield ($200K) |
| Fase 3 | $5,000,000 USD | Utilidad en Economía Real | Expansión red LUKASH Pay (100K+ comercios, NFC/QR). Licencias IFPE México + SEDPE Colombia. Infraestructura RWA (SPVs, T-bills tokenizados). Localización Brasil (portugués). Nicho España (remesas). LUKAI Institutional Bridge. Etapa 4 DAO |

> **SOSTENIBILIDAD:** La Fase 1 es la única que requiere capital externo. Fases 2 y 3 se co-financian con el Vault Sociedad acumulado durante Etapa 1. Sin dependencia de VCs. Los socios fundadores no se diluyen.
>
> **CONDICIÓN PARA $500K:** El diseño técnico completo se realiza antes del primer día de desarrollo. Smart contracts diseñados en pseudocódigo y simulados previo a implementación. Equipo mínimo 2–3 devs con arquitectura prevalidada.

### 9.3 Distribución del Supply — Actualización v4.3

`[Cambio v4.3: Equipo/Fundador 2% — ADR-014. Marketing/CEX reducido de 10% a 8%]`

| Categoría | Porcentaje | Cantidad | Mecánica de Seguridad |
|---|---|---|---|
| Venta (Seed → Public) | 45% | 4,500M $LUKA | Seed ≤5% supply (≤500M) con SAFE + token warrant + vesting on-chain. Lo NO vendido rueda a Public (fair-launch, circulante desde TGE). Ventana Seed cierra en TGE (ADR-013) |
| Pool de Liquidez (LP) | 30% | 3,000M $LUKA | Emparejados y quemados en Meteora. LP Fundador 365d lock obligatorio |
| Marketing / CEX | 8% | 800M $LUKA | Custodiado en Multisig. KOLs y MM asignados aquí. KOLs pagados en tokens vesteados (ADR-019) |
| Airdrops / Comunidad | 10% | 1,000M $LUKA | Distribución por hitos on-chain públicos. No distribuido antes de TGE |
| Staking | 5% | 500M $LUKA | Recompensa base. Motor A financia el staking pool perpetuamente |
| **Equipo / Fundador** | **2%** | **200M $LUKA** | **Cliff 12m + lineal 48m on-chain pre-TGE. Alineación + gobernanza + upside del token (ADR-014)** |

### 9.4 Economía del Fundador — 3 Capas (ADR-014)

`[Cambio v4.3: Estructura explícita de compensación del fundador en 3 capas]`

| Capa | Fuente | Horizonte | Función |
|---|---|---|---|
| O&M (15% de cada fee) | Fees operativos del protocolo | Corto plazo | Compensación por trabajo operativo. Flujo desde día 1 |
| Equipo 2% (200M $LUKA) | Supply asignado | Largo plazo (cliff 12m + 48m) | Upside del token + voz en DAO. Alineación con holders |
| Vault Sociedad (~17%) | 30% de fees del protocolo | Largo plazo (KASH Lock) | Patrimonio del negocio. Independiente del precio de $LUKA |

---

## 10 VALIDACIÓN TÉCNICA

### Verificación On-Chain + Modelo Económico

*Contratos v10.2 (~2100 líneas Anchor/Rust) desplegados en Solana devnet. Proyecciones económicas: Monte Carlo v4.3 (200 iteraciones × 3 campañas × 5 años, ago 2026). Motor de simulación fiel al contrato (replica aritmética entera de lib.rs). 6 SIMs: invariantes, MC principal, sensibilidad, estrés, throttle, MM vs no-MM.*

| Devnet v10 activo | Guardian 2-de-3 | KASH Shield completo | Distribución atómica |
|---|---|---|---|
| `AmRW...Cuy` en Solana devnet | Scope: solo pausa/veto (ADR-022) | CB + Anti-Whale + Exit Fee + Transfer Hook | 35/35/15/15 en una transacción |

### 10.1 Proyecciones Económicas — Monte Carlo v4.3 (ago 2026)

`[Cambio v4.3: Reemplaza modelo v3.1. 200 iteraciones × 3 campañas × 5 años. Motor fiel al contrato (aritmética entera lib.rs). Cola drena todos modos (ADR-016). TGE sin MM (ADR-019). Vesting equipo 2%/12m/48m (ADR-014).]`

| Métrica | Conservador | Base | Agresivo |
|---|---|---|---|
| Vault Mediana (5 años) | $90M | $388M | $1,882M |
| Rango P10–P90 | $38M – $326M | $197M – $1,146M | $741M – $5,705M |
| Riesgo de Espiral de Muerte | 40% | 0% | 0% |
| Múltiplo precio vs TGE | 4,381× | 22,799× | 107,126× |
| Supply quemado (mediana) | 67% | 67% | 66% |

**Hallazgos clave:**
- **Volumen es el driver #1** (SIM 2 sensibilidad): swing $1.29B en Vault. Fee Motor A es #2.
- **Resiliencia** (SIM 3 estrés): crash BTC -80% reduce Vault solo -3.8%. Exploit 15% Vault: -0.3%. Único riesgo sistémico: Motor A -70% permanente.
- **Throttle actual óptimo** (SIM 4): masa crítica marginal en configuración actual.
- **MM vs no-MM** (SIM 5): MM sube Vault 11-19% pero baja precio 10-25% por dilución token loan (3% supply, 40% sell-through). Solo escenario conservador se beneficia significativamente (espiral 34.5%→3.5%). Valida fair-launch (ADR-011/019).
- **Riesgo de ruina total**: 0.0% en todos los escenarios (200 iteraciones cada uno).

### 10.2 Nota Técnica — Probabilidad de Espiral de Muerte: Precisión del 1.4%

> **QUÉ MIDE EL 1.4%:** El ~1.4% es el riesgo de espiral de muerte durante la TRANSICIÓN B0→B2 — el período más vulnerable del protocolo, cuando el sistema acaba de cruzar K_min ($25M), la R_op está recién formada, y un bear market inmediatamente posterior podría presionar el sistema antes de que tenga suficiente madurez.

**POR QUÉ ES UN RIESGO DE TRANSICIÓN, NO PERMANENTE:**

En B0 la espiral es posible si: precio cae sostenidamente + R_op se agota + volumen colapsa simultáneamente. El Throttle DEFENSIVO extiende 4x la vida de R_op reduciendo el consumo al 25% del rate base.

En B2 consolidado la espiral es estructuralmente muy difícil porque:

1. El Vault no necesita R_op para su operación — en B2 el tramo LP recircula, no quema. La R_op se acumula idle del yield y no es parte del flujo operativo normal.
2. Motor A sigue activo: mientras haya trading, el 35% Asset Layer compra Hard Assets y el 35% LP crea presión compradora sobre $LUKA todos los días.
3. KASH Exit Fee (5%/3%/1%) envía el 100% de las ventas en pánico directamente al Vault Core — el pánico mismo fortalece la reserva.
4. Anti-Whale envía sus fees al Vault Core.
5. Cola diferida ejecuta deflación adicional en la recuperación al 10%/semana.
6. Arbitraje natural: cuando precio de mercado cae muy por debajo del Precio KASH, inversores racionales compran $LUKA. El mercado lo ejecuta solo.

> **EN RESUMEN:** En B2 la recuperación del precio ocurre por inercia estructural sin usar la R_op ni tocar el Vault existente.

**OPCIÓN DE ÚLTIMO RECURSO EN ETAPA 3** (decisión del DAO, no automática):

Si en Etapa 3 el precio de mercado cayera sistemáticamente por debajo del Precio KASH, el DAO tiene la opción de activar redención directa al Precio KASH: los holders entregan $LUKA al protocolo y reciben USDC del Vault a valor justo. Los $LUKA entregados se bloquean permanentemente, reduciendo el supply efectivo y subiendo el Precio KASH restante. Esta decisión requiere aprobación de gobernanza — el Vault no se toca de forma automática bajo ninguna circunstancia.

### 10.3 Hitos de Inversión

| Hito | Métrica | Plazo (Simulación Base) | Plazo (Con MM activos) |
|---|---|---|---|
| Hito 1 — K_min | $25M en Vault Core. Motor B2 activado | <= 24 meses | <= 12 meses |
| Hito 2 — K_activación | $30M O 12 meses. KASH Lock activo | Mes 12–18 | Mes 6–12 |
| Hito 3 — Distribución | Distribución líquida perpetua desde mes 13 (30% fees KASH Sociedad) | Mes 13+ | Mes 13+ |
| Hito 4 — Escala Total | $100M en Vault. ENZ activa. Motor C activo | <= 36 meses | <= 24 meses |

---

## 11 ELEVATOR SPEECHES

### Mom Speech · Elevator Pitch — Para Todos los Públicos

Los siguientes discursos explican LUKASH de forma simple, directa y persuasiva. Incorporan datos reales verificados de 2025.

`[Cambio v4.3: Rebrand — Aura reemplaza "Jaguar Score". Toda la terminología actualizada]`

### Mom Speech — Máximo 4 minutos

> Para inversores y no inversores sin conocimiento técnico. Memorizable, emocional y anclada en datos reales.

Permítame hacerle una pregunta. ¿Sabía usted que 7 de cada 10 adultos en América Latina ya tienen una cuenta digital? El Banco Mundial lo confirmó en 2025. Pero aquí está el problema: solo 3 de cada 10 accede a instrumentos financieros reales que les permiten crecer. Los otros 4 tienen la cuenta — pero la puerta está cerrada por dentro.

Mientras tanto, el sistema financiero global está migrando a blockchain. BlackRock ya tiene $2,900 millones en activos tokenizados on-chain. Franklin Templeton. Siemens. Apollo. Las instituciones más grandes del planeta están construyendo las finanzas del futuro sobre esta tecnología ahora mismo. Y el mercado de activos tokenizados creció de $5,000 millones a $24,000 millones en solo tres años. Boston Consulting Group proyecta $16 billones para 2030.

Nosotros creamos LUKASH. La pregunta que nos hicimos fue simple: ¿y si los 260 millones de latinoamericanos excluidos pudieran tener acceso a esa misma infraestructura financiera del futuro, hoy, desde su teléfono?

LUKASH es eso. Cada transacción construye automáticamente una reserva de activos reales. Bitcoin nativo. Dólares digitales. Activos sólidos. Guardados en una bóveda digital que cualquier persona en el mundo puede verificar en tiempo real. No promesas. No papelitos. Activos reales.

¿Cómo funciona en la práctica?

- Cada transacción genera un fee pequeño. Ese fee no se lo queda un banco. Se divide automáticamente: una parte construye la reserva, otra reduce la cantidad de tokens en circulación — lo que hace que el token valga más con el tiempo — y el resto premia a quienes participan activamente.

- Tienes una Aura: tu historial financiero digital. On-chain, transparente, tuyo. Nadie te lo puede quitar. Mientras más participas y aprendes, más puertas se abren — incluyendo acceso a instrumentos de inversión que antes solo tenían los grandes capitales.

- No necesitas saber nada de blockchain. LUKAI — nuestra inteligencia artificial — habla contigo en lenguaje normal y gestiona todo por ti.

- Puedes participar solo o en grupo con tu comunidad — las Manadas. Es la natillera o la tanda del siglo XXI: la misma lógica de confianza de siempre, ahora con contratos inteligentes y acceso a la economía global.

¿Para quién es LUKASH?

- Para el joven de 18 años que quiere construir patrimonio desde cero, sin capital inicial grande.
- Para el profesional de 35 que quiere proteger sus ahorros de la inflación y acceder a instrumentos reales.
- Para quien envía dinero a su familia en otro país y paga comisiones abusivas.
- Para el emprendedor que quiere acceso a crédito colectivo sin depender de un banco tradicional.

¿Y los números? Las simulaciones Monte Carlo — 200 escenarios distintos por cada campaña de marketing — muestran que la reserva del protocolo crece entre $90 millones y $1,900 millones en 5 años dependiendo del escenario. El riesgo de colapso total es del 0.0%. No lo decimos nosotros — lo dice el modelo matemático que replica exactamente los contratos desplegados en blockchain.

LUKASH no es una apuesta especulativa. Es la infraestructura de las finanzas del futuro, construida para Latinoamérica, disponible hoy desde tu teléfono.

Las finanzas del futuro, disponibles hoy. ¿Quieres ser parte de los primeros en construirlas?

### Elevator Mom Speech — Máximo 1 minuto 10 segundos

> Versión de ascensor — Simple, directa, memorable. Para cualquier público en cualquier momento.

El sistema financiero global está migrando a blockchain. BlackRock ya tiene $2,900 millones de dólares tokenizados on-chain. Franklin Templeton. Siemens. Las instituciones más grandes del planeta lo están haciendo ahora.

El problema: 7 de cada 10 latinoamericanos tiene una cuenta digital, pero solo 3 de cada 10 accede a instrumentos financieros reales que les permiten crecer. Los otros 4 tienen la puerta — pero no pueden entrar.

LUKASH es la llave para esos 260 millones de personas.

Es una plataforma digital donde cada transacción construye automáticamente una reserva real de activos — Bitcoin, dólares digitales — que respalda el valor de tu dinero y crece con el tiempo. Tiene una inteligencia artificial que habla contigo en lenguaje normal. No necesitas saber nada técnico.

Puedes participar solo o en grupo — las Manadas: la natillera o la tanda del siglo XXI con contratos inteligentes. Las matemáticas dicen que el riesgo de colapso total es cero. En 600 escenarios simulados con un modelo que replica exactamente los contratos.

No estamos creando otro token especulativo. Estamos construyendo las finanzas del futuro, disponibles hoy para toda Latinoamérica.

---

## 12 AURA — Sistema Formal de Reputación Financiera On-Chain

`[Cambio v4.3: Rebrand total — ADR-002, ADR-005, ADR-017. "Jaguar Score" pasa a llamarse "Aura". De 4 niveles a 7 niveles. Niveles en español, son marca y no se traducen. ADR-023: Capa 0 eliminada. ADR-024: Tótem tiers + Avatar NFT. ADR-026: educación multi-track]`

Aura es el currículum financiero digital del usuario. Es el mecanismo de retención más poderoso del protocolo: el usuario no abandona una reputación construida durante años. LUKAI v4.1 lo calibra en tiempo real según la participación activa en el ecosistema.

> **DEFINICIÓN:** Número acumulativo on-chain. Refleja la participación financiera del usuario en LUKASH. No reversible excepto por inactividad prolongada. No transferible. Vinculado a wallet. Decae 2%/semana adicional después de 90 días de inactividad.

### 12.1 Parámetros de Calibración

| Acción del Usuario | Puntos | Notas |
|---|---|---|
| Holding activo de Tótem Universal | 10 pts/semana | Máximo incentivo de holdeo del instrumento premium |
| Holding activo de Tótem Nativo (Capa 1, 1.5%) o Tótem Estándar (Capa 3B) | 5 pts/semana | Reconoce holding comprometido en cualquier instrumento del protocolo |
| Participación en Manada activa (Capa 2) | 15 pts/evento | Por cada transacción completada en Manada |
| Apuesta Bet & Win completada (Capa 2) | 8 pts/evento | Por cada apuesta liquidada, independientemente del resultado |
| Acceso a evento o marketplace (Capa 2) | 5 pts/evento | Por cada uso de servicios sociales en la App |
| Completar misión educativa en Jungle Arena | 20 pts/misión | Máxima ponderación. Premia el aprendizaje financiero real |
| Completar lección de Escuela Financiera | 5 pts/lección | 4 tracks paralelos: finanzas personales, ciudadano, cripto, LUKASH |
| Duelo de Tótems (participación) | 12 pts/duelo | Por cada duelo completado. Incentiva cohesión grupal |
| Predicción Relámpago (streak activa) | 3 pts/día | Streak diaria, multiplicador acumulativo |
| Staking activo $LUKA via Tótem (período mínimo 7d) | 3 pts/día | Requiere Tótem Nativo — no existe staking sin Tótem |
| LP Comprometido activo (30d / 90d / 365d) | 5 / 10 / 20 pts/semana | Escalonado por nivel de lock. Mayor compromiso = mayor Aura semanal |

### 12.2 Niveles y Beneficios — El Camino del Rugido

| Nivel | Puntos | Badge on-chain | Beneficios desbloqueados |
|---|---|---|---|
| **Cachorro** | 0–499 | Cachorro | Acceso básico a servicios Motor D Capa 2. Tótem Nativo Bronce (1.5%, desbloqueado por misión educativa). Participación en Manadas estándar |
| **Rastreador** | 500–1,499 | Rastreador | Acceso a Tótem Universal. Entrada a instrumentos DeFi Capa 1. Puede mintear Avatar como NFT completo (milestone). Gate mínimo para instrumentos premium |
| **Cazador** | 1,500–2,999 | Cazador | Puede crear Sellos de Manada (líder). Voto en parámetros Motor D (DAO activo). Acceso prioritario a nuevos instrumentos Tótem |
| **Alfa** | 3,000–4,999 | Alfa | LP 90d con +12% rewards. Visibilidad en ranking de Manadas. Acceso anticipado Capas 3A/3B |
| **Emperador** | 5,000–9,999 | Emperador | Gobernanza premium. LP Fundador 365d +20% rewards. Acceso a instrumentos RWA y T-bills tokenizados en Fase 3 |
| **Shamán** | 10,000–24,999 | Shamán | Rol de mentor en Jungle Arena. Curaduría comunitaria de instrumentos. Propuesta directa al DAO |
| **Titán** | 25,000+ | Titán | Exención Anti-Whale + KASH Exit Fee. Máximo nivel del protocolo. Voz ponderada en gobernanza DAO |

> **DECAIMIENTO:** 2%/semana adicional tras 90 días de inactividad. Se detiene al retomar actividad. Los niveles alcanzados no bajan por decaimiento — el umbral de nivel usa el Score máximo histórico, no el Score actual.
>
> PENDIENTE FASE APP — Aura — ajuste post-datos reales: Los valores actuales son los de lanzamiento. Ajustables por gobernanza con Timelock 48h en Etapa 2A basado en datos reales de uso y retención.

---

## 13 KASH SHIELD — Sistema de Seguridad del Protocolo

`[Cambio v4.3: Sección nueva. Consolida KASH Shield completo — ADR-012, ADR-015, ADR-022, ADR-027. Anti-Whale por % del pool (no supply). Tridente → Guardian de Pausa 2-de-3. Totem Guard como seguro individual]`

### 13.1 Componentes del KASH Shield

| Componente | Mecánica | Parámetros |
|---|---|---|
| **Anti-Whale** | Fee escalonado por % del pool de liquidez. Solo ventas/transferencias, NO compras. 100% al Vault Core | Tiers 3/6/10% sobre el excedente del umbral. Exención: Titán (Aura ≥25000), LP Fundador, MM registrado en Guardian, staking activo via Tótem, swaps internos Motor D |
| **KASH Exit Fee** | Fee en ventas de pánico. 100% al Vault Core | Et.1: 5%, Et.2: 3%, Et.3: 1%. Activación: precio <0.7xEMA30 AND venta >0.3% supply/hora. Exime holders comprometidos |
| **Circuit Breaker** | 3 niveles de protección de liquidez | Protege el pool contra eventos de liquidación masiva |
| **Guardian de Pausa** (ADR-022) | Solo puede: pausar el protocolo + vetar operaciones en timelock. NO puede mover fondos ni modificar parámetros | 2-de-3 (antes 3-de-3). Nace INACTIVO. Sunset automático en Etapa 3 (K>$50M) o por voto DAO |
| **Totem Guard** (ADR-027) | Seguro paramétrico opt-in para Tótems. Micro-prima en $LUKA proporcional al Tótem. Payout automático por oráculo ante exploit | Cascada: (1) Pool Totem Guard → (2) Póliza externa (O&M) → La Reserva NO se toca. Máx 1 payout/12m. Primas moduladas por Aura |
| **Póliza externa O&M** (ADR-027) | Seguro de activos digitales contratado con proveedor externo. Financiado por % del 15% O&M | Cubre Vault contra exploits/hacks. Actúa como segunda línea después del Pool Totem Guard |
| **Seguro Anti-Exploit del Vault** | Cobertura hasta 5% del valor total del Vault. Máximo 1 evento cada 12 meses | Herramienta de última instancia disponible solo para el DAO en Etapa 4. NO se activa automáticamente pre-DAO — la Reserva Sagrada solo crece |
| **Timelock 48h** | En parámetros críticos: K_min, fees, thresholds Throttle, composición Vault | Previene cambios unilaterales instantáneos |
| **Token-2022 Transfer Hook** | Anti-Whale + KASH Exit Fee se aplican en cada transferencia on-chain | Imposible de evadir vendiendo en otro DEX. El Transfer Hook verifica exenciones automáticamente |

### 13.2 Exenciones Canónicas (Anti-Whale + Exit Fee)

- Swaps internos del Motor D (operaciones dentro del protocolo)
- Staking activo via Tótem (holders comprometidos)
- LP Comprometido en lock activo (proveen liquidez, no la sacan)
- LP Fundador 365d (lock máximo, incentivo estructural)
- Market Makers registrados en el Guardian de Pausa (registro explícito on-chain, aprobado por Guardian 2-de-3)
- Nivel Titán de Aura (Aura ≥25,000)
- KOLs NO tienen exención explícita — su mecanismo de alineación es el vesting on-chain (ADR-011)

---

## 14 IDENTIDAD DEL FUNDADOR

`[Cambio v4.3: Sección nueva — ADR-021. Pseudónimo conocido]`

| Nivel de Confianza | Exposición | Detalle |
|---|---|---|
| Inversores / Legal / Auditoría | Identidad completa | KYC, contrato, comunicación directa |
| Equipo / Socios | Nombre real | Comunicación directa, relación profesional |
| Comunidad pública | Alias consistente + voz | AMAs sin cámara, historial verificable del proyecto |
| Prensa / Redes | El proyecto y su narrativa | No la persona. El proyecto habla por sí mismo |

> Razones: seguridad personal en LatAm, flexibilidad regulatoria pre-framing legal, protección ante phishing/extorsión dirigida. Se re-evalúa el doxx público en mes 6-12 según tracción y contexto.

---

## 15 ROADMAP KASH SOCIEDAD — Rondas de Inversión

`[Cambio v4.3: Sección nueva — ADR-020. Rondas escalonadas con valoración creciente]`

| Ronda | Timing | Inversores Objetivo | Capital | Participación KASH Sociedad | Valoración Implícita |
|---|---|---|---|---|---|
| Seed | Pre-TGE | Ángeles cripto-nativos | $500K | 5-8% | ~$6-10M |
| Estratégica | Mes 6-12 (post-tracción) | Fintechs LatAm, cooperativas, remesadoras | $1-2M | 5-8% | ~$15-25M |

**Distribución resultante del KASH Sociedad:**

| Participante | % del KASH Sociedad |
|---|---|
| Fundador | ~17% |
| Inversores Seed | 5-8% |
| Inversores Estratégicos | 5-8% |
| Reserva DAO / Futuro | ~6-13% |

> **Regla estricta (ADR-019):** Tokens vesteados = pago por servicios (MM/KOLs). KASH Sociedad = solo para capital/inversores a largo plazo. NUNCA mezclar.

---

## CUADRO RESUMEN — BUSINESS MODEL CANVAS LUKASH v4.3

Vista consolidada. Cada celda referencia el bloque detallado correspondiente.

| **08 ASOCIACIONES CLAVE** | **06 ACTIVIDADES CLAVE** | **01 PROPUESTA DE VALOR** | **04 RELACIONES CLIENTES** | **02 SEGMENTOS** |
|---|---|---|---|---|
| MM contingente (ADR-019) | Orquestación B0/B2 | Estándar KASH: piso de valor en Hard Assets auditado on-chain. Contratos v10 verificados en devnet | Et.1: Mentor–Aprendiz. LUKAI como tutor gamificado | Segmento A (12–25 años): Jungle Arena, Manadas, Bet & Win |
| KOLs KASH Lock (tokens vesteados) | Throttle Dual (lógica invertida Et.3) | Las finanzas del futuro: RWA de $5B→$24B en 2025 (+380%). BCG: $16 billones para 2030 | Et.2: Compañero de Manada | Segmento B (25–45 años): LUKAI, Tótems, Vault, acceso RWA |
| Auditor externo a cotizar | Routing Tótem Motor D 4 capas | LUKASH es la puerta de entrada de 260M latinoamericanos excluidos | Et.3: Socio del Protocolo | Geografía: Colombia+México (P1) · El Salvador+CA (P2) · Argentina+Perú+Chile (P2) · Brasil PT (P3) · España remesas (P4) |
| Inversores Seed ($500K) + Estratégicos ($1-2M) | Gestión Vault contra-cíclica | Experiencia gamificada: Aura como RPG financiero. Tótems con identidad | Costo de Salida Psicológico (Aura) | |
| Pyth + Switchboard + Jupiter | Curaduría instrumentos Tótem | Manadas = natilleras del siglo XXI | | |
| Kamino / Marginfi / Sanctum | Calibración Aura | $LUKA como identidad cultural: pagar es un acto de pertenencia | | |
| Hub: El Salvador CNAD | | 4 Motores: El Cazador (A) + El Motor Interno (B) + El de Escala (C) + El Alma (D) | | |
| Protocolos RWA (Fase 3) | **07 RECURSOS CLAVE** | | **03 CANALES** | |
| Neobancos LatAm (Fase 3) | Vault KASH Core (Hard Assets) | | Jungle Arena | |
| | LUKAI v4.1 (orquestador + IA) | | App LUKASH + LUKAI v4.1 | |
| | Sistema Tótem (instrumento financiero) | | Dashboard (La Bóveda / El Fuego / La Cola) | |
| | Aura (capital social, 7 niveles) | | KOLs KASH Lock | |
| | Smart Contracts Token-2022 (Solana nativo) | | Manadas (CAC ~$1/miembro adicional) | |
| | Pool de Liquidez LP (Meteora) | | DEX Meteora / Jupiter + CEX (mes 4+) | |

| **09 ESTRUCTURA DE COSTOS** | **05 FUENTES DE INGRESOS** |
|---|---|
| Fase 1: $500K (Tech 35% / Liquidez 20% / Marketing+MM 30% / Legal 10% / Reserva 5%) | Motor A: 4%/2.5%WL (Et.1) → 2.5%/1.5%WL (Et.2) \| SOL \| EL CAZADOR |
| Fase 2: $2M (400–500K usuarios) | Motor B: 2.5%/1.5%WL \| $LUKA \| Deflación B0 → Recirculación B2 \| EL MOTOR INTERNO. R_op no interviene |
| Fase 3: $5M (RWA+Licencias+LUKASH Pay+Brasil) | Motor C: 0.5% \| USDC \| Pagos masivos Fase 3 \| EL MOTOR DE ESCALA |
| CAC: $5–$9 (Et.1) → $3–$5 (Et.2 viral) → $1 (orgánico objetivo, ref. Nubank) | Motor D: Capa 0 (0%) · Capa 1 (1.5%) · Capa 2 (3%/3.5%) · Capa 3A (1.5%) · Capa 3B (2%) \| EL ALMA |
| Costo/tx: $0.0003 vs $20–$65 SWIFT. Soporte: $0 (LUKAI 24/7). Escala sin VCs | Yield Vault: LST 6–8% + USDC lending 4–8% → regla 70/30. Distribución perpetua a socios desde mes 13 |
| Equipo/Fundador: 2% supply vesteado (cliff 12m + 48m lineal) | Rondas: Seed $500K/5-8% · Estratégica $1-2M/5-8% |

---

## ESTADO DEL MODELO — VALIDACIÓN Y PENDIENTES

### Completamente Definidos en v4.3

- Propuesta de valor con 8 secciones: contexto RWA/blockchain 2025, Estándar KASH, experiencia gamificada, identidad cultural $LUKA, Manadas como economía comunitaria, 4 motores con identidad, posicionamiento competitivo, filosofía.
- Fuentes de ingreso: Motor D en 4 capas, flujos completos por motor, sinergia entre motores, proyecciones por etapa. Corrección: R_op no interviene en el flujo normal del Motor B0 ni B2.
- CAC corregido con tres niveles diferenciados basados en datos reales de Nubank 2024.
- Inversión Fase 1 redistribuida: Tech 35% / Liquidez 20% / Marketing+MM 30% / Legal 10% / Reserva 5% (ADR-018).
- Geografía: todo LATAM + Brasil + España con mapa de prioridades y CAC estimado por mercado.
- Probabilidad de Espiral de Muerte: ~1.4% es riesgo de transición B0→B2 (modelo v3.1, pendiente reconciliar con v4.3). En B2 consolidado <0.5% por inercia estructural.
- Elevator Speeches con datos reales verificados: Global Findex 2025, Credicorp IIF 2024, RWA tokenización 2025.
- Aura con 7 niveles, parámetros formales de calibración y beneficios diferenciados.
- KASH Shield completo: Anti-Whale por % del pool, KASH Exit Fee, Circuit Breaker, Seguro Anti-Exploit, Guardian de Pausa 2-de-3 (ADR-022), Totem Guard (ADR-027), Timelock 48h, Token-2022 Transfer Hook.
- Estrategia MM/KOL en 3 fases (ADR-019).
- Rondas de inversión escalonadas con roadmap KASH Sociedad (ADR-020).
- Identidad del fundador definida (ADR-021).
- Supply con fila explícita Equipo/Fundador 2% (ADR-014).
- Rebrand completo: terminología "jaguar" eliminada de todo vocabulario activo (ADR-017).

### Pendientes de Definición — Fase App

- PENDIENTE FASE APP — Mercado secundario Tótems: Fee de venta, herencia de modo de rendimiento, actualización de Aura al transferir. BLOQUEANTE para contrato Tótem.
- PENDIENTE FASE APP — Frecuencia de distribución Modo B: ¿Semanal o mensual? ¿Fijo por instrumento? BLOQUEANTE para contrato Tótem.
- PENDIENTE FASE APP — Jungle Arena — mecánicas de juego detalladas: Niveles completos, misiones concretas, Bet & Win, economía de recompensas sostenible.
- PENDIENTE FASE APP — Manadas — parámetros de producto: Tipos, límites de capital, gobernanza interna, árbitro LUKAI para disputas.
- PENDIENTE FASE APP — Go-to-market local por país: Colombia, México, El Salvador, Argentina, Brasil (portugués). Regulación específica y partnerships locales.
- PENDIENTE FASE APP — Yield sharing DeFi externo Capa 3: Modelo de gestora patrimonial descentralizada para Etapa 3.

---

> LUKASH no es simplemente una alternativa a la banca tradicional. Es la puerta de entrada de 260 millones de latinoamericanos a la infraestructura financiera del futuro — la misma infraestructura que BlackRock, Franklin Templeton y los grandes gestores del mundo ya están construyendo sobre blockchain.
>
> **Las finanzas del futuro, disponibles hoy para toda Latinoamérica.**

---

**LUKASH PROTOCOL BMC v4.3 — CONFIDENCIAL · USO EXCLUSIVO PARA INVERSORES**

Contratos v10 verificados en Solana devnet · ADRs 001-027 integrados · Agosto 2026

Ing. Sebastián Botero Pabón — Arquitecto del Protocolo
