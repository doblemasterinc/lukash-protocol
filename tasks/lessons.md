# Lessons — LUKASH

> Nunca borrar. Cada lección registra qué pasó y qué haremos distinto. Patrón 2+ veces → promover a learned-rules.md.

## 2026-08-19 — El proyecto usa dos nombres para el mismo sistema de reputación
"Jaguar Score" (docs v4.2) vs "Aura" (término nuevo de Sebastián). Decisión: Aura es el rebrand oficial (ADR-002).
Lección: al haber muchas versiones de documentos, confirmar terminología con Sebastián antes de codificar.

## 2026-08-19 — Subagentes leyendo .docx/.txt gigantes se cuelgan
Las dos auditorías en background fallaron por watchdog (600s sin progreso) al leer archivos grandes en loop.
Lección: extraer a texto acotado primero; para docs ya leídos en contexto, auditar inline sin subagente.

## 2026-08-20 — El panel del navegador no compositaba → no se pudo compilar en Playground desde aquí
Screenshots/clics fallaban ("Browser pane is not displayed"). Solución: preparar single-file + guiar al usuario (compiló él con éxito). Lección: para compilar Anchor sin toolchain local, Solana Playground guiado por el usuario es la vía; hacer revisión "de escritorio" rigurosa antes (cacé el error de comentarios //! así).

## 2026-08-20 — Financiación con capital cero: NO diluir primero
Aprendizaje cross-proyecto: en Solana hay dinero no dilutivo (Superteam Earn/Instagrants, Solana Foundation + Finternet grants) y Colosseum da $250K pre-seed vía hackathon. El SAFE con ángeles es último recurso, no primero. Aplicable a cualquier proyecto cripto propio con poco capital.

## 2026-08-20 — Compensación de partners: tokens vesteados, no Vault Sociedad
Regla: tokens vesteados = pago por servicio (KOLs, MM). Vault Sociedad (equity) = solo para capital/largo plazo. No mezclar. Protege el patrimonio del fundador. (ADR-011 / decisión de diseño.)

## 2026-08-20 (sesión 2) — El "100%" del Exit Fee/Anti-Whale es el DESTINO, no la penalización
Confusión común: "100% al Vault" = todo el fee (5/3/1%) va a la reserva, NO una penalización del 100%. Aclarar siempre destino vs tasa. Anti-Whale: métrica por % de supply era brutal al inicio → corregido a % del pool + solo ventas (C10/ADR-012).

## 2026-08-20 (sesión 2) — Auditoría con capital cero: gratis primero, luego subsidio
Cross-proyecto (cualquier cripto propia): (1) herramientas gratis (Sec3 X-Ray, Trident fuzzing, clippy) bajan a la mitad el costo del audit humano; (2) **subsidio Areta $1M** para builders Solana (Colosseum = fast-track); (3) boutique $5-20K con descuento por llegar con tests; (4) Immunefi post-mainnet. NUNCA lanzar contratos que custodian fondos sin auditar en mainnet (la reserva es un honeypot).

## 2026-08-20 (sesión 3) — Faucet devnet es un cuello de botella real; separar deploy de creación de token
El faucet devnet estuvo en 429/seco todo el día (web pide GitHub con antigüedad; CLI rate-limited por IP; bug "body stream already read" del RPC default de Playground). **Deploy de programa Anchur ~2.12 SOL** (no basta 1). **Crear+acuñar un token SPL cuesta <0.1 SOL.** Lección: cuando el SOL escasea, priorizar el token (win visible barato) sobre el deploy; para más SOL usar RPC propio de Helius (evita también el bug de stream) o cambiar de IP.

## 2026-08-20 (sesión 3) — `spl-token` de Playground es viejo: sin metadata on-chain
No soporta `--enable-metadata` ni `--program-2022`. Token se crea clásico SPL; nombre/símbolo/logo requieren **Metaplex Token Metadata** aparte (metaboss local con keypair exportado, o script TS). La metadata necesita **URI pública** (imagen+JSON hosteados). Lección: para "token con nombre y logo en el explorador" no basta el CLI; planear el paso Metaplex + hosting desde el inicio.

## 2026-08-20 (sesión 3) — Economía de fundador en 3 capas (no pedirle a un vehículo lo que no da)
Cross-proyecto (cripto propia): separar (1) **O&M** = ingreso por trabajar (corto plazo), (2) **tokens de equipo** vesteados = upside del token + gobernanza (largo), (3) **equity/Vault** = patrimonio del negocio (largo). Los tokens de equipo NO son ingreso temprano (van con cliff). Un fundador con 0% de su token levanta alarma en ángeles ("sin piel en el juego"): mejor bolsa pequeña, transparente y muy vesteada. (ADR-014.)

## 2026-08-21 (sesión 4) — Copy de un proyecto cripto no debe atacar "la cripto"
Decir "la cripto es un casino" siendo nosotros cripto es autogol. El casino son los **memes** y el ruido de miles de proyectos sin valor / difíciles de entender — de ahí nos distinguimos. Regla: criticar el ruido (memes/hype), nunca la categoría a la que pertenecemos.

## 2026-08-21 (sesión 4) — Landing juvenil: nivel de detalle ≠ investor deck
En una landing de captación (público <45), el tokenomics pesado y la composición exacta del Vault abruman y confunden. Mostrar solo "qué hay dentro" (activos, no %). El detalle (proyecciones, Vault Sociedad, cómo se alinea el equipo) va en litepaper/data room. Cuidado: explicar el 2% del equipo vía "conserva parte del Vault" puede contradecir "la Reserva es común de todos".

## 2026-08-22 (sesión 6) — Verificar contra el diseño original ANTES de proponer cambios
Propuse drenar la cola en CONS/DEF como mejora obvia. Al verificar contra v2.1→v4.3, descubrí que el diseño original intencionalmente NO drena en esos modos ("bono de deflación futura"). Sebastián igual eligió el cambio (riesgo de acumulación excesiva en bears prolongados), pero la decisión fue informada. Lección: SIEMPRE verificar que un "detalle" que parece mejora no contradiga una decisión de diseño deliberada. Verificar ANTES de presentar como hecho consumado.

## 2026-08-21 (sesión 4) — Costo de IA a escala: planear antes de prometer
Una IA conversacional (LUKAI) para muchos usuarios cuesta inferencia × escala. Viable si: 80% de consultas por plantilla/reglas (sin LLM), modelos baratos (Gemini Flash/Groq) para lo abierto, caché+rate-limits, y financiado por el bucket O&M (escala con el uso). Escalonar v1 orquestador → v2 conversacional. No sobre-prometer en material público hasta tener el break-even.

## 2026-08-22 (sesión 7) — En DeFi, deuda suave = vector de ataque con wallets desechables
El diseño original de WhaleDebt (registrar fee pendiente en PDA, cobrar después) tenía un vector: crear wallets desechables, acumular deuda incobrable, y evadir el fee. En DeFi, si el cobro no es atómico (en la misma transacción), se puede evadir. Regla: fees/penalizaciones deben retenerse del monto transferido dentro del mismo tx, nunca como deuda pendiente.

## 2026-08-22 (sesión 7) — A veces el mecanismo existente ya es la respuesta
Al debatir si la exención de staking necesitaba un umbral mínimo anti-ballena, Sebastián señaló que los tiers de fee progresivos (3%/6%/10% sobre el exceso) YA son la protección contra ballenas — añadir otro gate era complejidad sin beneficio. Lección: antes de proponer protecciones adicionales, verificar si el mecanismo que ya existe cumple el mismo propósito.

## 2026-08-22 (sesión 9) — Agregar campos a una PDA Anchor requiere migración explícita
Al extender ProtocolState con 8 campos nuevos (Sprint 2, 07-a), el espacio de la PDA crece. Anchor no puede deserializar una PDA vieja con el struct nuevo (más bytes). En devnet: cerrar PDAs y re-inicializar. En mainnet: instrucción de migración con realloc. Lección: planear la estrategia de migración ANTES de agregar campos a cuentas on-chain — no es como agregar columnas a un DB, es un cambio destructivo sin realloc.

## 2026-08-25 (sesión 11) — Verificar firma COMPLETA (params + account names) antes de escribir client scripts
Al escribir el test de `process_fee` en Playground, fallé 2 veces: primero pasé 3 params en vez de 5 (`layer` y `currency` faltaban), luego usé `authority` en vez de `caller` como nombre de cuenta. Anchor da errores crípticos ("Invalid arguments: config not provided") que no apuntan al param faltante. Lección: antes de escribir CUALQUIER llamada en client.ts, grep la firma completa del Rust (todos los params con tipos) + el struct de Accounts (nombres exactos). Son 2 greps de 5 segundos que ahorran minutos de debug.

## 2026-08-25 (sesión 12) — Solana Playground Fill button genera PDAs con program ID incorrecto
El botón "Fill" de Solana Playground (beta.solpg.io) genera PDAs usando un program ID interno diferente al programa realmente desplegado. Esto causa `ConstraintSeeds` en TODAS las instrucciones con PDAs (config, state, burn_vault). Lección: NUNCA confiar en Fill para PDAs. Entrar siempre manualmente. Las PDAs correctas se obtienen de los logs de error: `ConstraintSeeds. Left: [incorrecto]. Right: [correcto]` — el "Right" es la PDA real.

## 2026-08-25 (sesión 12) — Feeds Pyth devnet son poco confiables para testing
Los feeds Pyth V2 en devnet tienen dos problemas: (1) se actualizan irregularmente (horas o días de retraso vs 60s en mainnet), y (2) el aggregate status puede no ser `Trading` (valor 1). Ambos causan falsos positivos en validaciones de frescura/calidad. Lección: para testing en devnet, relajar `ORACLE_FEED_MAX_STALENESS` a 86400s y considerar deshabilitar la validación de status. **SIEMPRE restaurar ambos guardrails antes de mainnet** (staleness=60s, status=Trading obligatorio). Marcar con comentarios `// devnet only` para no olvidar.

## 2026-08-25 (sesión 12) — Migración de PDAs con campos nuevos requiere UncheckedAccount
Al agregar 5 campos a ProtocolState (v7→v9, +40 bytes), Anchor no puede deserializar la PDA vieja con el struct nuevo (AccountDidNotDeserialize). La instrucción `close_protocol` tuvo que cambiar `state: Account<ProtocolState>` a `state: UncheckedAccount` con cierre manual: transferir lamports al authority, asignar al system_program, realloc(0). Lección: si close_protocol necesita cerrar PDAs de versiones anteriores, SIEMPRE usar UncheckedAccount — el tipo exacto del struct ya no importa al cerrar. Validar identidad por seeds solamente.

## 2026-08-22 (sesión 8) — Cada motor tiene una ventana temporal: no codificar branches para etapas imposibles
Motor C (motor=2) estaba en el branch de quema junto con Motor A. Pero Motor C solo existe en Etapa 3 (post-ENZ, supply fijo 3.3B), donde toda quema ya se detuvo. El guard ENZ siempre lo captura antes. Lección: al implementar lógica condicional por motor, verificar en QUÉ ETAPA opera cada motor — si un motor solo existe cuando una condición ya está activa (ej. ENZ), incluirlo en el branch alternativo es dead code que confunde el modelo mental. Patrón: Motor A (Etapas 1-3, quema hasta ENZ) · Motor B (Etapa 2+, B0 quema/B2 recircula) · Motor C (Etapa 3 only, post-ENZ, inyección LP) · Motor D (gradual, throttle).
