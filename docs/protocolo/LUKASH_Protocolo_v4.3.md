# PROTOCOLO LUKASH v4.3 — Arquitectura Canónica de Implementación

> Ing. Sebastián Botero Pabón · Agosto 2026 · CONFIDENCIAL — USO EXCLUSIVO PARA INVERSORES
> **Este documento es la fuente de verdad para la implementación.** Consolida el Protocolo v4.2 y aplica
> todas las correcciones de la auditoría 2026-08-19. Donde una sección es puramente narrativa/de mercado,
> se resume y se remite al v4.2 (`docs/protocolo/LUKASH_Protocolo_v4_2_FINAL.docx`). Los parámetros técnicos
> están completos y son definitivos.
> Tesis: *"Las finanzas del futuro, disponibles hoy para toda Latinoamérica."*

---

## CAMBIOS v4.2 → v4.3 (correcciones de auditoría)

| # | Corrección | Antes (v4.2) | Ahora (v4.3) |
| --- | --- | --- | --- |
| C1 | **Composición del Vault** | Sumaba 105% (incluía oráculos 5%) | **100%** — se eliminan PYTH/JTO/JUP como reserva (pasan a infraestructura O&M). cBTC 35 / SOL 15 / LST 20 / USDC reserva 25 / lending 5 |
| C2 | **Liberación Vault Sociedad** | Atada a $30M (TVL) *y* a Etapa 3 ($50M) — contradictorio | **Un solo hito Jaguar Lock**: KASH Core $30M o 12 meses. Fees líquidos desde mes 13 (Etapa 2B). Etapa 3 ya no la "reinicia" |
| C3 | **Reputación** | "Jaguar Score", 4 niveles | **Aura**, 5 niveles (Cachorro→Rastreador→Cazador→Jaguar Maduro→Jaguar Sabio) |
| C4 | **Nomenclatura Precio KASH** | "P_th" y "P_KASH" mezclados | **P_KASH** unificado |
| C5 | **LUKAI** | "v4.1" + releases v1.0/v2.0 (confuso) | **LUKAI** con releases **v1.0** (orquestador) y **v2.0** (IA). Sin "v4.1" |
| C6 | **cBTC en capas del Vault** | Listado en Capa 2 y Capa 3 a la vez | Solo en **Capa 3 (Reserva Profunda)**. Capa 2 = SOL+LST+USDC lending |
| C7 | **Los dos 70/30** | Ambiguos | **Split de Fee** (70% Core / 30% Sociedad) vs **Split de Yield** (70% reinversión / 30% R_op) |
| C8 | **Ticker** | "$Lukas"/"$LUKA" mezclados en material viejo | **$LUKA** canónico |
| C9 | **Cifras Monte Carlo** | Atribuidas al modelo v4.2 | Etiquetadas como **modelo conservador v3.1** — reconciliación pendiente (ver `audits/AUDITORIA_COMPLEMENTARIA.md`) |

---

## 0. Tesis (resumen)
LUKASH es un ecosistema DeFi sobre Solana: la puerta de entrada de 260M de latinoamericanos excluidos a la
infraestructura financiera on-chain que las instituciones (BlackRock BUIDL $2.9B, Franklin Templeton, Siemens,
Apollo) ya construyen. Cada unidad de **$LUKA** está respaldada por una reserva creciente de activos duros
(el **Vault KASH Core** / "La Reserva Sagrada" en la UI), auditada on-chain. Deflacionario, verificable, de
reserva creciente. El hype es el bootstrapping, no el producto. *(Narrativa completa y datos de mercado: v4.2 §0.)*

---

## 1. Jerarquía de fees y etapas

| Etapa | Fee Base | Fee WL | Motores | Descripción |
| --- | --- | --- | --- | --- |
| **1 — Génesis** | 4.0% | 2.5% | A | Deflación agresiva, acumulación del Vault desde TGE (ref. $WIF) |
| **2A — App / B0** | 2.5% | 1.5% | A + B0 + D | App en adopción. Vault hacia K_min $25M. Motor D 4 capas |
| **2B — Madurez / B2** | 2.5% | 1.5% | A + B2 + D | K(t) ≥ $25M. Motor B2 (LP recircula). Libera Vault Sociedad (mes 13) |
| **3 — Soberanía** | 0.5% | 0.5% | A + B2 + C + D | K > $50M. Motor C activo. Integración fiat. Supply fijo 3.3B, ENZ |

### 1.1 Motor D — 4 capas (regla del diferencial: pagar en $LUKA es siempre 0.5% más barato)

| Capa | Servicio | Fee $LUKA | Fee SOL/USDC | Identidad / gate |
| --- | --- | --- | --- | --- |
| **0** | Staking $LUKA / Pools internos | **0%** | N/A | Base silenciosa. Exención absoluta e incondicional (no modificable por gobernanza) |
| **1** | DeFi interno premium en $LUKA | 1.5% | — | Club privado. Gated por cNFT Jaguar Universal (**Aura ≥ 500 = nivel Rastreador**). APY variable |
| **2** | Servicios cotidianos: Crowdfunding, Crowdlending, Fondos, Seguros, Eventos, Marketplaces, Jungle Arena, Bet & Win | **3%** | 3.5% | Corazón de la comunidad. Motor de adopción masiva |
| **3A** | DeFi externo en $LUKA (Kamino, Marginfi, RWA) | 1.5% | swap previo sin cargo + 1.5% | Puente soberano. Rendimiento externo 100% al usuario |
| **3B** | DeFi externo SOL/USDC | N/A | 2% | Puerta de entrada sin $LUKA. **Idéntico al Motor C en mecánica de distribución** (no en tasa: fee 2%) |

Reglas: Capa 0 exención absoluta · diferencial 0.5% siempre a favor de $LUKA · en 3A/3B LUKAI hace el swap interno sin cobro adicional (routing contable).

---

## 2. Tokenomics

**Supply:** inicial **10,000M** $LUKA → objetivo **3,300M** (quemar 6,700M) → **Emisión Neta Cero** al llegar a 3.3B.

| Categoría | % | Cantidad | Seguridad |
| --- | ---: | ---: | --- |
| Venta (Seed + Public) | 45% | 4,500M | Vesting escalonado seed. Circulante desde TGE |
| Pool de Liquidez (LP) | 30% | 3,000M | Emparejados/quemados en Meteora. LP Fundador 365d lock |
| Marketing / CEX | 10% | 1,000M | Multisig. KOLs y MM |
| Airdrops / Comunidad | 10% | 1,000M | Por hitos on-chain públicos |
| Staking | 5% | 500M | Recompensa base. Financiado por Motor A |

**ENZ (Fase 3):** al llegar a 3,300M, PDA on-chain suspende la quema; el valor capturado va a recompensas
perpetuas de staking y distribución a la comunidad. $LUKA = acción preferente con dividendos del Vault.

**Vesting equipo (no negociable):** publicado on-chain ANTES del TGE. <10% supply, cliff 18m, lineal 36m.

---

## 3. Arquitectura de capital: Core, Sociedad y LP

| Destino | % capital | Función | Seguridad |
| --- | ---: | --- | --- |
| Pool de Liquidez (LP) | 30% | Soporte de trading en Meteora | Tokens emparejados/quemados. LP Fundador 365d lock |
| **Vault KASH Core** (Reserva Soberana) | 40% | Respaldo inamovible del valor de $LUKA | Nunca liquidable. Solo crece. Determina P_KASH |
| **Vault KASH Sociedad** (Equity) | 30% | Renta líquida y crecimiento patrimonial de socios | Bajo Jaguar Lock. Liberación por hitos |

### 3.1 Core vs Sociedad — los dos splits (¡son distintos!)

| Concepto | KASH Core | KASH Sociedad |
| --- | --- | --- |
| Función | Respaldo y precio piso. El muro institucional | Renta líquida y patrimonio de socios |
| **Split de Fee** (del componente KASH de cada fee) | **70% al Core** | **30% a la Sociedad** (líquido desde mes 13) |
| **Split de Yield** (del rendimiento del Vault) | **70% reinversión compuesta** / **30% a R_op** | 100% capitalizable |
| Acceso | Nunca liquidable. Solo crece | Bajo Jaguar Lock (ver 3.2) |

> Aclaración de nomenclatura (C7): el **Split de Fee** (70/30 Core/Sociedad) y el **Split de Yield**
> (70/30 reinversión/R_op) son dos reglas distintas sobre bases distintas. No confundir.

### 3.2 Jaguar Lock — liberación del Vault Sociedad (REGLA ÚNICA, corregida C2)

**Un solo hito gobierna la liberación de los accionistas:**

| Disparador (Jaguar Lock) | Fees a la Sociedad | Capital principal de la Sociedad |
| --- | --- | --- |
| El **KASH Core** alcanza **$30M USD** *o* transcurren **12 meses**, lo que ocurra primero | Líquidos y perpetuos **desde el mes 13** (ocurre en **Etapa 2B**) | Liberación lineal en **48 meses** post-hito |

- El umbral **$30M es sobre el KASH Core** (misma métrica K que K_min $25M y Etapa 3 $50M). **No es TVL total.**
- **Etapa 3 (K>$50M) NO reinicia** esta distribución: la distribución a la Sociedad ya está activa desde el
  mes 13. El rasgo distintivo de Etapa 3 es Motor C + integración fiat + escala.
- Escalera canónica de umbrales, toda sobre el **KASH Core**:
  `$25M (activa B2) → $30M o 12m (Jaguar Lock: libera Sociedad, mes 13) → $50M (Etapa 3: Motor C) → $100M (escala)`.

### 3.3 Sostenibilidad de infraestructura
El escalamiento y mantenimiento se financian con una fracción del Vault Sociedad acumulado en Etapa 1.
Sin dependencia de VCs, sin dilución de fundadores. La infraestructura crece con el éxito del protocolo,
sin tocar el KASH Core.

---

## 4. Vault KASH Core — composición v4.3 (100%, corregida C1)

Métrica estructural clave. Su valor en USD (**K**) determina la activación del Motor B2 (K_min=$25M) y el
Precio KASH. 100% Solana-nativo (sin bridge).

| Activo | Asignación | Yield anual | Justificación | Riesgo bridge |
| --- | ---: | --- | --- | --- |
| cBTC (Bitcoin nativo, cbBTC) | **35%** | 0% + apreciación | Reserva de valor primaria | NINGUNO |
| SOL nativo | **15%** | 0% + apreciación | Liquidez operativa, gas | NINGUNO |
| SOL/LST (JitoSOL + mSOL) | **20%** | 6–8% | Exposición SOL con yield. JitoSOL anti-MEV | NINGUNO |
| USDC Reserva Inmediata | **25%** | 0% (líquido) | Buffer Capa 1. Primera línea de emergencias | NINGUNO |
| USDC Lending (Kamino/Marginfi) | **5%** | 4–8% | Capital ocioso con yield. Retiro <24h | NINGUNO |
| **TOTAL** | **100%** | | | |

> **Oráculos (C1):** PYTH/Switchboard/Jupiter son **infraestructura operativa** (lectura de precios, routing),
> financiada del **O&M**, no una línea de reserva del Vault. No se compran como "activo duro". La exposición a
> Jito ya está en el LST (JitoSOL).

### 4.1 Arquitectura de 3 capas del Vault (corregida C6)

| Capa | Composición | Función | Acceso |
| --- | --- | --- | --- |
| **1 — Liquidez Inmediata** | USDC Reserva (25%) | ≈6 meses de gastos operativos. Primera respuesta | Nunca inmovilizado |
| **2 — Trabajo Activo** | SOL (15%) + SOL/LST (20%) + USDC Lending (5%) = 40% | Rebalanceo gestionado por LUKAI según régimen | Rebalanceo gradual solo con nuevas entradas |
| **3 — Reserva Profunda** | **cBTC (35%)** | Reserva de último recurso | Solo por Tridente Multisig (3 de 3) en emergencias |

Suma: 25 + 40 + 35 = **100%**. (cBTC solo en Capa 3.)

### 4.2 Gestión del Yield — Split de Yield 70/30 y rol de la R_op
- **70%** del yield del Vault → reinversión compuesta. **30%** → **R_op** (Reserva Operativa USDC).
- La **R_op se acumula IDLE** como reserva de estabilización de último recurso. **NO interviene en el flujo
  normal del Motor B0 ni B2.** En B0 el tramo LP quema directo sin USDC; en B2 el precio se recupera por
  inercia estructural. (Corrección arquitectónica clave heredada de v4.2.)

| Capital del Vault | Yield LST 20% (6-8%) | Yield USDC Lending 5% (4-8%) | Yield total | Aporte R_op (30%) |
| --- | --- | --- | --- | --- |
| $10M | $120K-$160K | $20K-$40K | $140K-$200K | $42K-$60K |
| $25M (K_min) | $300K-$400K | $50K-$100K | $350K-$500K | $105K-$150K |
| $50M | $600K-$800K | $100K-$200K | $700K-$1M | $210K-$300K |

- **UCR (Umbral Crítico de Reserva):** si la R_op cae bajo el 20% de su capacidad proyectada, LUKAI activa
  Recarga Forzada (15% del yield semanal → USDC). Solo para estrés extremo.

### 4.3 Precio KASH (P_KASH) — piso de valor institucional
`P_KASH = Valor del Vault Core / Supply Circulante`. Precio al que el protocolo puede recomprar todos los
tokens con activos reales. Único precio garantizable. Crece con cada transacción. Verificable on-chain.

| Año | Vault (mediana, neutral)* | Supply | P_KASH | Múltiplo vs TGE ($0.0001) |
| --- | --- | --- | --- | --- |
| 1 | $20.9M | 3.3B | $0.0063 | 63x |
| 2 | $29.6M | 3.3B | $0.0090 | 90x |
| 3 | $33.7M | 3.3B | $0.0102 | 102x |
| 5 | $37.2M | 3.3B | $0.0113 | 113x |

\* **Cifras del modelo conservador v3.1.** La reconciliación con el modelo v4.2 está pendiente (ver
`audits/AUDITORIA_COMPLEMENTARIA.md`, hallazgo H10). No usar sin etiquetar la fuente.

---

## 5. LUKAI — orquestador de estados (corregido C5)

| Release | Capacidades |
| --- | --- |
| **LUKAI v1.0** (TGE) | Orquestador on-chain puro: conmutación B0/B2, Throttle dinámico, módulo contra-cíclico, routing cNFT por capa, monitor de paridad LSTs, gestión UCR/Recarga Forzada |
| **LUKAI v2.0** (Etapa 2A) | + Interfaz conversacional IA (chamán digital): onboarding educativo, asesoría por nivel de Aura, notificaciones del Vault en lenguaje humano, curaduría de instrumentos cNFT |

Módulo contra-cíclico (dirige solo NUEVAS entradas, no toca el Vault existente):

| Régimen | Activos volátiles | USDC | Lógica |
| --- | ---: | ---: | --- |
| BEAR (EMA30 < EMA90 BTC) | 70% | 30% | Acumula activos duros baratos |
| NEUTRAL | 75% | 25% | Proporción estándar |
| BULL (EMA30 > EMA90 BTC) | 40% | 60% | Construye USDC para el próximo bear (+22-23% Vault final) |

---

## 6. Los 4 Motores — regla universal 35/35/15/15

Cada fee, en todos los motores y capas, se distribuye: **35% Vault/Asset Layer · 35% LP/Quema · 15% O&M · 15% Staking.**

| Motor | Divisa | Fee (Et.1/WL) | Desde | Identidad |
| --- | --- | --- | --- | --- |
| **A — El Cazador** | SOL | 4% / 2.5% | TGE | Captura capital especulativo del DEX → Vault |
| **B — El Motor Interno** | $LUKA | 2.5% / 1.5% | **Etapa 2A** | App: quema (B0) o recircula (B2) |
| **C — El de Escala** | USDC | 0.5% | Etapa 3 | Pagos masivos → presión de compra |
| **D — El Alma** | multi | 0%→3.5% | Etapa 2A | Manadas, gaming, cNFTs, DeFi (4 capas) |

### 6.1 Flujos atómicos (ejemplos)
- **Motor A** (100 SOL, fee 4 SOL): 1.40 (35%) → activos KASH → Vault Core `[35% cBTC · 25% USDC · 15% SOL · 20% LST · 5% lending]` · 1.40 (35%) → compra $LUKA → BURN · 0.60 (15%) → O&M · 0.60 (15%) → compra $LUKA → Staking.
- **Motor B:** 35% Asset Layer → Hard Assets (NUNCA quema, sin R_op) · 35% LP → **B0: burn directo a null / B2: recirculación al pool** · 15% staking · 15% O&M. *Inercia estructural en B2 (sin tocar Vault ni R_op): Motor A activo + Jaguar Exit Fee + Anti-Whale + cola diferida + arbitraje natural.*
- **Motor C** (fee 0.5% USDC): 35% → Vault (buffer USDC) · 35% → compra $LUKA → LP · 15% O&M · 15% → $LUKA staking.
- **Motor D:** decisión por capa (0 exento / 1 = 1.5% $LUKA gated / 2 = 3%-3.5% / 3A = 1.5% / 3B = 2%), luego 35/35/15/15.

---

## 7. Throttle dinámico (dos algoritmos de lógica inversa)

**Etapas 1-2 (pre-ENZ)** — modula la velocidad de quema del tramo LP del Motor B según P_mercado vs EMA30:

| Condición (P vs EMA30) | Modo | Quema | Cola diferida |
| --- | --- | --- | --- |
| P > 1.2× | ACELERADO | 125% | — |
| 0.8×–1.2× | NORMAL | 100% | — |
| 0.5×–0.8× | CONSERVADOR | 60% | 40% |
| P < 0.5× | DEFENSIVO | 25% | 75% |

Cola on-chain pública; se ejecuta ≤10%/semana al normalizar. Modo DEFENSIVO = supervivencia (protege R_op).

**Etapa 3 (post-ENZ)** — lógica INVERTIDA (maximiza valor USD del Vault): BULL acumula USDC · NEUTRAL DCA proporcional · BEAR DCA agresivo de Hard Assets con el USDC acumulado.

> El ~1.4% de espiral de muerte es riesgo de **transición B0→B2**; en B2 consolidado <0.5% por inercia
> estructural. En B2 el Vault NO se toca para defender el precio. *(Cifras del modelo v3.1 — ver H10.)*

---

## 8. Sistema cNFT — instrumentos financieros tokenizados

Los cNFT **no son llaves de acceso**: son instrumentos de inversión (capital + rendimiento). Transferibles,
auditables, con **disclaimer on-chain obligatorio** (no garantizan rendimiento ni protección de capital).

| Tipo cNFT | Capa | Activo | Fee | Gate de Aura | Tótem (UI) |
| --- | --- | --- | --- | --- | --- |
| **Nativo $LUKA** | 0 | $LUKA | 0% | Ninguno (Cachorro) | Bronce |
| **Jaguar Universal** | 1 y 3A | $LUKA | 1.5% | **Aura ≥ 500 (Rastreador)** | Plata/Oro |
| **Estándar SOL/USDC** | 3B | SOL/USDC | 2% | Ninguno | — |

Modos de rendimiento: **A — Compuesto** (reinvierte) · **B — Distribución periódica** (paga al holder).
En Etapa 3, opción C: redimir Modo B y re-emitir Modo A pagando fee de cambio de modo.

> **Nombres de marca (Tótems):** Fuego Eterno (cBTC), Espíritu (JitoSOL), Agua Viva (mSOL), USDC Sagrado,
> Jaguar Negro (premium) — capa de UI sobre los 3 tipos técnicos. Ver `brand/DESIGN_TOKENS.md`.

**PENDIENTES bloqueantes de smart contract:** (1) mercado secundario de cNFT (fee de venta, herencia del modo,
actualización de Aura al transferir); (2) frecuencia de distribución del Modo B (semanal/mensual, fija/elegible).

---

## 9. AURA — reputación financiera on-chain (corregido C3)

**Aura** ("el resplandor del Jaguar") reemplaza a "Jaguar Score". On-chain, no transferible, acumulativa.
Decae 2%/semana tras 90d de inactividad. El nivel usa el Aura máximo histórico. Calibrada por LUKAI.

### 9.1 Niveles (5, arco narrativo)

| Nivel | Aura | Beneficios |
| --- | --- | --- |
| **Cachorro** | 0–499 | Acceso básico Motor D Capa 2, Manadas estándar, cNFT Nativo |
| **Rastreador** | 500–1,999 | **cNFT Jaguar Universal → DeFi Capa 1.** Voto Motor D (DAO). Acceso prioritario a nuevos cNFT |
| **Cazador** | 2,000–4,999 | LP Jaguar 90d +12%. Ranking de Manadas. Acceso anticipado Capas 3A/3B. Liderar Manada |
| **Alfa** | 5,000–9,999 | Gobernanza avanzada. LP Fundador 365d +20%. Override Anti-Whale. Multiplicador Aura ×1.15 |
| **Jaguar** | 10,000+ | Gobernanza premium. RWA/T-bills tokenizados (Fase 3). Multiplicador ×1.30. Skin legendario |

Umbrales 500/2,000/5,000 preservados de v4.2 (compatibilidad de gates). Detalle de calibración y misiones:
`specs/01-aura-jungle-arena.md`.

---

## 10. Jaguar Shield — infraestructura de seguridad

| Mecanismo | Descripción |
| --- | --- |
| Circuit Breaker Vault | Variación negativa >10% en 1h → Modo Emergencia (pausa swaps/salidas 24h, auditoría Multisig+LUKAI) |
| Circuit Breaker LP | 3 niveles de delay (Alerta >10%/2h · Delay 12h >15%/4h · Delay 48h >30%/24h). Sin confiscación |
| Monitor paridad LST | JitoSOL/mSOL vía Sanctum; desviación >3% → bloquea conversiones |
| Oráculos redundantes | TWAP Pyth + Switchboard, umbral 2% |
| Anti-MEV | Bundles privados Jito |
| Seguro Anti-Exploit | Hasta 5% del Vault (Halborn/OtterSec). Activo desde TGE |
| Tridente Multisig | 3-de-3 para: activar Capa 3, cancelar Circuit Breaker LP, modificar K_min |
| **Jaguar Exit Fee** | Dual: precio <0.7×EMA30 AND venta >0.3% supply/hora → 5%/3%/1% (Et.1/2/3). 100% al Vault Core |
| **Anti-Whale** | 1-2% supply: 3% · 2-5%: 6% · >5%: 10% sobre el excedente. 100% al Vault Core |
| **Timelock 48h** | Obligatorio para cambiar K_min, fees, thresholds del Throttle, composición del Vault |

Exit Fee/Anti-Whale exenciones: swaps internos Motor D, staking activo, LP en lock, MMs registrados, nivel Jaguar Maduro+.

---

## 11. Etapas del protocolo + mapa Fase/Etapa (canónico, corregido C2)

| Etapa | Nombre | Condición | Motores | Capital Inversionistas |
| --- | --- | --- | --- | --- |
| 0 | Pre-lanzamiento | Auditoría completa. Vesting on-chain. 4 Frentes listos | — | Capital en KASH Sociedad bajo Jaguar Lock |
| 1 | Génesis | TGE. LP en Meteora. MM desde D1 | A | LP Fundador 365d. Jaguar Shield activo |
| 2A | App / B0 | App lanzada. K < $25M | A + B0 + D | Reserva Jaguar Shield. LP Comprometido disponible |
| **2B** | Madurez / B2 | K ≥ $25M (Pyth) | A + B2 + D | **Libera Vault Sociedad: fees líquidos desde mes 13 (hito Jaguar Lock $30M Core o 12m)** |
| 3 | Soberanía | K > $50M. Fiat activa | A + B2 + C + D | **Distribución a Sociedad ya activa (desde mes 13). Foco: Motor C y escala** |
| 4 | DAO | Año 5. Supply ≤ 3.3B. ENZ | Todos (quema suspendida) | Control pasa al DAO |

**Mapa Fase (inversión) ↔ Etapa (protocolo):** Fase 1 ($500K) ≈ Etapas 0-1 (+ prototipo App) · Fase 2 ($2M) ≈ Etapas 2A-2B · Fase 3 ($5M) ≈ Etapa 3. Etapa 4 (DAO) es post-Fase 3. **"Fase" = capital; "Etapa" = condición on-chain; "B0/B2" = estado del Motor B. No confundir.**

---

## 12. Inversión y CAC
Fase 1 **$500K** (Core & Awareness) · Fase 2 **$2M** (Product & Scale, 400-500K usuarios) · Fase 3 **$5M**
(Utilidad en economía real). Solo Fase 1 requiere capital externo; Fases 2-3 se co-financian con el Vault
Sociedad. CAC: $5-9 (Et.1) → $3-5 (Et.2 viral) → $1 (orgánico). *(Detalle componentes: v4.2 §15 / BMC §9.2.)*

---

## 13. Parámetros formales — Blueprint de smart contracts (definitivo)

| Parámetro | Valor v4.3 | Notas |
| --- | --- | --- |
| K_min (activa Motor B2) | $25,000,000 | Hardcoded. Modificable solo por DAO (Etapa 4). Verificación Pyth |
| Jaguar Lock (libera Sociedad) | $30M Core **o** 12 meses | Sobre KASH Core. Fees líquidos mes 13; principal lineal 48m |
| Etapa 3 (Motor C) | K > $50M | + integración fiat |
| Escala final | $100M en Vault **Core** | Objetivo original |
| Fee Motor A Et.1 / Et.2 | 4.0% / 2.5% | WL 2.5% / 1.5% |
| Fee Motor B (desde 2A) | 2.5% | WL 1.5% |
| Fee Motor C (Et.3) | 0.5% | Solo USDC |
| Fee Motor D | Capa0 0% · Capa1 1.5% · Capa2 3%/3.5% · Capa3A 1.5% · Capa3B 2% | Diferencial 0.5% pro-$LUKA |
| Distribución fees | 35% Vault / 35% LP-Quema / 15% O&M / 15% Staking | Atómica. Split de Fee Vault: 70% Core / 30% Sociedad |
| **Vault** | cBTC 35 · SOL 15 · LST 20 · USDC reserva 25 · lending 5 = **100%** | Sin oráculos como reserva |
| Throttle | ACEL 125% (P>1.2×) · NORM 100% · CONS 60%/cola40% · DEF 25%/cola75% | Vía LUKAI + Pyth. Cola ≤10%/sem |
| Circuit Breaker LP | Delay 12h (>15%/4h) · 48h (>30%/24h) | Override Tridente |
| UCR | 20% de R_op → Recarga Forzada 15% yield/sem | |
| Cap burn diario | 1% supply/día | Exceso al día siguiente |
| Jaguar Exit Fee | precio<0.7×EMA30 AND venta>0.3% supply/h → 5/3/1% | 100% al Vault Core |
| Anti-Whale | 1-2%:3% · 2-5%:6% · >5%:10% | 100% al Vault Core |
| Timelock gobernanza | 48h | Parámetros críticos |
| Aura niveles | Cachorro 0-499 · Rastreador 500-1999 · Cazador 2000-4999 · Alfa 5000-9999 · Jaguar 10000+ | Gate cNFT Jaguar Universal: ≥500 |
| Supply objetivo / ENZ | 3,300M | PDA suspende quema |

---

## 14. Riesgos (resumen) — ver `audits/` para el detalle
- Espiral de muerte: ~1.4% transición B0→B2, <0.5% B2 consolidado *(cifras v3.1)*.
- **Abiertos (heredados de auditorías v3, no cerrados por v4.x):** auditoría de contratos + seguro anti-exploit
  (bloqueante TGE), oráculo único Pyth sin fallback implementado, correlación Vault ~90%, 45% supply circulante
  en TGE sin vesting seed obligatorio, consistencia burn/staking en modo DEFENSIVO. Ver `AUDITORIA_COMPLEMENTARIA.md`.
- **Provenance Monte Carlo (H10):** las cifras publicadas provienen del modelo conservador v3.1, no del código v4.2. Reconciliación pendiente.

---

## 15. Métricas de salud on-chain (dashboard)
K(t) vs K_min · R_op vs UCR · cola de quemas diferidas · S(t) supply vs 3.3B · modo Throttle · LP total · Aura
agregado · usuarios activos mensuales · régimen de mercado LUKAI. *(Umbrales: v4.2 §19.)*

---

## Apéndice — Pendientes para implementación
**Bloqueantes smart contract:** mercado secundario cNFT · frecuencia Modo B · reconciliación Monte Carlo (H10).
**Fase App:** mecánicas detalladas de Jungle Arena (→ `specs/01-aura-jungle-arena.md`, en diseño) · parámetros de
Manadas · go-to-market local · yield sharing DeFi externo Capa 3 · recalibración de Aura post-datos reales.

---
*LUKASH Protocol v4.3 — Arquitectura Canónica de Implementación. Correcciones de auditoría 2026-08-19 aplicadas.
Ing. Sebastián Botero Pabón — Arquitecto del Protocolo.*
