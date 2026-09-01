# LUKASH Protocol — Litepaper v1.0

> August 2026 | Data Room Document | Confidential
> Contact: Sebastián Botero Pabón · doblemaster.inc@gmail.com
> GitHub: github.com/doblemasterinc/lukash-protocol (access on request)
> Devnet Program: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`

---

## 1. Executive Summary

LUKASH is optimized monetary infrastructure for Latin America, built on Solana.

$LUKA is a deflationary transactional currency backed by the **Vault KASH Core** — a growing reserve of hard assets (BTC, SOL, LST, USDC) that is 100% on-chain and auditable 24/7. Every transaction in the ecosystem feeds the Vault, burns supply, and rewards participants.

Unlike traditional central banks that emit currency against sovereign debt (inflationary by design), LUKASH builds its reserve from real transaction volume and deflates the supply toward a fixed floor of 3.3 billion tokens. The result: a transactional currency with a **verifiable price floor** (P_KASH = Vault / Supply), transparent governance, and embedded security mechanisms that prevent the catastrophic failures common in DeFi.

The protocol targets 260 million Latin Americans who have digital bank accounts but zero access to the institutional financial infrastructure (RWA, blockchain, yield on hard assets) that the developed world already uses.

**Current state:** Smart contracts v10.2 deployed on Solana devnet (~2,100 lines Anchor/Rust). Monte Carlo v4.3 validated across 600 scenarios. 0.0% ruin risk in base and aggressive campaigns. Landing page live. Pre-TGE.

---

## 2. The Problem

Latin America has a financial paradox: digital adoption is high (200M+ bank accounts, growing mobile-first population) but access to real financial instruments is near zero. The average person under 45 in Colombia, Mexico, or Brazil:

- **Cannot** access hard-asset exposure (BTC, SOL) without navigating opaque exchanges
- **Cannot** earn yield on savings without trusting opaque intermediaries
- **Cannot** build a verifiable financial reputation across platforms
- **Loses** 5-15% of purchasing power annually to inflation with no hedge

Existing crypto products fail this audience: they speak a language nobody understands, they're designed for traders (not savers), and they offer no embedded protection against the volatility that scares first-time users away.

The problem isn't lack of money — it's lack of **access, trust, and transparency**.

---

## 3. The Solution — An Optimized Central Bank

LUKASH replicates the central bank model — a currency backed by a managed reserve — while correcting its fundamental defects:

| Dimension | Traditional Central Bank | LUKASH Protocol |
|---|---|---|
| **Issuance** | Currency emitted against sovereign debt | Fixed supply (10B), deflationary to 3.3B floor |
| **Reserve** | Built on debt and monetary policy | Built by real transactions (35% of all fees → Vault) |
| **Trend** | Inflationary by design (2-10%/yr, worse in LatAm) | Deflationary: burns reduce supply, Vault grows |
| **Price floor** | None (currency can devalue without limit) | P_KASH = Vault / Supply (auditable on-chain) |
| **Transparency** | Opaque (closed committee decisions) | Smart contract auditable, Vault verifiable 24/7 |
| **Governance** | Centralized in government institution | Guardian 2-of-3 → DAO (Stage 4) |

This is not "another DeFi protocol." It is the optimized version of monetary policy, democratized for 650 million Latin Americans who live currency devaluation as a daily experience.

---

## 4. Protocol Architecture

### 4.1 The Atom — Universal Fee Engine

Every operation in the LUKASH ecosystem — regardless of motor, layer, or asset — distributes fees atomically in a single on-chain transaction:

| Bucket | Allocation | Purpose |
|---|---|---|
| Vault / Asset Layer | 35% | Grows the hard-asset reserve |
| Burn / LP | 35% | Burns $LUKA supply + provides liquidity |
| O&M | 15% | Operational expenses, team compensation |
| Staking | 15% | Rewards for Tótem holders |

This distribution is immutable and verified by on-chain invariants.

### 4.2 The Four Motors

| Motor | Name | Asset | Fee | Stage | Purpose |
|---|---|---|---|---|---|
| **A** | El Cazador | SOL | 4% (2.5% WL) | TGE (Stage 1) | DEX trading — primary volume driver |
| **B** | El Motor Interno | $LUKA | 2.5% (1.5% WL) | Stage 2 | In-app transactions. B0=direct burn (K<$25M), B2=recirculation (K≥$25M) |
| **C** | El de Escala | USDC | 0.5% | Stage 3 | Mass payments, fiat rails |
| **D** | El Alma | Multi | 0-3.5% | Gradual | Tótems, gaming, Manadas, financial instruments |

Motor A is the economic engine. Monte Carlo simulations confirm that DEX trading volume is the #1 driver of protocol health, with a sensitivity swing of $1.29B on the 5-year Vault value.

### 4.3 Vault KASH Core

A hard-asset reserve that **only grows, is never liquidable**, and determines the price floor.

| Asset | Allocation | Rationale |
|---|---|---|
| cBTC (wrapped) | 35% | Hardest asset, long-term store of value |
| SOL | 15% | Native chain asset, staking yield |
| SOL/LST (JitoSOL, mSOL) | 20% | Liquid staking, additional yield |
| USDC (reserve) | 25% | Stability, immediate liquidity |
| USDC (lending) | 5% | Yield via Kamino/Marginfi |

100% Solana-native. Zero bridge risk. Oracles: Pyth Network + Switchboard (redundancy, 2% deviation threshold).

**Dynamic composition by market regime:** An on-chain contra-cyclical module (LUKAI) adjusts the split between volatile and stable assets based on BTC EMA30/EMA90 signals:
- BULL: 40% volatile / 60% USDC
- NEUTRAL: 75% volatile / 25% USDC
- BEAR: 70% volatile / 30% USDC (accumulate cheap assets)

Fail-safe: if the keeper doesn't update within 48h, the regime defaults to NEUTRAL.

### 4.4 Deflation Mechanism

Supply: 10,000,000,000 → floor 3,300,000,000 (burn 6.7B tokens).

- **Direct burn:** 35% of every fee (Burn/LP bucket) destroys $LUKA via CPI `token::burn` — verified on devnet
- **Deferred burn queue:** When daily burn cap (1% of supply) is hit, excess queues for later execution
- **Dynamic throttle:** Queue drains at different rates based on market conditions (ACCEL 25%/wk, NORMAL 10%, CONSERVATIVE 5%, DEFENSIVE 2%)
- **Hard-stop ENZ:** At 3.3B supply, ALL burn machinery stops permanently. Queue freezes. Supply never goes below floor.

Post-ENZ (Stage 3): $LUKA becomes a fixed-supply asset with perpetual rewards from the Vault — similar to a preferred share with real dividends.

### 4.5 KASH Shield — Embedded Security

| Mechanism | Description |
|---|---|
| **Anti-Whale** | Tiered fee on large sales (>1% of LP pool): 3%/6%/10% on excess. Sales only, not buys. |
| **KASH Exit Fee** | Dual trigger: price < 0.7×EMA30 AND sell pressure > 0.3% supply/hr. Fee: 5%/3%/1% by stage. |
| **Circuit Breaker** | Auto-pause 24h if Vault drops >10% in 1h window. Cancellable by Guardian 2-of-3. |
| **Guardian of Pause** | 2-of-3 multisig (was 3-of-3 "Tridente"). Can ONLY pause protocol — cannot move funds. Auto-sunset at Stage 3. |
| **Token-2022 Transfer Hook** | Every $LUKA transfer passes through the KASH Shield — impossible to evade by trading on another DEX. |
| **Timelock 48h** | All critical parameter changes require 48h delay. |
| **ASU (Totem Guard)** | Parametric insurance for Tótem holders. Auto-payout on confirmed exploit. Backed by external reinsurance policy. |

Exemptions from Anti-Whale/Exit Fee: internal Motor D swaps, active staking, locked LP, LP Fundador (365d lock), registered Market Makers (approved by Guardian), Aura level Titán (≥25,000).

---

## 5. Tokenomics

### 5.1 Supply Distribution

| Bucket | % | Tokens | Vesting | Notes |
|---|---|---|---|---|
| **Public (Fair-Launch)** | ≥40% | ≥4,000M | Circulating at TGE | Fair-launch, no pre-mine concentration |
| **Seed (optional)** | ≤5% | ≤500M | SAFE + token warrant, on-chain vesting | Only if grants don't cover runway. Unsold rolls to Public |
| **Vault Sociedad** | 30% | 3,000M | KASH Lock: unlocks at $30M Vault or 12mo (whichever first). Liquid from month 13, linear 48mo | Founder's equity — NOT distributed to holders |
| **Marketing / CEX** | 8% | 800M | Vested per agreement | KOLs paid in vested tokens, never cash |
| **Staking Rewards** | 10% | 1,000M | Distributed to Tótem holders via Motor D | Perpetual rewards pool |
| **Initial Liquidity** | 5% | 500M | LP-locked permanently | Meteora pool $LUKA/SOL |
| **Team / Founder** | 2% | 200M | Cliff 12mo + linear 48mo, on-chain pre-TGE | Alignment + governance + token upside |

**Total: 100% (10,000,000,000 $LUKA)**

### 5.2 Founder Economics — Three Layers

The founder's incentive structure is designed for alignment, not extraction:

1. **O&M (15% of all fees):** Operational compensation for running the protocol. Short-term income.
2. **Team/Founder (2% supply):** Token upside + DAO governance. Long-term, fully vested (cliff 12mo + 48mo linear). Published on-chain before TGE.
3. **Vault Sociedad (≥10% of 30% bucket):** Equity in the protocol's fee stream. Long-term wealth. Subject to KASH Lock.

A founder with 0% token allocation signals "no skin in the game" — uncomfortable for angels and accelerators. The 2% is small, transparent, and aggressively vested.

### 5.3 Vault Sociedad — What Investors Are Buying

The Vault Sociedad represents **30% of all protocol fees** — a growing revenue stream, not a fixed pool. It is the equity-equivalent of the protocol.

**What a seed investor gets:**
- A percentage of the Vault Sociedad (5-8% of the 30% = 1.5-2.4% of all protocol fees)
- Structured as SAFE + token warrant with on-chain vesting
- Subject to KASH Lock (unlocks at $30M Vault or 12 months, whichever first)
- Liquid from month 13, linear over 48 months

**Implied valuation:** $500K for 5-8% → ~$6-10M pre-money. This is reasonable for a protocol with:
- Live devnet deployment (not vaporware)
- Validated economics (Monte Carlo, not napkin math)
- Solo founder with AI-augmented development (lean capital structure)

**What a seed investor does NOT get:**
- Control over the Vault KASH Core (it only grows, never liquidable)
- Token supply allocation (fair-launch is preserved)
- Override on protocol parameters (governed by Timelock + Guardian + DAO)

**Planned distribution of the 30% Sociedad:**
- ~17% founder
- 5-8% seed round (angels, pre-TGE)
- 5-8% strategic round (fintech/cooperatives, month 6-12, at higher valuation)
- ~6-13% reserve for DAO/future

---

## 6. Monte Carlo Simulations — Honest Numbers

All simulations use a **contract-faithful engine**: the Python simulation replicates the exact integer arithmetic from `lib.rs` line by line. This validates the CODE, not just the design.

**MC v4.3 results (200 iterations × 3 campaigns × 5 years):**

### 6.1 Core Results (SIM 1)

| Scenario | Daily Volume | Vault Median (5yr) | Spiral Risk | B2 Activation (median) |
|---|---|---|---|---|
| **Conservative** | ~$100K-500K | $90M | **40%** | Day 1,034 |
| **Base** | ~$500K-2M | $388M | 0.0% | Day 448 |
| **Aggressive** | ~$2M-10M | $1,880M | 0.0% | Day 195 |

**The honest truth:** In the conservative scenario (low volume, weak community), there is a 40% probability of death spiral. This is why **volume is the existential risk** and why the budget allocates 30% ($150K) to marketing. The protocol's economic design is sound — but it needs volume to work.

### 6.2 Sensitivity Analysis (SIM 2)

| Factor | Vault Impact (5yr) |
|---|---|
| Trading Volume | **$1,287M** (driver #1) |
| Motor A Fee Rate | $278M |
| Vault Yield | $7M |
| App Launch Day | $2M |
| K_min Threshold | $0 (neutral) |

Volume dominates everything else by 4.6x. This confirms: the protocol's success depends on community and adoption, not on parameter tuning.

### 6.3 Stress Tests (SIM 3)

| Scenario | Vault Impact |
|---|---|
| BTC crash -80% | -3.8% (resilient) |
| Exploit 15% of Vault | -0.3% (KASH Shield absorbs) |
| LP withdrawal | -0.8% |
| Motor A volume -70% permanent | **-68%** (only real threat) |

The protocol survives market crashes. The only existential threat is sustained loss of trading volume — which is an adoption risk, not a protocol risk.

### 6.4 Market Maker Analysis (SIM 5)

| Metric | No MM | With MM |
|---|---|---|
| Vault | Baseline | +11-19% |
| Price | Baseline | -10-25% (dilution from MM inventory) |
| Conservative spiral | 40% | 3.5% |

MM improves survival in conservative scenarios but damages price via dilution. Strategy: **fair-launch without MM** (ADR-011), with contingent MM activation only if volume falls below $500K/day for 5 consecutive days (ADR-019).

---

## 7. Aura — On-Chain Financial Reputation

Aura is a non-transferable, cumulative, on-chain reputation score. It decays 2%/week after 90 days of inactivity. Level is determined by historical maximum (levels never regress).

| Level | Aura Score | Key Benefits |
|---|---|---|
| Cachorro | 0-499 | Basic access, Motor D Layer 2, Tótem Nativo Bronze |
| Rastreador | 500-1,499 | Unlock Tótem Universal, Avatar NFT mint, DAO voting |
| Cazador | 1,500-2,999 | LP rewards +12%, create Manadas, lead groups |
| Alfa | 3,000-4,999 | Advanced governance, LP Fundador +20%, x1.10 multiplier |
| Emperador | 5,000-9,999 | Anti-Whale override, x1.15, early premium access |
| Shamán | 10,000-24,999 | Premium governance, RWA access (Stage 3), x1.25 |
| Titán | 25,000+ | Total Shield exemption, found Dynasties, x1.30 |

Users earn Aura through **152 missions** across 7 trails: education (55 missions), daily engagement (12), viral marketing (42), collectibles (18), group activities (12), seasonal events (13). The cultural hook — "farm your Aura" — converges with existing internet slang, providing free marketing.

---

## 8. Product Ecosystem

### 8.1 Tótems (cNFTs)
Financial instruments as collectible compressed NFTs (~$0.001/mint). Three categories:
- **Nativo:** $LUKA deposits, 1.5% fee, no gate. Bronze/Silver/Gold tiers by amount.
- **Universal:** $LUKA deposits, 1.5% fee, requires Rastreador (Aura ≥500).
- **Estándar:** SOL/USDC deposits, 2% fee. Stage 3.

The first Nativo Tótem is gated by an educational mission — users learn before they invest.

### 8.2 Manadas
Group financial instruments created by community leaders (requires Cazador, Aura ≥1,500):
- **Vaca:** Group savings toward a goal
- **Fondo:** Peer-to-peer crowdlending
- **Negocio:** Crowdfunding
- **Evento:** On-chain verifiable tickets
- **Club:** Membership with marketplace

All fees through Motor D Layer 2 (3% $LUKA). LUKAI arbitrates disputes. The protocol never lends or takes custody.

### 8.3 Jungle Arena
Gamified mission system with real financial outcomes:
- Totem Duels (stats from real activity, $LUKA stakes)
- Territory Conquest (Manadas claim map zones)
- Lightning Prediction (daily, streak rewards)
- Battle Pass (monthly free + premium track)
- Proof of Roar (verified content creation for viral growth)

### 8.4 LUKAI
On-chain orchestrator (v1.0, TGE) + conversational AI interface (v2.0, Stage 2A).
- v1.0: Keeper functions — oracle updates, regime detection, queue drainage. Cost: $150-500/mo.
- v2.0: User-facing financial assistant. 80/20 routing (templates/Haiku for common queries, Sonnet for complex). Break-even at ~$3K O&M/month.

### 8.5 ASU (Digital Insurance)
Two-layer parametric insurance:
1. External policy (funded by O&M) covering the Vault against exploits
2. Internal product for Tótem holders — opt-in, auto-payout, micro-premium in $LUKA

The Vault KASH Core is NEVER used to cover losses. The insurance layers exist precisely to protect it.

---

## 9. Roadmap

| Stage | Timeline | Milestones |
|---|---|---|
| **Stage 0: Pre-launch** | Now - TGE | External audit, community genesis, TGE preparation |
| **Stage 1: Genesis** | TGE | Motor A active, fair-launch, Vault begins filling |
| **Stage 2A: App** | Month 2-6 | LUKASH App (payments, Tótems, Jungle Arena), Motor B0 |
| **Stage 2B: Maturity** | Month 6-18 | K reaches $25M, Motor B2 activates, LUKAI v2.0 |
| **Stage 3: Sovereignty** | Year 2+ | Motor C (fiat rails), ENZ reached, DAO preparation |
| **Stage 4: DAO** | Year 3+ | Full decentralization, Guardian sunset, community governance |

### Pre-TGE Blockers (non-negotiable)
1. Identify 2 additional Guardian signers (2-of-3 multisig)
2. Activate Guardian on-chain (one-way, irreversible)
3. Create O&M multisig (2-of-3 via Squads Protocol)
4. LP Fundador with 365d on-chain lock
5. External audit completed (via Colosseum/Areta/Superteam subsidy)
6. Devnet deployment stable 2+ weeks

---

## 10. Use of Funds ($500K Seed)

| Category | Amount | Allocation |
|---|---|---|
| **App / Tech** | $175K (35%) | External audit ($15-25K), frontend MVP, LUKAI v1.0 keeper, infrastructure |
| **Initial Liquidity** | $100K (20%) | $LUKA/SOL pool on Meteora, LP-locked permanently |
| **Marketing + Contingent MM** | $150K (30%) | KOLs (vested tokens, $0 cash), community manager, organic platforms, ads. MM contingent $40-60K (only if vol <$500K/day for 5d). CEX listing $30K |
| **Legal** | $50K (10%) | Crypto lawyer, El Salvador entity, regulatory framing |
| **Operating Reserve** | $25K (5%) | Founder O&M months 1-6, emergencies |

---

## 11. Technical Stack

- **Smart contracts:** Rust + Anchor (v10.2, ~2,100 lines, deployed on Solana devnet)
- **Token standard:** Token-2022 with Transfer Hook (mainnet). SPL classic (devnet testing)
- **Oracles:** Pyth Network (primary) + Switchboard (redundancy). Manual Pyth V2 deserialization (no `pyth-sdk-solana` dependency)
- **DEX infrastructure:** Meteora (LP), Jupiter (swaps/routing), Jito (private bundles, anti-MEV)
- **Liquid staking:** Sanctum (LST parity), JitoSOL, mSOL
- **Lending:** Kamino / Marginfi (USDC yield)
- **CI/CD:** GitHub Actions — Clippy (DeFi lints), Soteria (25+ Solana vulns), Anchor build, Anchor test
- **Off-chain cache:** Supabase PostgreSQL
- **Frontend:** TypeScript (planned)

### Devnet Deployment
- Program ID: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`
- Token $LUKA (SPL, devnet): `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`
- Config PDA: `3MqnJPy3RtUhqkTL2bmkTfp7vPHwt5ALUCg7MbcWsgPf`
- State PDA: `6bzY2xkCkkUTAwmZhVS67Jxygc5phMMUb2knWY124MWC`
- Burn Vault PDA: `7iD2hbX9FawsHLW4NyrNzzBy46qr4p3JuiEX8UNAyF2f`

Verified end-to-end flows: `update_oracle_state → refresh_vault_valuation (Pyth) → process_fee (real CPI burn + atomic distribution + pending swaps) → execute_vault_swaps (USD→native at oracle price)`.

---

## 12. Team

**Sebastián Botero Pabón** — Founder & Solo Builder
- AI-augmented development: protocol design, smart contracts, simulations, brand, pitch — all built with Claude Code as co-pilot
- Background: [details to be shared under NDA]
- Identity: Pseudonymous in public (voice, no face). Full KYC for investors and legal.

The lean team structure is intentional: a solo founder with AI tooling can move faster than a 5-person team, with lower burn rate and clearer decision-making. The $500K budget includes hiring a community manager (24/7) and contracting a frontend developer for the App MVP.

---

## 13. Legal Framework

- **Target jurisdiction:** El Salvador (CNAD — 0% crypto tax, $2K minimum capital, ~$5.5K registration)
- **Token classification:** Utility token (transactional currency that builds a reserve; reserve is NOT distributed to holders — this weakens the Howey test "expectation of profits from efforts of others")
- **Guardian design:** Cannot move funds, cannot modify parameters, cannot upgrade contracts — only pause. This further weakens the "efforts of others" prong.
- **cNFT framing:** Financial instruments with on-chain disclaimer (no guaranteed returns, no capital protection). Legal review pending.
- **Motor C (Stage 3):** Fiat rails require money transmission licensing — deferred until tracción and legal counsel.

**Pending:** Crypto lawyer consultation for cNFT and Motor C regulatory framing (budgeted in Legal $50K).

---

## 14. Risk Factors

We believe in transparent disclosure. These are the real risks:

| Risk | Severity | Mitigation |
|---|---|---|
| **Low volume (death spiral)** | HIGH | 30% budget to marketing. MM contingent gate. Motor A drives 4.6x more Vault growth than any other factor. |
| **Smart contract exploit** | HIGH | External audit pre-TGE (non-negotiable). CI pipeline. KASH Shield. ASU insurance. Vault composition diversified. |
| **Regulatory action** | MEDIUM | El Salvador jurisdiction. Utility token framing. Guardian cannot move funds. cNFTs with disclaimer. Motor C deferred. |
| **Oracle manipulation** | MEDIUM | Pyth + Switchboard redundancy. 2% confidence interval check. Circuit breaker auto-pause. |
| **Solo founder risk** | MEDIUM | AI-augmented development reduces bus factor. Code is documented. Investor can fork. DAO transition planned. |
| **LatAm adoption barriers** | MEDIUM | Trilingual (ES/EN/PT). Gamified onboarding via Jungle Arena. Tótem gated by education. Cultural hook ("farm your Aura"). |
| **Token-2022 compatibility** | LOW | Jupiter, Meteora, and major Solana infra already support Token-2022. Validated pre-TGE. |

---

## 15. Why Now

1. **Solana maturity:** Fees low enough for micro-transactions. Ecosystem (Meteora, Jupiter, Jito, Pyth) is production-ready.
2. **Cultural convergence:** "Farming Aura" is already internet slang — the brand gets free cultural distribution.
3. **LatAm moment:** Record mobile banking adoption. Stablecoin usage growing. Regulatory windows opening (El Salvador, Brazil, Colombia).
4. **AI-augmented building:** A solo founder can now build what took teams of 10 two years ago. The protocol, contracts, simulations, brand, and pitch materials were all built in 12 days with Claude Code.

---

## Appendices

### A. ADR Registry
29 Architecture Decision Records governing all protocol decisions. Available in `.claude/knowledge/key-decisions.md`. Key ADRs for investors: ADR-013 (Seed structure), ADR-014 (Team allocation), ADR-018 (Budget), ADR-019 (MM strategy), ADR-020 (Investment rounds), ADR-028 (Banco Central Optimizado narrative).

### B. Simulation Outputs
Full Monte Carlo outputs available in `simulations/out/`. Includes: SIM 0 (invariants), SIM 1 (core MC), SIM 2 (sensitivity), SIM 3 (stress), SIM 4 (throttle optimization), SIM 5 (MM vs No-MM).

### C. Audit Reports
- `audits/SECURITY_AUDIT_LIB_RS_V9_1.md` — Manual security audit (10 categories, 13 findings, all resolved)
- `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` — Contract-faithful simulation validation
- `audits/AUDITORIA_INTEGRAL_Y_VEREDICTO.md` — Integral protocol audit (8.5/10 concept, 6.5-7/10 viability with focus)
- External audit: pending (pre-TGE blocker)

### D. Protocol Specification
Full protocol v4.3 available in `docs/protocolo/LUKASH_Protocolo_v4.3.md` (~2,300 lines). Consolidated from v4.2 with 20 corrections (C1-C20).

---

*$LUKA is a utility token of the LUKASH ecosystem. The reserve backs the system; it does not constitute a promise of returns or a negotiable security. Nothing in this document is financial advice. Past simulation results do not guarantee future performance.*
