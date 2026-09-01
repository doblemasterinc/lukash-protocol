# LUKASH — Pitch Coaching & Preparation

> Preparado: 2026-08-31. Para Sebastián Botero Pabón.
> Contexto: pre-seed $500K, target = ángeles Solana + grants + aceleradoras.

---

## 1. Estructura del Pitch (15 minutos máx)

| Tiempo | Sección | Qué decir | Qué NO decir |
|--------|---------|-----------|--------------|
| 0:00-0:30 | Hook | "260M Latin Americans have digital accounts but zero access to real financial instruments. We're building the infrastructure to fix that — on Solana." | No empezar con "Hi, my name is..." ni con la historia del token |
| 0:30-2:00 | Problema | La brecha real: inflación LatAm, exclusión financiera, productos opacos y caros. Dato duro: solo 12% de LatAm usa algún instrumento de ahorro formal. | No hablar de crypto como solución genérica. No decir "bank the unbanked" (gastado) |
| 2:00-4:00 | Solución | "LUKASH es infraestructura monetaria optimizada." La tabla Banco Central Optimizado vs Banco Central Tradicional. El Vault crece, el supply baja, el piso sube. | NUNCA decir "DeFi protocol". No prometer retornos. No comparar con shitcoins |
| 4:00-6:00 | Cómo funciona | Los 4 motores → fee atómica 35/35/15/15. Mostrar el diagrama del átomo. "Cada transacción hace 4 cosas a la vez, en una sola instrucción on-chain." | No entrar en detalles técnicos de Anchor/Rust a menos que pregunten |
| 6:00-8:00 | Tracción | "No es un whitepaper — los contratos están en devnet." Mostrar el Program ID en Solana Explorer. Monte Carlo: 0.0% ruin risk. 3 auditorías internas. | No decir "estamos en stealth". No inflar métricas |
| 8:00-10:00 | Aura + Engagement | El hook: "farmear aura" ya es cultura. Nosotros lo hacemos real y on-chain. Mostrar el prototipo con las misiones. Las Manadas = vaquitas reimaginadas. | No hacer demo larga. 2-3 pantallas máximo del prototipo |
| 10:00-12:00 | Modelo de negocio | Ecosistema autofinanciado por fases. O&M del 15% de cada transacción. No dependemos de market makers ni de token price para operar. | No entrar en tokenomics detallados a menos que pregunten |
| 12:00-13:00 | El ask | "$500K pre-seed. External audit + TGE + community Genesis. SAFE + token warrant sobre fee stream del Vault Sociedad. No diluimos supply." | No dar valoración a menos que pregunten. Si preguntan: "$6-10M implied" |
| 13:00-15:00 | Q&A | Responder con datos. Tener el litepaper listo para enviar después. | No inventar respuestas. "That's a great question, let me get you the exact number after this call" es válido |

---

## 2. Las 10 preguntas que VAN a hacer (y cómo responder)

### "What's your competitive advantage?"
**Respuesta:** "Three things. First, we're the only project building complete monetary infrastructure on Solana for LatAm — not a DEX, not a lending protocol, but the full stack from transactional currency to reserve to reputation. Second, the self-reinforcing loop: every transaction simultaneously feeds the reserve, burns supply, and builds user reputation. Third, cultural alignment — Manadas are Latin America's traditional saving circles reimagined on-chain. We're not importing a Silicon Valley product; we're digitizing something people already do."

### "Why Solana and not [X]?"
**Respuesta:** "Fees and speed. Our target user sends $20 — a $0.50 gas fee kills the product. Solana gives us sub-cent transactions at 400ms finality. Plus the ecosystem is mature: we use Meteora for LP, Jupiter for routing, Jito for anti-MEV bundles, Pyth for oracles. We'd have to build 80% of this from scratch on any other chain."

### "How do you acquire users?"
**Respuesta:** "Three channels. First, the Aura system gamifies financial participation — completing missions, maintaining streaks, building reputation. It turns 'opening a financial app' into something a 19-year-old wants to do daily. Second, Manadas: group savings circles are deeply cultural in LatAm — we digitize them with shared vaults and collective Tótem NFTs. Third, our community rewards scale: each referral that reaches Rastreador level (500 Aura) earns the referrer 40 Aura. Network effects built into the protocol."

### "What if the token price crashes?"
**Respuesta:** "The KASH Shield has six layers of protection: circuit breakers halt trading if price drops 20% in 15 minutes, exit fees discourage panic selling, anti-whale limits prevent single-entity dumps, the Token-2022 transfer hook enforces all of this at the protocol level, and the 2-of-3 Guardian multisig can pause everything in an emergency. Plus, the Vault itself is a price floor: P_KASH = Vault value / circulating supply. The vault only grows, supply only decreases. The floor goes up structurally."

### "What's your team?"
**Respuesta:** "Right now it's me, building solo with AI-augmented development. That's a feature, not a bug — the entire protocol architecture, 2,100 lines of Anchor code, Monte Carlo simulations, brand design, and community system were built by one person in weeks, not months. The $500K includes hiring: a senior Solana engineer and a growth lead for LatAm. I'm looking for the right co-builder, not just any hire."

### "You're a solo founder — isn't that a risk?"
**Respuesta:** "Fair concern. Two things. First, everything is code — it's auditable, testable, on-chain. This isn't a vision deck; the contracts work on devnet right now. Second, the $500K specifically addresses this: senior Solana engineer + growth lead. I need the capital to build the team, and I need the team to justify the capital. That's exactly what pre-seed is for."

### "What's your valuation?"
**Respuesta:** (solo si preguntan) "$500K for 5-8% of the Vault Sociedad fee stream — that implies $6-10M. We're not selling tokens. Investors get a perpetual share of the protocol's operational revenue, structured as a SAFE with token warrant."

### "How is this different from [competitor]?"
**Respuesta:** "Most 'LatAm fintech' projects are either (a) dollar-pegged stablecoins — which solve volatility but give zero upside, or (b) DeFi protocols optimized for crypto-natives in the US. LUKASH is neither. We're building deflationary monetary infrastructure with a built-in reserve that acts as a price floor — plus a cultural engagement layer (Aura, Manadas, Tótems) designed for LatAm's actual population. We don't compete with Nubank or Mercado Pago — we complement them by providing the hard-asset backbone they can't offer."

### "What's your regulatory strategy?"
**Respuesta:** "$LUKA is a utility token — it's transactional currency within the ecosystem, not a security. The reserve backs the system; it is not distributed to holders. We include an on-chain disclaimer on every cNFT. No promises of returns anywhere in the protocol, the app, or the marketing. We designed this with ADR-009 and ADR-010 specifically to stay clean. Our legal framework follows the Howey test criteria: no expectation of profits derived from the efforts of others."

### "What happens if you don't raise?"
**Respuesta:** "We keep building. The protocol is designed to be self-funded after TGE — the 15% O&M fee sustains operations. Without external capital, the timeline stretches from 6 months to 18 months for the same milestones. The raise accelerates us; it doesn't determine whether we exist."

---

## 3. Consejos de Delivery (cómo presentar)

### Para llamada (Zoom/Google Meet)
- **Cámara encendida siempre.** Background limpio (pared neutra o virtual simple).
- **Compartir pantalla solo cuando muestres algo.** No compartir desde el inicio — el primer minuto es cara a cara.
- **Tener abiertos antes de la llamada:** Solana Explorer con el Program ID, el prototipo v2, el pitch deck.
- **Ritmo:** habla con calma. Los nervios aceleran. Si sientes que vas rápido, para, respira, sigue.
- **Duración:** pide 15 minutos. Si se extiende a 30, es buena señal. No pases de 30 sin que ellos lo pidan.

### Para pitch presencial
- **Lleva laptop con todo precargado** (sin depender de Wi-Fi para mostrar devnet).
- **One-pager impreso** (3-5 copias). El litepaper solo en digital.
- **No leas slides.** Cuenta la historia: "260M de personas en LatAm no tienen acceso a... nosotros construimos..."
- **Contacto visual.** Con el decisor, no con la pantalla.
- **Cierra pidiendo algo concreto:** "¿Puedo enviarte el litepaper y agendar una segunda conversación?"

### Errores a evitar
1. **No digas "somos como [X] pero para LatAm."** Es la muerte de un pitch. Eres LUKASH, punto.
2. **No te disculpes por ser solo.** Di "I built this" con orgullo. Los inversores respetan builders.
3. **No hables de precio del token.** Si preguntan: "The KASH price is a verifiable floor based on reserve divided by supply — it's structural, not speculative."
4. **No prometas fechas exactas de mainnet.** Di: "After the external audit passes, which takes 4-8 weeks."
5. **No hables mal de competidores.** Di: "Different approach" o "complementary".
6. **No uses buzzwords vacías:** "revolutionary", "disruptive", "game-changer". Deja que los números hablen.

---

## 4. Email dedicado para LUKASH

**Recomendación: SÍ, créalo antes de enviar cualquier outreach.**

Opciones:
- `lukash.protocol@gmail.com` (simple, profesional)
- `hello@lukash.io` (si compras dominio — recomendado a futuro)
- `sebastian@lukash.io` (el más profesional, requiere dominio)

**Por qué:**
- Separación clara entre personal y proyecto
- Los grants y fondos notan la profesionalidad de un email dedicado
- Permite configurar firma con logo y links
- Si el proyecto crece, ya tienes el canal institucional listo

**Configuración sugerida de la firma:**
```
Sebastián Botero Pabón
Founder · LUKASH Protocol
lukash.protocol@gmail.com
🐆 Dueño de tu ascenso
```

**NO usar el email personal (doblemaster.inc@gmail.com) para outreach de inversores.**
El personal está bien para GitHub y cuentas de desarrollo, pero para inversores
y grants necesitas parecer establecido.

---

## 5. Checklist pre-pitch

- [ ] Email dedicado creado y configurado
- [ ] Pitch deck v4.3 en PDF, listo para adjuntar
- [ ] One-pager en EN, ES, PT — PDFs listos
- [ ] Litepaper en PDF, listo para data room
- [ ] Prototipo v2 accesible via URL (o HTML local para demo)
- [ ] Solana Explorer abierto con Program ID
- [ ] GitHub repo listo para dar acceso (si lo piden)
- [ ] Landing page actualizada (sin imágenes de "jaguar" en texto)
- [ ] Practicar el pitch 3 veces cronometrado (≤15 min)
- [ ] Respuestas a las 10 preguntas memorizadas (no de memoria — internalizadas)
