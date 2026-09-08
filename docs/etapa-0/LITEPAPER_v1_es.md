# LUKASH Protocol — Litepaper v1.0 (Español)

> September 2026 | Data Room Document | Confidential
> Contact: Kash Sensei · kash.sensei.sol@gmail.com
> GitHub: github.com/doblemasterinc/lukash-protocol (access on request)
> Devnet Program: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`

---

## 1. Resumen Ejecutivo

LUKASH es infraestructura monetaria optimizada para América Latina, construida sobre Solana.

$LUKA es una moneda transaccional deflacionaria respaldada por el **Vault KASH Core** — una reserva creciente de activos duros (BTC, SOL, LST, USDC) que es 100% on-chain y auditable 24/7. Cada transacción en el ecosistema alimenta el Vault, quema supply y recompensa a los participantes.

A diferencia de los bancos centrales tradicionales que emiten moneda contra deuda soberana (inflacionarios por diseño), LUKASH construye su reserva a partir del volumen real de transacciones y deflacta el supply hacia un piso fijo de 3,300 millones de tokens. El resultado: una moneda transaccional con un **piso de precio verificable** (P_KASH = Vault / Supply), gobernanza transparente y mecanismos de seguridad integrados que previenen las fallas catastróficas comunes en DeFi.

El protocolo apunta a 260 millones de latinoamericanos que tienen cuentas bancarias digitales pero cero acceso a la infraestructura financiera institucional (RWA, blockchain, rendimiento sobre activos duros) que el mundo desarrollado ya utiliza.

**Estado actual:** Smart contracts v10.2 desplegados en Solana devnet (~2,100 líneas Anchor/Rust). Monte Carlo v4.3 validado en 7 simulaciones (600+ escenarios). 0.0% de riesgo de ruina en escenarios base y agresivos. Análisis ROI SIM 6: retorno de 15x para inversionistas seed en escenario base. Landing page en producción. Pre-TGE.

---

## 2. El Problema

América Latina tiene una paradoja financiera: la adopción digital es alta (200M+ cuentas bancarias, población creciente mobile-first) pero el acceso a instrumentos financieros reales es casi nulo. La persona promedio menor de 45 años en Colombia, México o Brasil:

- **No puede** acceder a exposición en activos duros (BTC, SOL) sin navegar exchanges opacos
- **No puede** generar rendimiento sobre sus ahorros sin confiar en intermediarios opacos
- **No puede** construir una reputación financiera verificable entre plataformas
- **Pierde** del 5 al 15% de poder adquisitivo anual por inflación sin cobertura

Los productos cripto existentes le fallan a esta audiencia: hablan un lenguaje que nadie entiende, están diseñados para traders (no para ahorradores), y no ofrecen protección integrada contra la volatilidad que ahuyenta a los usuarios primerizos.

El problema no es falta de dinero — es falta de **acceso, confianza y transparencia**.

---

## 3. La Solución — Un Banco Central Optimizado

LUKASH replica el modelo de banco central — una moneda respaldada por una reserva administrada — corrigiendo sus defectos fundamentales:

| Dimensión | Banco Central Tradicional | Protocolo LUKASH |
|---|---|---|
| **Emisión** | Moneda emitida contra deuda soberana | Supply fijo (10B), deflacionario hasta piso de 3.3B |
| **Reserva** | Construida sobre deuda y política monetaria | Construida por transacciones reales (35% de todos los fees → Vault) |
| **Tendencia** | Inflacionario por diseño (2-10%/año, peor en LatAm) | Deflacionario: las quemas reducen supply, el Vault crece |
| **Piso de precio** | Ninguno (la moneda puede devaluarse sin límite) | P_KASH = Vault / Supply (auditable on-chain) |
| **Transparencia** | Opaca (decisiones de comités cerrados) | Smart contract auditable, Vault verificable 24/7 |
| **Gobernanza** | Centralizada en institución gubernamental | Guardian 2-of-3 → DAO (Etapa 4) |

Esto no es "otro protocolo DeFi." Es la versión optimizada de la política monetaria, democratizada para 650 millones de latinoamericanos que viven la devaluación de su moneda como una experiencia diaria.

---

## 4. Arquitectura del Protocolo

### 4.1 El Átomo — Motor Universal de Fees

Cada operación en el ecosistema LUKASH — sin importar el motor, capa o activo — distribuye fees atómicamente en una sola transacción on-chain:

| Bucket | Asignación | Propósito |
|---|---|---|
| Vault / Capa de Activos | 35% | Hace crecer la reserva de activos duros |
| Quema / LP | 35% | Quema supply de $LUKA + provee liquidez |
| O&M | 15% | Gastos operativos, compensación del equipo |
| Staking | 15% | Recompensas para holders de Tótem |

Esta distribución es inmutable y verificada por invariantes on-chain.

### 4.2 Los Cuatro Motores

| Motor | Nombre | Activo | Fee | Etapa | Propósito |
|---|---|---|---|---|---|
| **A** | El Cazador | SOL | 4% (2.5% WL) | TGE (Etapa 1) | Trading en DEX — principal generador de volumen |
| **B** | El Motor Interno | $LUKA | 2.5% (1.5% WL) | Etapa 2 | Transacciones in-app. B0=quema directa (K<$25M), B2=recirculación (K≥$25M) |
| **C** | El de Escala | USDC | 0.5% | Etapa 3 | Pagos masivos, vías fiat |
| **D** | El Alma | Multi | 0-3.5% | Gradual | Tótems, gaming, Manadas, instrumentos financieros |

Motor A es el motor económico. Las simulaciones Monte Carlo confirman que el volumen de trading en DEX es el factor #1 de la salud del protocolo, con una oscilación de sensibilidad de $1.29B en el valor del Vault a 5 años.

### 4.3 Vault KASH Core

Una reserva de activos duros que **solo crece, nunca es liquidable**, y determina el piso de precio.

| Activo | Asignación | Justificación |
|---|---|---|
| cBTC (wrapped) | 35% | Activo más duro, reserva de valor a largo plazo |
| SOL | 15% | Activo nativo de la cadena, rendimiento por staking |
| SOL/LST (JitoSOL, mSOL) | 20% | Liquid staking, rendimiento adicional |
| USDC (reserva) | 25% | Estabilidad, liquidez inmediata |
| USDC (lending) | 5% | Rendimiento vía Kamino/Marginfi |

100% Solana-nativo. Cero riesgo de bridge. Oracles: Pyth Network + Switchboard (redundancia, umbral de desviación 2%).

**Composición dinámica por régimen de mercado:** Un módulo contra-cíclico on-chain (LUKAI) ajusta la distribución entre activos volátiles y estables basándose en señales BTC EMA30/EMA90:
- BULL: 40% volátil / 60% USDC
- NEUTRAL: 75% volátil / 25% USDC
- BEAR: 70% volátil / 30% USDC (acumular activos baratos)

Mecanismo de seguridad: si el keeper no actualiza en 48h, el régimen se establece en NEUTRAL por defecto.

### 4.4 Mecanismo de Deflación

Supply: 10,000,000,000 → piso 3,300,000,000 (quema de 6.7B tokens).

- **Quema directa:** 35% de cada fee (bucket Quema/LP) destruye $LUKA vía CPI `token::burn` — verificado en devnet
- **Cola de quema diferida:** Cuando se alcanza el tope diario de quema (1% del supply), el excedente se encola para ejecución posterior
- **Throttle dinámico:** La cola se drena a diferentes velocidades según las condiciones de mercado (ACCEL 25%/sem, NORMAL 10%, CONSERVATIVE 5%, DEFENSIVE 2%)
- **Parada total ENZ:** Al llegar a 3.3B de supply, TODA la maquinaria de quema se detiene permanentemente. La cola se congela. El supply nunca baja del piso.

Post-ENZ (Etapa 3): $LUKA se convierte en un activo de supply fijo con recompensas perpetuas del Vault — similar a una acción preferente con dividendos reales.

### 4.5 KASH Shield — Seguridad Integrada

| Mecanismo | Descripción |
|---|---|
| **Anti-Whale** | Fee escalonado en ventas grandes (>1% del pool LP): 3%/6%/10% sobre el excedente. Solo ventas, no compras. |
| **KASH Exit Fee** | Doble gatillo: precio < 0.7×EMA30 Y presión de venta > 0.3% supply/hora. Fee: 5%/3%/1% por etapa. |
| **Circuit Breaker** | Pausa automática de 24h si el Vault cae >10% en ventana de 1h. Cancelable por Guardian 2-of-3. |
| **Guardian de Pausa** | Multisig 2-of-3 (antes 3-of-3 "Tridente"). SOLO puede pausar el protocolo — no puede mover fondos. Auto-eliminación en Etapa 3. |
| **Token-2022 Transfer Hook** | Cada transferencia de $LUKA pasa por el KASH Shield — imposible de evadir operando en otro DEX. |
| **Timelock 48h** | Todos los cambios a parámetros críticos requieren 48h de espera. |
| **ASU (Totem Guard)** | Seguro paramétrico para holders de Tótem. Pago automático ante exploit confirmado. Respaldado por póliza de reaseguro externa. |

Exenciones de Anti-Whale/KASH Exit Fee: swaps internos de Motor D, staking activo, LP bloqueado, LP Fundador (lock 365 días), Market Makers registrados (aprobados por Guardian), nivel Aura Titán (≥25,000).

---

## 5. Tokenomics

### 5.1 Distribución del Supply

| Bucket | % | Tokens | Vesting | Notas |
|---|---|---|---|---|
| **Público (Fair-Launch)** | ≥40% | ≥4,000M | Circulando desde TGE | Fair-launch, sin concentración de pre-mine |
| **Seed (opcional)** | ≤5% | ≤500M | SAFE + token warrant, vesting on-chain | Solo si los grants no cubren el runway. Lo no vendido pasa a Público |
| **Vault Sociedad** | 30% | 3,000M | KASH Lock: se desbloquea a $30M en Vault o 12 meses (lo que ocurra primero). Líquido desde mes 13, lineal 48 meses | Equity del fundador — NO se distribuye a holders |
| **Marketing / CEX** | 8% | 800M | Vested según acuerdo | KOLs pagados en tokens con vesting, nunca efectivo |
| **Recompensas de Staking** | 10% | 1,000M | Distribuido a holders de Tótem vía Motor D | Pool de recompensas perpetuas |
| **Liquidez Inicial** | 5% | 500M | LP bloqueado permanentemente | Pool Meteora $LUKA/SOL |
| **Equipo / Fundador** | 2% | 200M | Cliff 12 meses + lineal 48 meses, on-chain pre-TGE | Alineación + gobernanza + upside del token |

**Total: 100% (10,000,000,000 $LUKA)**

### 5.2 Economía del Fundador — Tres Capas

La estructura de incentivos del fundador está diseñada para alineación, no para extracción:

1. **O&M (15% de todos los fees):** Compensación operativa por operar el protocolo. Ingreso a corto plazo.
2. **Equipo/Fundador (2% del supply):** Upside del token + gobernanza DAO. Largo plazo, vesting completo (cliff 12 meses + 48 meses lineal). Publicado on-chain antes del TGE.
3. **Vault Sociedad (≥10% del bucket del 30%):** Equity en el flujo de fees del protocolo. Riqueza a largo plazo. Sujeto a KASH Lock.

Un fundador con 0% de asignación de tokens señala "sin piel en el juego" — incómodo para ángeles y aceleradoras. El 2% es pequeño, transparente y con vesting agresivo.

### 5.3 Vault Sociedad — Lo Que Compran los Inversionistas

El Vault Sociedad representa el **30% de todos los fees del protocolo** — un flujo de ingresos creciente, no un pool fijo. Es el equivalente al equity del protocolo.

**Lo que obtiene un inversionista seed:**
- Un porcentaje del Vault Sociedad, negociado por operación
- Estructurado como SAFE + token warrant con vesting on-chain
- Sujeto a KASH Lock (se desbloquea a $30M en Vault KASH Core o 12 meses, lo que ocurra primero)
- Líquido desde el mes 13, lineal en 48 meses

**Valuación implícita:** $500K seed → ~$6-10M pre-money. Esto es razonable para un protocolo con:
- Deployment en devnet activo (no vaporware)
- Economía validada (Monte Carlo, no cálculos en servilleta)
- Fundador solo con desarrollo aumentado por IA (estructura de capital liviana)

**SIM 6 — ROI del Inversionista (Monte Carlo, escenario base):**
- 6% de participación en Sociedad → **$7.5M en 5 años (15x ROI)**
- IRR: 147% | Recuperación: ~14 meses
- Escenario conservador: 5x ROI | Escenario agresivo: 57x ROI

**Lo que un inversionista seed NO obtiene:**
- Control sobre el Vault KASH Core (solo crece, nunca es liquidable)
- Asignación del supply de tokens (el fair-launch se preserva)
- Poder de veto sobre parámetros del protocolo (gobernado por Timelock + Guardian + DAO)

La distribución interna del 30% de Sociedad (fundador, seed, operaciones, reserva) está estructurada según ADR-030 y se comparte bajo NDA durante el due diligence. Tope de asignación externa: 20%.

---

## 6. Simulaciones Monte Carlo — Números Honestos

Todas las simulaciones usan un **motor fiel al contrato**: la simulación en Python replica la aritmética exacta de enteros de `lib.rs` línea por línea. Esto valida el CÓDIGO, no solo el diseño.

**Resultados MC v4.3 (200 iteraciones × 3 campañas × 5 años):**

### 6.1 Resultados Principales (SIM 1)

| Escenario | Volumen Diario | Mediana Vault (5 años) | Riesgo de Espiral | Activación B2 (mediana) |
|---|---|---|---|---|
| **Conservador** | ~$100K-500K | $85M | **42%** | Día 1,034 |
| **Base** | ~$500K-2M | $434M | 0.0% | Día 448 |
| **Agresivo** | ~$2M-10M | $1,870M | 0.0% | Día 195 |

**La verdad honesta:** En el escenario conservador (bajo volumen, comunidad débil), hay un 42% de probabilidad de espiral de muerte. Por eso **el volumen es el riesgo existencial** y por eso el presupuesto asigna el 30% ($150K) a marketing — y por eso ADR-030 exige un Market Maker desde el día 1 del TGE (reduciendo la espiral conservadora de 36% a 2.5% según SIM 5). El diseño económico del protocolo es sólido — pero necesita volumen para funcionar.

### 6.2 Análisis de Sensibilidad (SIM 2)

| Factor | Impacto en Vault (5 años) |
|---|---|
| Volumen de Trading | **$1,309M** (factor #1) |
| Tasa de Fee Motor A | $283M |
| Rendimiento del Vault | $7M |
| Día de Lanzamiento de la App | $2M |
| Umbral K_min | $0 (neutro) |

El volumen domina todo lo demás por 4.6x. Esto confirma: el éxito del protocolo depende de la comunidad y la adopción, no del ajuste de parámetros.

### 6.3 Pruebas de Estrés (SIM 3)

| Escenario | Impacto en Vault |
|---|---|
| Crash BTC -80% | -3.8% (resiliente) |
| Exploit 15% del Vault | -0.3% (KASH Shield absorbe) |
| Retiro de LP | -0.8% |
| Volumen Motor A -70% permanente | **-68%** (única amenaza real) |

El protocolo sobrevive a crashes de mercado. La única amenaza existencial es la pérdida sostenida de volumen de trading — lo cual es un riesgo de adopción, no un riesgo del protocolo.

### 6.4 Análisis de Market Maker (SIM 5)

| Métrica | Sin MM | Con MM |
|---|---|---|
| Vault | Línea base | +11-19% |
| Precio | Línea base | -10-25% (dilución por inventario del MM) |
| Espiral conservadora | 36% | 2.5% |
| Activación B2 (BASE) | Día 493 | Día 97 |

El MM mejora dramáticamente la supervivencia: la espiral conservadora cae de 36% a 2.5%, y B2 se activa 5x más rápido en el escenario base. Estrategia (ADR-030): **MM activo desde el día 1 del TGE**, compensado con equity de Sociedad + préstamo de tokens (incentivo alineado, no efectivo). Esto reemplaza la estrategia previa de contingencia únicamente (ADR-019).

### 6.5 ROI del Inversionista (SIM 6)

| Escenario | Valor 6% Sociedad (5 años) | ROI | IRR | Recuperación |
|---|---|---|---|---|
| **Conservador** | $2.6M | 5x | 68% | ~22 meses |
| **Base** | $7.5M | **15x** | **147%** | **~14 meses** |
| **Agresivo** | $28.5M | 57x | 312% | ~8 meses |

Una participación del 6% en Sociedad (seed) genera $7.5M en ingresos acumulados por fees en 5 años en el escenario base — un retorno de 15x sobre una inversión de $500K. El modelo asume que el Vault KASH Core alcanza $25M (activando Motor B2) en la línea temporal mediana. Los escenarios conservador y agresivo acotan el rango.

---

## 7. Aura — Reputación Financiera On-Chain

Aura es un puntaje de reputación on-chain no transferible y acumulativo. Decae 2%/semana después de 90 días de inactividad. El nivel se determina por el máximo histórico (los niveles nunca retroceden).

| Nivel | Puntaje Aura | Beneficios Principales |
|---|---|---|
| Cachorro | 0-499 | Acceso básico, Motor D Capa 2, Tótem Nativo Bronce |
| Rastreador | 500-1,499 | Desbloqueo de Tótem Universal, mint de Avatar NFT, votación DAO |
| Cazador | 1,500-2,999 | Recompensas LP +12%, crear Manadas, liderar grupos |
| Alfa | 3,000-4,999 | Gobernanza avanzada, LP Fundador +20%, multiplicador x1.10 |
| Emperador | 5,000-9,999 | Override Anti-Whale, x1.15, acceso anticipado premium |
| Shamán | 10,000-24,999 | Gobernanza premium, acceso RWA (Etapa 3), x1.25 |
| Titán | 25,000+ | Exención total de Shield, fundar Dinastías, x1.30 |

Los usuarios ganan Aura a través de **152 misiones** en 7 senderos: educación (55 misiones), engagement diario (12), marketing viral (42), coleccionables (18), actividades grupales (12), eventos estacionales (13). El gancho cultural — "farmea tu Aura" — converge con el argot de internet existente, proporcionando marketing gratuito.

---

## 8. Ecosistema de Productos

### 8.1 Tótems (cNFTs)
Instrumentos financieros como NFTs comprimidos coleccionables (~$0.001/mint). Tres categorías:
- **Nativo:** Depósitos en $LUKA, fee 1.5%, sin restricción de acceso. Niveles Bronce/Plata/Oro por monto.
- **Universal:** Depósitos en $LUKA, fee 1.5%, requiere Rastreador (Aura ≥500).
- **Estándar:** Depósitos en SOL/USDC, fee 2%. Etapa 3.

El primer Tótem Nativo está condicionado a una misión educativa — los usuarios aprenden antes de invertir.

### 8.2 Manadas
Instrumentos financieros grupales creados por líderes de comunidad (requiere Cazador, Aura ≥1,500):
- **Vaca:** Ahorro grupal hacia una meta
- **Fondo:** Préstamos entre pares (crowdlending)
- **Negocio:** Financiación colectiva (crowdfunding)
- **Evento:** Boletos verificables on-chain
- **Club:** Membresía con marketplace

Todos los fees a través de Motor D Capa 2 (3% $LUKA). LUKAI arbitra disputas. El protocolo nunca presta ni toma custodia.

### 8.3 Jungle Arena
Sistema gamificado de misiones con resultados financieros reales:
- Duelos de Tótem (estadísticas de actividad real, apuestas en $LUKA)
- Conquista de Territorio (Manadas reclaman zonas del mapa)
- Predicción Relámpago (diaria, recompensas por racha)
- Battle Pass (mensual, pistas gratuita + premium)
- Proof of Roar (creación de contenido verificado para crecimiento viral)

### 8.4 LUKAI
Orquestador on-chain (v1.0, TGE) + interfaz conversacional de IA (v2.0, Etapa 2A).
- v1.0: Funciones keeper — actualizaciones de oracle, detección de régimen, drenaje de cola. Costo: $150-500/mes.
- v2.0: Asistente financiero de cara al usuario. Enrutamiento 80/20 (plantillas/Haiku para consultas comunes, Sonnet para complejas). Punto de equilibrio a ~$3K O&M/mes.

### 8.5 ASU (Seguro Digital)
Seguro paramétrico de dos capas:
1. Póliza externa (financiada por O&M) que cubre el Vault contra exploits
2. Producto interno para holders de Tótem — opt-in, pago automático, micro-prima en $LUKA

El Vault KASH Core NUNCA se usa para cubrir pérdidas. Las capas de seguro existen precisamente para protegerlo.

---

## 9. Hoja de Ruta

| Etapa | Línea temporal | Hitos |
|---|---|---|
| **Etapa 0: Pre-lanzamiento** | Ahora → TGE | Auditoría externa, génesis de comunidad, onboarding de MM, preparación de TGE |
| **Etapa 1: Génesis** | TGE | Motor A activo, fair-launch, Vault comienza a llenarse |
| **Etapa 2A: App** | Mes 2-6 | LUKASH App (pagos, Tótems, Jungle Arena), Motor B0 |
| **Etapa 2B: Madurez** | Mes 6-18 | K alcanza $25M, Motor B2 se activa, LUKAI v2.0 |
| **Etapa 3: Soberanía** | Año 2+ | Motor C (vías fiat), ENZ alcanzado, preparación de DAO |
| **Etapa 4: DAO** | Año 3+ | Descentralización completa, sunset de Guardian, gobernanza comunitaria |

### Bloqueantes Pre-TGE (no negociables)
1. Identificar 2 firmantes adicionales para Guardian (multisig 2-of-3)
2. Activar Guardian on-chain (unidireccional, irreversible)
3. Crear multisig O&M (2-of-3 vía Squads Protocol)
4. LP Fundador con lock on-chain de 365 días
5. Auditoría externa completada (vía subsidio Colosseum/Areta/Superteam)
6. Deployment en devnet estable 2+ semanas

---

## 10. Uso de Fondos ($500K Seed)

| Categoría | Monto | Asignación |
|---|---|---|
| **App / Tech** | $175K (35%) | Auditoría externa ($15-25K), frontend MVP, keeper LUKAI v1.0, infraestructura |
| **Liquidez Inicial** | $100K (20%) | Pool $LUKA/SOL en Meteora, LP bloqueado permanentemente |
| **Marketing + MM** | $150K (30%) | KOLs (tokens con vesting, $0 en efectivo), community manager, plataformas orgánicas, anuncios. MM activo desde TGE (compensado con equity de Sociedad + préstamo de tokens, no efectivo — ADR-030). Listado CEX $30K |
| **Legal** | $50K (10%) | Abogado cripto, entidad en El Salvador, marco regulatorio |
| **Reserva Operativa** | $25K (5%) | O&M del fundador meses 1-6, emergencias |

---

## 11. Stack Técnico

- **Smart contracts:** Rust + Anchor (v10.2, ~2,100 líneas, desplegados en Solana devnet)
- **Estándar de token:** Token-2022 con Transfer Hook (mainnet). SPL clásico (testing en devnet)
- **Oracles:** Pyth Network (primario) + Switchboard (redundancia). Deserialización manual de Pyth V2 (sin dependencia de `pyth-sdk-solana`)
- **Infraestructura DEX:** Meteora (LP), Jupiter (swaps/routing), Jito (bundles privados, anti-MEV)
- **Liquid staking:** Sanctum (paridad LST), JitoSOL, mSOL
- **Lending:** Kamino / Marginfi (rendimiento USDC)
- **CI/CD:** GitHub Actions — Clippy (lints DeFi), Soteria (25+ vulnerabilidades Solana), Anchor build, Anchor test
- **Caché off-chain:** Supabase PostgreSQL
- **Frontend:** TypeScript (planificado)

### Deployment en Devnet
- Program ID: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`
- Token $LUKA (SPL, devnet): `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`
- Config PDA: `3MqnJPy3RtUhqkTL2bmkTfp7vPHwt5ALUCg7MbcWsgPf`
- State PDA: `6bzY2xkCkkUTAwmZhVS67Jxygc5phMMUb2knWY124MWC`
- Burn Vault PDA: `7iD2hbX9FawsHLW4NyrNzzBy46qr4p3JuiEX8UNAyF2f`

Flujos verificados end-to-end: `update_oracle_state → refresh_vault_valuation (Pyth) → process_fee (quema real via CPI + distribución atómica + swaps pendientes) → execute_vault_swaps (USD→nativo a precio de oracle)`.

---

## 12. Equipo

**Kash Sensei** — Fundador y Único Constructor
- Desarrollo aumentado por IA: diseño del protocolo, smart contracts, simulaciones, marca, pitch — todo construido con Claude Code como copiloto
- Experiencia: Arquitecto de soluciones. Identidad completa compartida bajo NDA.
- Identidad: Pseudónimo en público (voz, sin rostro). KYC completo para inversionistas y trámites legales.

La estructura de equipo liviana es intencional: un fundador solo con herramientas de IA puede moverse más rápido que un equipo de 5 personas, con menor tasa de quema y toma de decisiones más clara. El presupuesto de $500K incluye la contratación de un community manager (24/7) y un desarrollador frontend para el MVP de la App.

---

## 13. Marco Legal

- **Jurisdicción objetivo:** El Salvador (CNAD — 0% impuesto cripto, capital mínimo $2K, ~$5.5K de registro)
- **Clasificación del token:** Token de utilidad (moneda transaccional que construye una reserva; la reserva NO se distribuye a holders — esto debilita la prueba de Howey en "expectativa de ganancias por esfuerzos de otros")
- **Diseño de Guardian:** No puede mover fondos, no puede modificar parámetros, no puede actualizar contratos — solo pausar. Esto debilita aún más el elemento de "esfuerzos de otros".
- **Marco de cNFT:** Instrumentos financieros con disclaimer on-chain (sin rendimientos garantizados, sin protección de capital). Revisión legal pendiente.
- **Motor C (Etapa 3):** Las vías fiat requieren licencia de transmisión de dinero — diferido hasta tracción y asesoría legal.

**Pendiente:** Consulta con abogado cripto para el marco regulatorio de cNFT y Motor C (presupuestado en Legal $50K).

---

## 14. Factores de Riesgo

Creemos en la divulgación transparente. Estos son los riesgos reales:

| Riesgo | Severidad | Mitigación |
|---|---|---|
| **Bajo volumen (espiral de muerte)** | ALTA | 30% del presupuesto a marketing. MM activo desde TGE (ADR-030) reduce la espiral conservadora de 36% a 2.5%. Motor A genera 4.6x más crecimiento del Vault que cualquier otro factor. |
| **Exploit de smart contract** | ALTA | Auditoría externa pre-TGE (no negociable). Pipeline CI. KASH Shield. Seguro ASU. Composición del Vault diversificada. |
| **Acción regulatoria** | MEDIA | Jurisdicción en El Salvador. Marco de token de utilidad. Guardian no puede mover fondos. cNFTs con disclaimer. Motor C diferido. |
| **Manipulación de oracle** | MEDIA | Redundancia Pyth + Switchboard. Verificación de intervalo de confianza del 2%. Pausa automática vía Circuit Breaker. |
| **Riesgo de fundador único** | MEDIA | Desarrollo aumentado por IA reduce el factor de bus. Código documentado. Inversionista puede hacer fork. Transición a DAO planificada. |
| **Barreras de adopción en LatAm** | MEDIA | Trilingüe (ES/EN/PT). Onboarding gamificado vía Jungle Arena. Tótem condicionado a educación. Gancho cultural ("farmea tu Aura"). |
| **Compatibilidad Token-2022** | BAJA | Jupiter, Meteora y la infraestructura principal de Solana ya soportan Token-2022. Validado pre-TGE. |

---

## 15. Por Qué Ahora

1. **Madurez de Solana:** Fees lo suficientemente bajos para micro-transacciones. Ecosistema (Meteora, Jupiter, Jito, Pyth) listo para producción.
2. **Convergencia cultural:** "Farmear Aura" ya es argot de internet — la marca obtiene distribución cultural gratuita.
3. **Momento LatAm:** Adopción récord de banca móvil. Uso de stablecoins en crecimiento. Ventanas regulatorias abriéndose (El Salvador, Brasil, Colombia).
4. **Construcción aumentada por IA:** Un fundador solo ahora puede construir lo que antes tomaba equipos de 10 personas y dos años. El protocolo, contratos, simulaciones, marca y materiales de pitch fueron construidos en 12 días con Claude Code.

---

## Apéndices

### A. Registro de ADRs
30 Architecture Decision Records que gobiernan todas las decisiones del protocolo. Disponibles en `.claude/knowledge/key-decisions.md`. ADRs clave para inversionistas: ADR-013 (Estructura Seed), ADR-014 (Asignación del Equipo), ADR-018 (Presupuesto), ADR-030 (Estructura de Sociedad + estrategia MM), ADR-028 (Narrativa de Banco Central Optimizado).

### B. Resultados de Simulaciones
Resultados completos de Monte Carlo v4.3 disponibles en `simulations/out/`. 7 simulaciones: SIM 0 (invariantes), SIM 1 (MC principal), SIM 2 (sensibilidad), SIM 3 (estrés), SIM 4 (optimización de throttle), SIM 5 (impacto MM), SIM 6 (ROI del inversionista). Reporte HTML + PDF en `simulations/out/informe_simulaciones_v4.3.*`.

### C. Reportes de Auditoría
- `audits/SECURITY_AUDIT_LIB_RS_V9_1.md` — Auditoría de seguridad manual (10 categorías, 13 hallazgos, todos resueltos)
- `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` — Validación de simulación fiel al contrato
- `audits/AUDITORIA_INTEGRAL_Y_VEREDICTO.md` — Auditoría integral del protocolo (8.5/10 concepto, 6.5-7/10 viabilidad con enfoque)
- Auditoría externa: pendiente (bloqueante pre-TGE)

### D. Especificación del Protocolo
Protocolo completo v4.3 disponible en `docs/protocolo/LUKASH_Protocolo_v4.3.md` (~2,300 líneas). Consolidado desde v4.2 con 20 correcciones (C1-C20).

---

*$LUKA es un token de utilidad del ecosistema LUKASH. La reserva respalda el sistema; no constituye una promesa de rendimientos ni un valor negociable. Nada en este documento constituye asesoría financiera. Los resultados de simulaciones pasadas no garantizan desempeño futuro.*
