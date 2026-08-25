# LUKAI -- Analisis de Costo y Arquitectura

> Fecha: 2026-08-24 | Autor: Studio Airquitect | Documento interno LUKASH Protocol
> Precios API: tabla oficial Anthropic (cached 2026-06-24). Prompt caching: docs oficiales.

---

## 0. Resumen ejecutivo

LUKAI tiene dos fases con perfiles de costo radicalmente distintos:

- **v1.0 (TGE):** Keeper bot sin LLM. Costo fijo bajo (~$150--$500/mes).
- **v2.0 (Etapa 2A+):** Asistente conversacional con LLM. Costo variable por MAU que escala linealmente.

La viabilidad de v2.0 depende del volumen de fees del protocolo (bucket O&M = 15%).
Con la estrategia de routing propuesta (80% Haiku / 20% Sonnet), LUKAI v2.0 es
sostenible a partir de ~$15K--$20K de O&M mensual (~10K MAU).
Por debajo de 5K MAU, el costo LLM es tan bajo (~$1.5K/mes) que se absorbe
facilmente con O&M minimo. El riesgo no es el costo unitario, sino tener
suficiente O&M para justificar el esfuerzo de desarrollo.

---

## 1. Precios actuales de la API Claude (Anthropic, jun-2026)

### 1.1 Precios base por modelo

| Modelo | Model ID | Contexto | Input $/MTok | Output $/MTok |
|--------|----------|----------|-------------|--------------|
| Haiku 4.5 | `claude-haiku-4-5` | 200K | $1.00 | $5.00 |
| Sonnet 4.6 | `claude-sonnet-4-6` | 1M | $3.00 | $15.00 |
| Sonnet 5 | `claude-sonnet-5` | 1M | $3.00 ($2.00 intro hasta 2026-08-31) | $15.00 ($10.00 intro) |
| Opus 4.6 | `claude-opus-4-6` | 1M | $5.00 | $25.00 |
| Opus 5 | `claude-opus-5` | 1M | $5.00 | $25.00 |

**Nota:** Los precios que el usuario manejo inicialmente ($0.80/$4 Haiku, $3/$15 Sonnet,
$15/$75 Opus) corresponden a modelos anteriores o estimaciones. Los precios actuales
verificados son los de arriba. Para LUKAI usaremos Haiku 4.5 y Sonnet 4.6/5 --
Opus queda descartado para chat masivo por costo prohibitivo.

### 1.2 Prompt caching

| Concepto | Costo |
|----------|-------|
| Cache write (TTL 5 min) | 1.25x del precio base de input |
| Cache write (TTL 1 hora) | 2.0x del precio base de input |
| Cache read (hit) | 0.1x del precio base de input |
| Minimo cacheable (Haiku 4.5) | 4,096 tokens |
| Minimo cacheable (Sonnet 4.6) | 1,024 tokens |

**Costos de cache por modelo (por MTok):**

| Modelo | Cache write (5min) | Cache read (hit) | Input sin cache |
|--------|-------------------|-------------------|-----------------|
| Haiku 4.5 | $1.25 | $0.10 | $1.00 |
| Sonnet 4.6 | $3.75 | $0.30 | $3.00 |

**Break-even de cache:** Con TTL 5min, 2 requests con el mismo prefijo ya compensan
(1.25x + 0.1x = 1.35x vs 2.0x sin cache). En un chat de 4 turnos, los turnos 2-4
pagan solo 0.1x por el system prompt cacheado.

---

## 2. Costo por usuario para LUKAI v2.0 (LLM)

### 2.1 Supuestos del modelo de consumo

| Parametro | Valor | Justificacion |
|-----------|-------|---------------|
| Conversaciones/dia por usuario activo | 5 | Usuario curioso explorando portafolio, Aura, misiones |
| Turnos por conversacion | 4 | Pregunta + follow-up + aclaracion + cierre |
| Tokens input por turno (usuario) | 500 | Pregunta + contexto de conversacion acumulado |
| Tokens output por turno (modelo) | 300 | Respuesta concisa sobre estado del protocolo |
| System prompt (protocolo + contexto) | ~3,000 tokens | Reglas del protocolo, estado del Vault, Aura del usuario |
| Dias activos por mes | 20 | No todos los dias, usuario promedio |

**Tokens por usuario activo por mes:**

- Turnos/mes: 5 conv/dia x 4 turnos x 20 dias = **400 turnos/mes**
- Input bruto: 400 x 500 = 200,000 tokens/mes sin cache
- Output: 400 x 300 = **120,000 tokens/mes**
- System prompt: 3,000 tokens x 400 turnos = 1,200,000 tokens nominales

**Con prompt caching (90% hit rate en system prompt):**

- System prompt: turno 1 de cada conversacion = cache write (5 conv x 20 dias = 100 writes)
- Turnos 2-4 = cache read (300 reads)
- Input sin cache (solo la pregunta del usuario): 200,000 tokens
- Cache write: 100 x 3,000 = 300,000 tokens a 1.25x
- Cache read: 300 x 3,000 = 900,000 tokens a 0.1x

### 2.2 Costo mensual por usuario activo por modelo

#### Haiku 4.5 puro (todas las queries)

| Componente | Tokens/mes | Precio/MTok | Costo/usuario/mes |
|-----------|-----------|------------|-------------------|
| Input sin cache | 200,000 | $1.00 | $0.200 |
| Cache write (5min) | 300,000 | $1.25 | $0.375 |
| Cache read | 900,000 | $0.10 | $0.090 |
| Output | 120,000 | $5.00 | $0.600 |
| **Total Haiku** | | | **$1.265** |

#### Sonnet 4.6 puro (todas las queries)

| Componente | Tokens/mes | Precio/MTok | Costo/usuario/mes |
|-----------|-----------|------------|-------------------|
| Input sin cache | 200,000 | $3.00 | $0.600 |
| Cache write (5min) | 300,000 | $3.75 | $1.125 |
| Cache read | 900,000 | $0.30 | $0.270 |
| Output | 120,000 | $15.00 | $1.800 |
| **Total Sonnet** | | | **$3.795** |

#### Routing mixto: 80% Haiku + 20% Sonnet (recomendado)

| Escenario | Costo/usuario/mes |
|-----------|-------------------|
| 80% Haiku ($1.265 x 0.80) | $1.012 |
| 20% Sonnet ($3.795 x 0.20) | $0.759 |
| **Total mixto** | **$1.771** |

### 2.3 Costo mensual total por escala de MAU

| MAU | Solo Haiku | 80/20 Haiku/Sonnet | Solo Sonnet |
|-----|-----------|-------------------|-------------|
| 1,000 | $1,265 | $1,771 | $3,795 |
| 5,000 | $6,325 | $8,855 | $18,975 |
| 10,000 | $12,650 | $17,710 | $37,950 |
| 50,000 | $63,250 | $88,550 | $189,750 |
| 100,000 | $126,500 | $177,100 | $379,500 |

### 2.4 Clasificacion de queries para routing

**Haiku 4.5 (80% del trafico -- queries simples):**
- "Cual es mi saldo de $LUKA?"
- "Cuanto Aura tengo?"
- "Que misiones de caza hay hoy?"
- "Cual es el estado del Vault?"
- "A cuanto esta $LUKA?"
- FAQ del protocolo, glosario de terminos
- Estado de staking, LP, cNFTs del usuario

**Sonnet 4.6 (20% del trafico -- queries complejas):**
- "Explicame como funciona el Motor B2 y cuando se activa"
- "Que me conviene: staking o proveer LP? Analiza mi situacion"
- "Como afecta el throttle al rendimiento de mis cNFTs?"
- Analisis de escenarios de portafolio
- Explicaciones pedagogicas del protocolo
- Estrategias personalizadas segun nivel Aura

---

## 3. Arquitectura por etapas

### 3.1 v1.0 -- Keeper Bot (TGE, sin LLM)

```
[Pyth/Switchboard Oracles] --> [Keeper Bot (Rust/TS)]
                                    |
                    +---------------+---------------+
                    |               |               |
            [Conmutacion       [Throttle      [Rebalanceo
             B0 <-> B2]       Dinamico]        Vault]
                    |               |               |
                    +-------+-------+               |
                            |                       |
                    [Solana RPC Node] <-------------+
```

**Componentes:**

| Componente | Descripcion | Costo estimado/mes |
|-----------|-------------|-------------------|
| VPS (keeper) | 2 vCPU, 4GB RAM, Ubuntu. Ejecuta cron jobs cada 30s-5min | $20--$40 |
| Solana RPC | Helius/Triton dedicado (devnet gratis, mainnet ~$50-100) | $0--$100 |
| Pyth oracle reads | Gratis (on-chain, pagan los publishers) | $0 |
| Solana tx fees | ~500-2000 tx/dia x $0.00025 = despreciable | $5--$15 |
| Monitoring (Sentry/Grafana) | Tier gratis o basico | $0--$30 |
| **Total v1.0** | | **$25--$185** |

**Mainnet realista: ~$150--$500/mes** (RPC dedicado + redundancia + alertas).

**Funciones del keeper v1.0:**
1. Monitorear K(t) via Pyth cada 30s, conmutar B0/B2 al cruzar K_min=$25M
2. Calcular regimen de mercado (EMA30 vs EMA90 BTC, vol 30d, vol Motor A 7d)
3. Ejecutar rebalanceo del Vault con nuevas entradas (35% Asset Layer)
4. Actualizar throttle dinamico segun precio vs EMA30
5. Monitorear paridad LSTs (JitoSOL/mSOL via Sanctum), bloquear si >3% desviacion
6. Gestionar R_op: detectar si USDC < 20% capacidad, activar Recarga Forzada

**Stack recomendado:**
- **Runtime:** TypeScript + @solana/web3.js (o Rust nativo si el equipo lo prefiere)
- **Scheduler:** cron systemd o node-cron, con health checks
- **Oracles:** @pythnetwork/client + Switchboard SDK
- **Alertas:** webhook a Telegram/Discord del equipo
- **Logs:** stdout a archivo rotado + Sentry para errores criticos

### 3.2 v2.0 MVP -- Asistente Conversacional (Etapa 2A)

```
[Usuario App] --> [API Gateway (rate limit)]
                       |
                 [Auth + Aura check]
                       |
              [Router de modelo]
              /                \
     [Haiku 4.5]         [Sonnet 4.6]
     (simple)            (complejo)
              \                /
          [System prompt cacheado]
          [+ Contexto del usuario]
          [+ Estado del protocolo]
                       |
              [Respuesta al usuario]
```

**Componentes y costos de infraestructura (sin LLM):**

| Componente | Descripcion | Costo/mes |
|-----------|-------------|-----------|
| API Gateway | Cloudflare Workers / Supabase Edge Functions | $0--$25 |
| Rate limiting | Redis (Upstash serverless) | $0--$10 |
| Base de datos contexto | Supabase PostgreSQL (estado usuario, Aura, historial) | $0--$25 |
| Auth | Solana wallet signature (sin costo adicional) | $0 |
| Monitoring | Sentry + metricas de uso | $0--$30 |
| **Infra v2.0 (sin LLM)** | | **$0--$90** |

**Costo total v2.0 = Infra + LLM:**

| MAU | Infra | LLM (80/20) | Total | Costo/usuario |
|-----|-------|-------------|-------|---------------|
| 1,000 | ~$50 | $1,771 | **$1,821** | $1.82 |
| 5,000 | ~$50 | $8,855 | **$8,905** | $1.78 |
| 10,000 | ~$75 | $17,710 | **$17,785** | $1.78 |
| 50,000 | ~$150 | $88,550 | **$88,700** | $1.77 |
| 100,000 | ~$300 | $177,100 | **$177,400** | $1.77 |

El LLM domina el costo a cualquier escala. La infra es despreciable.

### 3.3 v2.0 Minima Viable -- Solo Haiku con cache

La version mas barata posible de LUKAI v2.0:

- **Modelo unico:** Haiku 4.5 para todo
- **System prompt:** ~3,000 tokens con docs del protocolo, cacheado (TTL 5min)
- **Sin routing:** todo va a Haiku
- **Rate limit agresivo:** 10 mensajes/hora por usuario (vs 20 turnos/hora del modelo base)

**Costo con rate limit (2.5 conv/dia, 4 turnos, 200 turnos/mes):**

| MAU | Costo LLM/mes | Costo/usuario |
|-----|--------------|---------------|
| 1,000 | $633 | $0.63 |
| 5,000 | $3,163 | $0.63 |
| 10,000 | $6,325 | $0.63 |
| 50,000 | $31,625 | $0.63 |

Con la version minima, LUKAI v2.0 cuesta **~$0.63/usuario/mes**.

---

## 4. Analisis de break-even

### 4.1 Fuente de ingresos para LUKAI

El bucket O&M recibe el **15% de TODOS los fees** de los 4 motores.
De ese 15%, LUKAI es un gasto operativo -- no se lleva el 100%.

**Estimacion del % de O&M destinado a LUKAI:**

| Etapa | Gastos O&M ademas de LUKAI | % O&M para LUKAI | Justificacion |
|-------|---------------------------|-------------------|---------------|
| 2A (MVP) | Equipo (1-2 devs), hosting, legal | 15--25% | LUKAI es un costo menor |
| 2B (madurez) | Equipo (3-5), marketing, legal, auditorias | 10--20% | Mas gastos compitiendo |
| 3 (escala) | Equipo (5-10+), fiat infra, compliance | 5--15% | Operacion mas grande |

### 4.2 Escenarios de O&M mensual y sostenibilidad

Supuesto conservador: **20% del O&M mensual se destina a LUKAI v2.0.**

| O&M mensual total | Presupuesto LUKAI (20%) | MAU sostenibles (80/20) | MAU sostenibles (solo Haiku) |
|-------------------|------------------------|------------------------|------------------------------|
| $5,000 | $1,000 | ~560 | ~1,580 |
| $10,000 | $2,000 | ~1,130 | ~3,160 |
| $15,000 | $3,000 | ~1,690 | ~4,740 |
| $20,000 | $4,000 | ~2,260 | ~6,330 |
| $30,000 | $6,000 | ~3,390 | ~9,490 |
| $50,000 | $10,000 | ~5,650 | ~15,820 |
| $100,000 | $20,000 | ~11,290 | ~31,620 |

### 4.3 Relacion O&M vs volumen necesario

Para generar $X de O&M mensual (15% de fees totales):

| O&M mensual | Fees totales del protocolo | Volumen necesario (Motor A, 4% fee) |
|-------------|--------------------------|-------------------------------------|
| $5,000 | $33,333 | $833,333 |
| $10,000 | $66,667 | $1,666,667 |
| $20,000 | $133,333 | $3,333,333 |
| $50,000 | $333,333 | $8,333,333 |
| $100,000 | $666,667 | $16,666,667 |

**Lectura:** Para cubrir LUKAI v2.0 con 5K MAU ($8,855/mes con routing 80/20), se necesita ~$60K de O&M mensual, que requiere ~$400K de fees totales, que requiere ~$10M de volumen mensual en Motor A. Esto es realista solo en Etapa 2B avanzada o Etapa 3.

### 4.4 Punto de equilibrio

**LUKAI v2.0 se autofinancia cuando:**

| Estrategia | Costo/usuario/mes | O&M necesario (1K MAU) | O&M necesario (10K MAU) |
|-----------|-------------------|----------------------|------------------------|
| Haiku minimo (rate limited) | $0.63 | $3,150 | $31,500 |
| Haiku completo | $1.27 | $6,325 | $63,250 |
| 80/20 Haiku/Sonnet | $1.77 | $8,855 | $88,550 |

**Conclusion break-even:** Con la version minima (Haiku + rate limit), LUKAI v2.0 es sostenible desde ~$3K de O&M (para 1K MAU). Con routing completo, necesita ~$9K para 1K MAU.

---

## 5. Recomendacion

### 5.1 Cuando activar v2.0

| Condicion | Umbral | Accion |
|-----------|--------|--------|
| MAU < 500 | O&M < $5K | **No activar v2.0.** FAQ estatico + documentacion. Keeper v1.0 unico. |
| MAU 500--2,000 | O&M $5K--$15K | **v2.0 minima viable:** solo Haiku, rate limit 10msg/hora, system prompt de ~2K tokens. Costo: $315--$1,265/mes. Viable. |
| MAU 2,000--10,000 | O&M $15K--$50K | **v2.0 con routing 80/20.** Haiku para queries simples, Sonnet para complejas. Rate limits por nivel Aura. Costo: $3,540--$17,710/mes. |
| MAU > 10,000 | O&M > $50K | **v2.0 completa.** Routing inteligente, features premium para niveles Aura altos, analytics de uso. |

### 5.2 Optimizaciones de costo

**A. Prompt caching agresivo (implementar desde dia 1)**
- System prompt congelado (NO interpolar timestamps, user IDs, ni datos variables)
- Inyectar datos del usuario y estado del protocolo como mensajes, NO en el system prompt
- TTL 5min es suficiente (el usuario tipicamente hace varios turnos en <5min)
- Ahorro estimado: 60--70% en tokens de input del system prompt

**B. Rate limits por nivel Aura**

| Nivel Aura | Rate limit | Modelo disponible | Justificacion |
|-----------|-----------|-------------------|---------------|
| Cachorro (0--499) | 5 msg/hora | Solo Haiku | Nivel basico, queries simples |
| Rastreador (500--1,999) | 15 msg/hora | Haiku + Sonnet | Usuario comprometido |
| Cazador (2,000--4,999) | 30 msg/hora | Haiku + Sonnet | Usuario avanzado |
| Alfa (5,000--9,999) | 60 msg/hora | Haiku + Sonnet | Power user |
| Jaguar (10,000+) | Sin limite | Haiku + Sonnet | Recompensa maxima |

**Efecto estimado:** Los usuarios Cachorro (mayoria) consumen menos, los Jaguar (minoria) consumen mas pero son pocos. Distribucion Pareto tipica reduce el costo promedio real a ~60--70% del estimado base.

**C. Compresion de contexto**
- Resumir conversaciones anteriores en vez de enviar historial completo
- Maximo 3 turnos de historial completo, el resto resumido
- Ahorro: ~30--40% en tokens de input en conversaciones largas

**D. Batching de datos del protocolo**
- Cachear estado del Vault, Aura del usuario, etc. en Redis (TTL 30s--1min)
- Inyectar snapshot como contexto, no consultar on-chain por cada turno
- Reduce latencia y permite cachear el bloque de datos

### 5.3 Evaluacion honesta

**Lo que funciona a escala pequena:**
- v1.0 (keeper) es trivial de operar y barato. No hay riesgo aqui.
- v2.0 minima (Haiku, rate limited) cuesta ~$630/mes para 1K MAU. Cualquier protocolo DeFi funcional cubre esto con O&M.

**Lo que NO funciona a escala pequena:**
- v2.0 con routing completo para <500 MAU cuesta mas en desarrollo que en LLM. El esfuerzo de engineering (routing, clasificacion, prompt tuning, monitoring) no se justifica si hay 200 usuarios.
- Sonnet/Opus para chat masivo no es viable economicamente. Opus a $5/$25 por MTok es 5x Haiku -- para una query de portafolio simple, no hay diferencia perceptible en calidad.

**La verdad sobre el costo:**
- El costo LLM por usuario ($0.63--$1.77/mes) es bajo. El problema no es el costo unitario.
- El problema real es: (1) el costo de desarrollo y mantenimiento del sistema v2.0, y (2) que el O&M inicial probablemente sea bajo ($5K--$15K/mes).
- Con $10K de O&M y 20% para LUKAI ($2K), solo se cubren ~1,100 MAU con routing o ~3,160 con Haiku puro.
- **Si el protocolo no tiene volumen, el debate del LLM es irrelevante.** Primero hay que generar $30K+ de fees mensuales para que LUKAI v2.0 sea una decision racional.

**Recomendacion final:**
1. **Etapa 1 (TGE):** Solo keeper v1.0. Costo total ~$150--$500/mes. Sin LLM.
2. **Etapa 2A (App):** Evaluar si O&M > $10K/mes. Si si: lanzar v2.0 minima (Haiku, rate limited, $630/mes para 1K MAU). Si no: FAQ estatico.
3. **Etapa 2B (K>$25M):** Con O&M > $30K--$50K, migrar a routing 80/20 Haiku/Sonnet. Implementar limites por Aura.
4. **No construir v2.0 prematuramente.** El sistema de prompts y routing es facil de agregar despues. La inversion prematura en infra LLM es un costo de oportunidad -- ese tiempo de dev se aprovecha mejor en los smart contracts y el keeper.

---

## Apendice A: Formula de costo por usuario

```
C_usuario_mes = T * (
    P_input * tokens_input_sin_cache +
    P_cache_write * tokens_system * (1/turnos_por_conv) +
    P_cache_read * tokens_system * (1 - 1/turnos_por_conv) +
    P_output * tokens_output
)

Donde:
  T = turnos por mes (conv/dia * turnos/conv * dias)
  P_input = precio input del modelo por token
  P_cache_write = 1.25 * P_input
  P_cache_read = 0.1 * P_input
  P_output = precio output del modelo por token
  tokens_system = tokens del system prompt (~3,000)
```

## Apendice B: Comparativa de opciones LLM alternativas

| Proveedor | Modelo equivalente a Haiku | Input $/MTok | Output $/MTok | Notas |
|-----------|---------------------------|-------------|--------------|-------|
| Anthropic | Haiku 4.5 | $1.00 | $5.00 | Recomendado. Prompt caching nativo. |
| OpenAI | GPT-4o mini | ~$0.15 | ~$0.60 | Mas barato, menor calidad en espanol |
| Google | Gemini 2.0 Flash | ~$0.075 | ~$0.30 | Mas barato, menor control de cache |

**Nota:** Se recomienda Claude por: (1) prompt caching nativo bien documentado, (2) mejor desempeno en espanol/portugues, (3) coherencia de ecosistema (si LUKAI usa Claude para el orquestador interno en v2.0+). Sin embargo, para escala masiva (>50K MAU), evaluar alternativas mas baratas es prudente.

## Apendice C: Costo del keeper v1.0 desglosado

| Item | Devnet (Milestone 1-2) | Mainnet (TGE) |
|------|----------------------|---------------|
| VPS (Hetzner/DigitalOcean) | $5--$10 | $20--$40 |
| Solana RPC (Helius) | $0 (free tier) | $49--$99 (startup) |
| Pyth oracle reads | $0 | $0 |
| Switchboard reads | $0 | $0 |
| Solana tx fees | <$1 | $5--$15 |
| Sentry (monitoring) | $0 (free tier) | $0--$26 |
| Redundancia (2o VPS) | -- | $20--$40 |
| **Total** | **$5--$11** | **$94--$220** |

Con un buffer de seguridad 2x: **mainnet ~$200--$500/mes.**
