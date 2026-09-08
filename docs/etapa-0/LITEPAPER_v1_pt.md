# LUKASH Protocol — Litepaper v1.0 (Português)

> September 2026 | Data Room Document | Confidential
> Contact: Kash Sensei · kash.sensei.sol@gmail.com
> GitHub: github.com/doblemasterinc/lukash-protocol (access on request)
> Devnet Program: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`

---

## 1. Resumo Executivo

LUKASH é uma infraestrutura monetária otimizada para a América Latina, construída sobre Solana.

$LUKA é uma moeda transacional deflacionária respaldada pelo **Vault KASH Core** — uma reserva crescente de ativos duros (BTC, SOL, LST, USDC) que é 100% on-chain e auditável 24/7. Cada transação no ecossistema alimenta o Vault, queima supply e recompensa os participantes.

Ao contrário dos bancos centrais tradicionais que emitem moeda contra dívida soberana (inflacionário por natureza), o LUKASH constrói sua reserva a partir de volume real de transações e deflaciona o supply rumo a um piso fixo de 3,3 bilhões de tokens. O resultado: uma moeda transacional com um **piso de preço verificável** (P_KASH = Vault / Supply), governança transparente e mecanismos de segurança integrados que previnem as falhas catastróficas comuns no DeFi.

O protocolo tem como alvo 260 milhões de latino-americanos que possuem contas bancárias digitais mas zero acesso à infraestrutura financeira institucional (RWA, blockchain, rendimento sobre ativos duros) que o mundo desenvolvido já utiliza.

**Estado atual:** Smart contracts v10.2 implantados no Solana devnet (~2.100 linhas Anchor/Rust). Monte Carlo v4.3 validado em 7 simulações (mais de 600 cenários). 0,0% de risco de ruína nos cenários base e agressivo. Análise ROI SIM 6: retorno de 15x para investidores seed no cenário base. Landing page ativa. Pré-TGE.

---

## 2. O Problema

A América Latina tem um paradoxo financeiro: a adoção digital é alta (mais de 200M de contas bancárias, população crescente mobile-first) mas o acesso a instrumentos financeiros reais é praticamente zero. A pessoa média com menos de 45 anos na Colômbia, no México ou no Brasil:

- **Não consegue** acessar exposição a ativos duros (BTC, SOL) sem navegar corretoras opacas
- **Não consegue** obter rendimento sobre poupança sem confiar em intermediários opacos
- **Não consegue** construir uma reputação financeira verificável entre plataformas
- **Perde** de 5% a 15% do poder de compra anualmente para a inflação sem hedge

Os produtos cripto existentes falham com esse público: falam uma linguagem que ninguém entende, são projetados para traders (não poupadores), e não oferecem proteção integrada contra a volatilidade que afasta os usuários de primeira viagem.

O problema não é falta de dinheiro — é falta de **acesso, confiança e transparência**.

---

## 3. A Solução — Um Banco Central Otimizado

O LUKASH replica o modelo de banco central — uma moeda respaldada por uma reserva gerida — corrigindo seus defeitos fundamentais:

| Dimensão | Banco Central Tradicional | Protocolo LUKASH |
|---|---|---|
| **Emissão** | Moeda emitida contra dívida soberana | Supply fixo (10B), deflacionário até piso de 3,3B |
| **Reserva** | Construída sobre dívida e política monetária | Construída por transações reais (35% de todas as taxas → Vault) |
| **Tendência** | Inflacionária por natureza (2-10%/ano, pior na LatAm) | Deflacionária: queimas reduzem supply, Vault cresce |
| **Piso de preço** | Nenhum (moeda pode desvalorizar sem limite) | P_KASH = Vault / Supply (auditável on-chain) |
| **Transparência** | Opaca (decisões de comitê fechado) | Smart contract auditável, Vault verificável 24/7 |
| **Governança** | Centralizada em instituição governamental | Guardian 2-de-3 → DAO (Etapa 4) |

Isto não é "mais um protocolo DeFi." É a versão otimizada da política monetária, democratizada para 650 milhões de latino-americanos que vivem a desvalorização da moeda como experiência diária.

---

## 4. Arquitetura do Protocolo

### 4.1 O Átomo — Motor Universal de Taxas

Toda operação no ecossistema LUKASH — independentemente do motor, camada ou ativo — distribui taxas atomicamente em uma única transação on-chain:

| Bucket | Alocação | Finalidade |
|---|---|---|
| Vault / Camada de Ativos | 35% | Faz crescer a reserva de ativos duros |
| Queima / LP | 35% | Queima supply de $LUKA + fornece liquidez |
| O&M | 15% | Despesas operacionais, remuneração da equipe |
| Staking | 15% | Recompensas para detentores de Tótem |

Essa distribuição é imutável e verificada por invariantes on-chain.

### 4.2 Os Quatro Motores

| Motor | Nome | Ativo | Taxa | Etapa | Finalidade |
|---|---|---|---|---|---|
| **A** | El Cazador | SOL | 4% (2,5% WL) | TGE (Etapa 1) | Trading em DEX — principal gerador de volume |
| **B** | El Motor Interno | $LUKA | 2,5% (1,5% WL) | Etapa 2 | Transações in-app. B0=queima direta (K<$25M), B2=recirculação (K≥$25M) |
| **C** | El de Escala | USDC | 0,5% | Etapa 3 | Pagamentos massivos, trilhos fiat |
| **D** | El Alma | Multi | 0-3,5% | Gradual | Tótems, gaming, Manadas, instrumentos financeiros |

O Motor A é o motor econômico. Simulações Monte Carlo confirmam que o volume de trading em DEX é o fator #1 de saúde do protocolo, com uma oscilação de sensibilidade de $1,29B no valor do Vault em 5 anos.

### 4.3 Vault KASH Core

Uma reserva de ativos duros que **só cresce, nunca é liquidável**, e determina o piso de preço.

| Ativo | Alocação | Justificativa |
|---|---|---|
| cBTC (wrapped) | 35% | Ativo mais duro, reserva de valor de longo prazo |
| SOL | 15% | Ativo nativo da chain, rendimento de staking |
| SOL/LST (JitoSOL, mSOL) | 20% | Liquid staking, rendimento adicional |
| USDC (reserva) | 25% | Estabilidade, liquidez imediata |
| USDC (lending) | 5% | Rendimento via Kamino/Marginfi |

100% nativo de Solana. Zero risco de bridge. Oracles: Pyth Network + Switchboard (redundância, limiar de desvio de 2%).

**Composição dinâmica por regime de mercado:** Um módulo contracíclico on-chain (LUKAI) ajusta a divisão entre ativos voláteis e estáveis com base em sinais BTC EMA30/EMA90:
- BULL: 40% voláteis / 60% USDC
- NEUTRAL: 75% voláteis / 25% USDC
- BEAR: 70% voláteis / 30% USDC (acumular ativos baratos)

Fail-safe: se o keeper não atualizar dentro de 48h, o regime retorna para NEUTRAL.

### 4.4 Mecanismo de Deflação

Supply: 10.000.000.000 → piso 3.300.000.000 (queima de 6,7B tokens).

- **Queima direta:** 35% de cada taxa (bucket Queima/LP) destrói $LUKA via CPI `token::burn` — verificado no devnet
- **Fila de queima diferida:** Quando o limite diário de queima (1% do supply) é atingido, o excedente entra na fila para execução posterior
- **Throttle dinâmico:** A fila é drenada em diferentes velocidades com base nas condições de mercado (ACCEL 25%/sem, NORMAL 10%, CONSERVATIVE 5%, DEFENSIVE 2%)
- **Hard-stop ENZ:** Em 3,3B de supply, TODA a maquinaria de queima para permanentemente. A fila congela. O supply nunca cai abaixo do piso.

Pós-ENZ (Etapa 3): $LUKA se torna um ativo de supply fixo com recompensas perpétuas do Vault — similar a uma ação preferencial com dividendos reais.

### 4.5 KASH Shield — Segurança Integrada

| Mecanismo | Descrição |
|---|---|
| **Anti-Whale** | Taxa escalonada em grandes vendas (>1% do pool LP): 3%/6%/10% sobre o excedente. Apenas vendas, não compras. |
| **KASH Exit Fee** | Gatilho duplo: preço < 0,7×EMA30 E pressão de venda > 0,3% supply/h. Taxa: 5%/3%/1% por etapa. |
| **Circuit Breaker** | Pausa automática de 24h se o Vault cair >10% em janela de 1h. Cancelável pelo Guardian 2-de-3. |
| **Guardião de Pausa** | Multisig 2-de-3 (antes 3-de-3 "Tridente"). Pode APENAS pausar o protocolo — não pode mover fundos. Auto-encerramento na Etapa 3. |
| **Token-2022 Transfer Hook** | Toda transferência de $LUKA passa pelo KASH Shield — impossível evadir negociando em outra DEX. |
| **Timelock 48h** | Todas as mudanças em parâmetros críticos requerem atraso de 48h. |
| **ASU (Totem Guard)** | Seguro paramétrico para detentores de Tótem. Pagamento automático em caso de exploit confirmado. Respaldado por apólice de resseguro externa. |

Isenções do Anti-Whale/Exit Fee: swaps internos do Motor D, staking ativo, LP travado, LP Fundador (lock de 365d), Market Makers registrados (aprovados pelo Guardian), nível Aura Titán (≥25.000).

---

## 5. Tokenomics

### 5.1 Distribuição do Supply

| Bucket | % | Tokens | Vesting | Observações |
|---|---|---|---|---|
| **Público (Fair-Launch)** | ≥40% | ≥4.000M | Circulando no TGE | Fair-launch, sem concentração de pre-mine |
| **Seed (opcional)** | ≤5% | ≤500M | SAFE + token warrant, vesting on-chain | Apenas se subsídios não cobrirem runway. Não vendido retorna ao Público |
| **Vault Sociedad** | 30% | 3.000M | KASH Lock: desbloqueio em $30M de Vault ou 12 meses (o que vier primeiro). Líquido a partir do mês 13, linear em 48 meses | Equity do fundador — NÃO distribuído aos detentores |
| **Marketing / CEX** | 8% | 800M | Vesting conforme acordo | KOLs pagos em tokens com vesting, nunca em dinheiro |
| **Recompensas de Staking** | 10% | 1.000M | Distribuído aos detentores de Tótem via Motor D | Pool perpétuo de recompensas |
| **Liquidez Inicial** | 5% | 500M | LP travado permanentemente | Pool Meteora $LUKA/SOL |
| **Equipe / Fundador** | 2% | 200M | Cliff 12 meses + linear 48 meses, on-chain pré-TGE | Alinhamento + governança + upside do token |

**Total: 100% (10.000.000.000 $LUKA)**

### 5.2 Economia do Fundador — Três Camadas

A estrutura de incentivos do fundador é projetada para alinhamento, não para extração:

1. **O&M (15% de todas as taxas):** Remuneração operacional pela administração do protocolo. Renda de curto prazo.
2. **Equipe/Fundador (2% do supply):** Upside do token + governança DAO. Longo prazo, totalmente investido (cliff 12 meses + 48 meses linear). Publicado on-chain antes do TGE.
3. **Vault Sociedad (≥10% do bucket de 30%):** Equity no fluxo de taxas do protocolo. Riqueza de longo prazo. Sujeito ao KASH Lock.

Um fundador com 0% de alocação de tokens sinaliza "sem pele no jogo" — desconfortável para anjos e aceleradoras. Os 2% são pequenos, transparentes e agressivamente investidos.

### 5.3 Vault Sociedad — O Que os Investidores Estão Comprando

O Vault Sociedad representa **30% de todas as taxas do protocolo** — um fluxo de receita crescente, não um pool fixo. É o equivalente em equity do protocolo.

**O que um investidor seed recebe:**
- Uma porcentagem do Vault Sociedad, negociada por negociação
- Estruturado como SAFE + token warrant com vesting on-chain
- Sujeito ao KASH Lock (desbloqueio em $30M de Vault KASH Core ou 12 meses, o que vier primeiro)
- Líquido a partir do mês 13, linear em 48 meses

**Avaliação implícita:** $500K seed → ~$6-10M pre-money. Isto é razoável para um protocolo com:
- Implantação ativa no devnet (não vaporware)
- Economia validada (Monte Carlo, não cálculo de guardanapo)
- Fundador solo com desenvolvimento aumentado por IA (estrutura de capital enxuta)

**SIM 6 — ROI do Investidor (Monte Carlo, cenário base):**
- 6% de participação no Sociedad → **$7,5M em 5 anos (ROI de 15x)**
- IRR: 147% | Payback: ~14 meses
- Cenário conservador: ROI de 5x | Cenário agressivo: ROI de 57x

**O que um investidor seed NÃO recebe:**
- Controle sobre o Vault KASH Core (só cresce, nunca liquidável)
- Alocação de supply de tokens (fair-launch é preservado)
- Override sobre parâmetros do protocolo (governado por Timelock + Guardian + DAO)

A distribuição interna dos 30% do Sociedad (fundador, seed, operações, reserva) é estruturada conforme ADR-030 e compartilhada sob NDA durante due diligence. Limite de alocação externa: 20%.

---

## 6. Simulações Monte Carlo — Números Honestos

Todas as simulações utilizam um **motor fiel ao contrato**: a simulação em Python replica a aritmética inteira exata do `lib.rs` linha por linha. Isto valida o CÓDIGO, não apenas o design.

**Resultados MC v4.3 (200 iterações × 3 campanhas × 5 anos):**

### 6.1 Resultados Principais (SIM 1)

| Cenário | Volume Diário | Vault Mediana (5 anos) | Risco de Espiral | Ativação B2 (mediana) |
|---|---|---|---|---|
| **Conservador** | ~$100K-500K | $85M | **42%** | Dia 1.034 |
| **Base** | ~$500K-2M | $434M | 0,0% | Dia 448 |
| **Agressivo** | ~$2M-10M | $1.870M | 0,0% | Dia 195 |

**A verdade honesta:** No cenário conservador (baixo volume, comunidade fraca), há uma probabilidade de 42% de espiral da morte. É por isso que **volume é o risco existencial** e por que o orçamento aloca 30% ($150K) para marketing — e por que o ADR-030 determina um Market Maker desde o dia 1 do TGE (reduzindo a espiral conservadora de 36% para 2,5% conforme SIM 5). O design econômico do protocolo é sólido — mas precisa de volume para funcionar.

### 6.2 Análise de Sensibilidade (SIM 2)

| Fator | Impacto no Vault (5 anos) |
|---|---|
| Volume de Trading | **$1.309M** (fator #1) |
| Taxa do Motor A | $283M |
| Rendimento do Vault | $7M |
| Dia de Lançamento do App | $2M |
| Limiar K_min | $0 (neutro) |

O volume domina tudo o mais por 4,6x. Isto confirma: o sucesso do protocolo depende de comunidade e adoção, não de ajuste de parâmetros.

### 6.3 Testes de Estresse (SIM 3)

| Cenário | Impacto no Vault |
|---|---|
| Crash BTC -80% | -3,8% (resiliente) |
| Exploit de 15% do Vault | -0,3% (KASH Shield absorve) |
| Retirada de LP | -0,8% |
| Volume Motor A -70% permanente | **-68%** (única ameaça real) |

O protocolo sobrevive a crashes de mercado. A única ameaça existencial é a perda sustentada de volume de trading — que é um risco de adoção, não um risco de protocolo.

### 6.4 Análise de Market Maker (SIM 5)

| Métrica | Sem MM | Com MM |
|---|---|---|
| Vault | Baseline | +11-19% |
| Preço | Baseline | -10-25% (diluição do inventário do MM) |
| Espiral conservadora | 36% | 2,5% |
| Ativação B2 (BASE) | Dia 493 | Dia 97 |

O MM melhora dramaticamente a sobrevivência: a espiral conservadora cai de 36% para 2,5%, e o B2 é ativado 5x mais rápido no cenário base. Estratégia (ADR-030): **MM ativo desde o dia 1 do TGE**, compensado com equity do Sociedad + empréstimo de tokens (incentivo alinhado, não dinheiro). Isto substitui a estratégia anterior contingente (ADR-019).

### 6.5 ROI do Investidor (SIM 6)

| Cenário | Valor de 6% do Sociedad (5 anos) | ROI | IRR | Payback |
|---|---|---|---|---|
| **Conservador** | $2,6M | 5x | 68% | ~22 meses |
| **Base** | $7,5M | **15x** | **147%** | **~14 meses** |
| **Agressivo** | $28,5M | 57x | 312% | ~8 meses |

Uma participação de 6% no Sociedad (seed) gera $7,5M em receita acumulada de taxas ao longo de 5 anos no cenário base — um retorno de 15x sobre um investimento de $500K. O modelo assume que o Vault KASH Core alcança $25M (ativando o Motor B2) no cronograma mediano. Os cenários conservador e agressivo delimitam o intervalo.

---

## 7. Aura — Reputação Financeira On-Chain

Aura é uma pontuação de reputação on-chain não transferível e cumulativa. Ela decai 2%/semana após 90 dias de inatividade. O nível é determinado pelo máximo histórico (níveis nunca regridem).

| Nível | Pontuação Aura | Benefícios Principais |
|---|---|---|
| Cachorro | 0-499 | Acesso básico, Motor D Camada 2, Tótem Nativo Bronze |
| Rastreador | 500-1.499 | Desbloqueia Tótem Universal, mint de Avatar NFT, votação DAO |
| Cazador | 1.500-2.999 | Recompensas de LP +12%, criar Manadas, liderar grupos |
| Alfa | 3.000-4.999 | Governança avançada, LP Fundador +20%, multiplicador x1,10 |
| Emperador | 5.000-9.999 | Override Anti-Whale, x1,15, acesso antecipado premium |
| Shamán | 10.000-24.999 | Governança premium, acesso RWA (Etapa 3), x1,25 |
| Titán | 25.000+ | Isenção total do Shield, fundar Dinastias, x1,30 |

Os usuários ganham Aura através de **152 missões** em 7 trilhas: educação (55 missões), engajamento diário (12), marketing viral (42), colecionáveis (18), atividades em grupo (12), eventos sazonais (13). O gancho cultural — "farm your Aura" — converge com gíria já existente na internet, proporcionando marketing gratuito.

---

## 8. Ecossistema de Produtos

### 8.1 Tótems (cNFTs)
Instrumentos financeiros como NFTs comprimidos colecionáveis (~$0,001/mint). Três categorias:
- **Nativo:** Depósitos em $LUKA, taxa de 1,5%, sem restrição. Níveis Bronze/Silver/Gold por valor.
- **Universal:** Depósitos em $LUKA, taxa de 1,5%, requer Rastreador (Aura ≥500).
- **Estándar:** Depósitos em SOL/USDC, taxa de 2%. Etapa 3.

O primeiro Tótem Nativo é desbloqueado por uma missão educacional — usuários aprendem antes de investir.

### 8.2 Manadas
Instrumentos financeiros coletivos criados por líderes da comunidade (requer Cazador, Aura ≥1.500):
- **Vaca:** Poupança coletiva com objetivo definido
- **Fondo:** Empréstimo peer-to-peer coletivo
- **Negocio:** Crowdfunding
- **Evento:** Ingressos verificáveis on-chain
- **Club:** Assinatura com marketplace

Todas as taxas passam pelo Motor D Camada 2 (3% $LUKA). LUKAI arbitra disputas. O protocolo nunca empresta nem assume custódia.

### 8.3 Jungle Arena
Sistema de missões gamificadas com resultados financeiros reais:
- Totem Duels (estatísticas de atividade real, apostas em $LUKA)
- Territory Conquest (Manadas conquistam zonas do mapa)
- Lightning Prediction (diário, recompensas por sequência)
- Battle Pass (trilha mensal gratuita + premium)
- Proof of Roar (criação de conteúdo verificada para crescimento viral)

### 8.4 LUKAI
Orquestrador on-chain (v1.0, TGE) + interface de IA conversacional (v2.0, Etapa 2A).
- v1.0: Funções de keeper — atualizações de oracle, detecção de regime, drenagem de fila. Custo: $150-500/mês.
- v2.0: Assistente financeiro voltado ao usuário. Roteamento 80/20 (templates/Haiku para consultas comuns, Sonnet para complexas). Ponto de equilíbrio em ~$3K O&M/mês.

### 8.5 ASU (Seguro Digital)
Seguro paramétrico de duas camadas:
1. Apólice externa (financiada pelo O&M) cobrindo o Vault contra exploits
2. Produto interno para detentores de Tótem — opt-in, pagamento automático, microprêmio em $LUKA

O Vault KASH Core NUNCA é usado para cobrir perdas. As camadas de seguro existem precisamente para protegê-lo.

---

## 9. Roadmap

| Etapa | Cronograma | Marcos |
|---|---|---|
| **Etapa 0: Pré-lançamento** | Agora → TGE | Auditoria externa, gênese da comunidade, onboarding de MM, preparação para TGE |
| **Etapa 1: Gênese** | TGE | Motor A ativo, fair-launch, Vault começa a ser preenchido |
| **Etapa 2A: App** | Mês 2-6 | App LUKASH (pagamentos, Tótems, Jungle Arena), Motor B0 |
| **Etapa 2B: Maturidade** | Mês 6-18 | K alcança $25M, Motor B2 é ativado, LUKAI v2.0 |
| **Etapa 3: Soberania** | Ano 2+ | Motor C (trilhos fiat), ENZ alcançado, preparação para DAO |
| **Etapa 4: DAO** | Ano 3+ | Descentralização completa, encerramento do Guardian, governança comunitária |

### Bloqueios Pré-TGE (inegociáveis)
1. Identificar 2 signatários adicionais para o Guardian (multisig 2-de-3)
2. Ativar Guardian on-chain (ação única, irreversível)
3. Criar multisig O&M (2-de-3 via Squads Protocol)
4. LP Fundador com lock on-chain de 365 dias
5. Auditoria externa concluída (via subsídio Colosseum/Areta/Superteam)
6. Implantação no devnet estável por 2+ semanas

---

## 10. Uso dos Fundos ($500K Seed)

| Categoria | Valor | Alocação |
|---|---|---|
| **App / Tecnologia** | $175K (35%) | Auditoria externa ($15-25K), frontend MVP, LUKAI v1.0 keeper, infraestrutura |
| **Liquidez Inicial** | $100K (20%) | Pool $LUKA/SOL na Meteora, LP travado permanentemente |
| **Marketing + MM** | $150K (30%) | KOLs (tokens com vesting, $0 em dinheiro), community manager, plataformas orgânicas, anúncios. MM ativo desde o TGE (compensado com equity do Sociedad + empréstimo de tokens, não dinheiro — ADR-030). Listagem em CEX $30K |
| **Jurídico** | $50K (10%) | Advogado cripto, entidade em El Salvador, enquadramento regulatório |
| **Reserva Operacional** | $25K (5%) | O&M do fundador meses 1-6, emergências |

---

## 11. Stack Técnico

- **Smart contracts:** Rust + Anchor (v10.2, ~2.100 linhas, implantados no Solana devnet)
- **Padrão de token:** Token-2022 com Transfer Hook (mainnet). SPL classic (testes em devnet)
- **Oracles:** Pyth Network (primário) + Switchboard (redundância). Desserialização manual Pyth V2 (sem dependência de `pyth-sdk-solana`)
- **Infraestrutura DEX:** Meteora (LP), Jupiter (swaps/routing), Jito (bundles privados, anti-MEV)
- **Liquid staking:** Sanctum (paridade LST), JitoSOL, mSOL
- **Lending:** Kamino / Marginfi (rendimento USDC)
- **CI/CD:** GitHub Actions — Clippy (DeFi lints), Soteria (25+ vulnerabilidades Solana), Anchor build, Anchor test
- **Cache off-chain:** Supabase PostgreSQL
- **Frontend:** TypeScript (planejado)

### Implantação no Devnet
- Program ID: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`
- Token $LUKA (SPL, devnet): `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`
- Config PDA: `3MqnJPy3RtUhqkTL2bmkTfp7vPHwt5ALUCg7MbcWsgPf`
- State PDA: `6bzY2xkCkkUTAwmZhVS67Jxygc5phMMUb2knWY124MWC`
- Burn Vault PDA: `7iD2hbX9FawsHLW4NyrNzzBy46qr4p3JuiEX8UNAyF2f`

Fluxos verificados ponta a ponta: `update_oracle_state → refresh_vault_valuation (Pyth) → process_fee (queima real via CPI + distribuição atômica + swaps pendentes) → execute_vault_swaps (USD→nativo ao preço do oracle)`.

---

## 12. Equipe

**Kash Sensei** — Fundador & Construtor Solo
- Desenvolvimento aumentado por IA: design do protocolo, smart contracts, simulações, marca, pitch — tudo construído com Claude Code como copiloto
- Experiência: Arquiteto de soluções. Identidade completa compartilhada sob NDA.
- Identidade: Pseudônimo em público (voz, sem rosto). KYC completo para investidores e jurídico.

A estrutura enxuta da equipe é intencional: um fundador solo com ferramentas de IA pode se mover mais rápido que uma equipe de 5 pessoas, com menor taxa de queima e tomada de decisão mais clara. O orçamento de $500K inclui a contratação de um community manager (24/7) e de um desenvolvedor frontend para o MVP do App.

---

## 13. Enquadramento Jurídico

- **Jurisdição alvo:** El Salvador (CNAD — 0% de imposto sobre cripto, $2K de capital mínimo, ~$5,5K de registro)
- **Classificação do token:** Utility token (moeda transacional que constrói uma reserva; a reserva NÃO é distribuída aos detentores — isto enfraquece o teste Howey "expectativa de lucros pelo esforço de terceiros")
- **Design do Guardian:** Não pode mover fundos, não pode modificar parâmetros, não pode atualizar contratos — apenas pausar. Isto enfraquece ainda mais o critério de "esforço de terceiros".
- **Enquadramento dos cNFT:** Instrumentos financeiros com disclaimer on-chain (sem retornos garantidos, sem proteção de capital). Revisão jurídica pendente.
- **Motor C (Etapa 3):** Trilhos fiat requerem licença de transmissão de dinheiro — adiado até tração e assessoria jurídica.

**Pendente:** Consulta com advogado cripto para enquadramento regulatório de cNFT e Motor C (orçado nos $50K jurídicos).

---

## 14. Fatores de Risco

Acreditamos em divulgação transparente. Estes são os riscos reais:

| Risco | Severidade | Mitigação |
|---|---|---|
| **Baixo volume (espiral da morte)** | ALTA | 30% do orçamento em marketing. MM ativo desde o TGE (ADR-030) reduz espiral conservadora de 36% para 2,5%. Motor A gera 4,6x mais crescimento do Vault que qualquer outro fator. |
| **Exploit de smart contract** | ALTA | Auditoria externa pré-TGE (inegociável). Pipeline CI. KASH Shield. Seguro ASU. Composição do Vault diversificada. |
| **Ação regulatória** | MÉDIA | Jurisdição de El Salvador. Enquadramento como utility token. Guardian não pode mover fundos. cNFTs com disclaimer. Motor C adiado. |
| **Manipulação de oracle** | MÉDIA | Redundância Pyth + Switchboard. Verificação de intervalo de confiança de 2%. Circuit breaker com pausa automática. |
| **Risco de fundador solo** | MÉDIO | Desenvolvimento aumentado por IA reduz fator de ônibus. Código documentado. Investidor pode fazer fork. Transição para DAO planejada. |
| **Barreiras de adoção na LatAm** | MÉDIA | Trilíngue (ES/EN/PT). Onboarding gamificado via Jungle Arena. Tótem protegido por educação. Gancho cultural ("farm your Aura"). |
| **Compatibilidade Token-2022** | BAIXA | Jupiter, Meteora e a infraestrutura principal de Solana já suportam Token-2022. Validado pré-TGE. |

---

## 15. Por Que Agora

1. **Maturidade da Solana:** Taxas baixas o suficiente para microtransações. Ecossistema (Meteora, Jupiter, Jito, Pyth) pronto para produção.
2. **Convergência cultural:** "Farming Aura" já é gíria da internet — a marca recebe distribuição cultural gratuita.
3. **O momento da LatAm:** Adoção recorde de mobile banking. Uso de stablecoins crescendo. Janelas regulatórias se abrindo (El Salvador, Brasil, Colômbia).
4. **Construção aumentada por IA:** Um fundador solo agora consegue construir o que equipes de 10 levavam dois anos. O protocolo, contratos, simulações, marca e materiais de pitch foram todos construídos em 12 dias com Claude Code.

---

## Apêndices

### A. Registro de ADRs
30 Architecture Decision Records governando todas as decisões do protocolo. Disponíveis em `.claude/knowledge/key-decisions.md`. ADRs principais para investidores: ADR-013 (estrutura Seed), ADR-014 (alocação da equipe), ADR-018 (orçamento), ADR-030 (estrutura do Sociedad + estratégia de MM), ADR-028 (narrativa do Banco Central Otimizado).

### B. Resultados das Simulações
Resultados completos do Monte Carlo v4.3 disponíveis em `simulations/out/`. 7 simulações: SIM 0 (invariantes), SIM 1 (MC principal), SIM 2 (sensibilidade), SIM 3 (estresse), SIM 4 (otimização de throttle), SIM 5 (impacto do MM), SIM 6 (ROI do investidor). Relatório HTML + PDF em `simulations/out/informe_simulaciones_v4.3.*`.

### C. Relatórios de Auditoria
- `audits/SECURITY_AUDIT_LIB_RS_V9_1.md` — Auditoria de segurança manual (10 categorias, 13 achados, todos resolvidos)
- `audits/VALIDACION_MOTORES_SIM_CONTRACT_FAITHFUL.md` — Validação da simulação fiel ao contrato
- `audits/AUDITORIA_INTEGRAL_Y_VEREDICTO.md` — Auditoria integral do protocolo (8,5/10 conceito, 6,5-7/10 viabilidade com foco)
- Auditoria externa: pendente (bloqueio pré-TGE)

### D. Especificação do Protocolo
Protocolo completo v4.3 disponível em `docs/protocolo/LUKASH_Protocolo_v4.3.md` (~2.300 linhas). Consolidado a partir da v4.2 com 20 correções (C1-C20).

---

*$LUKA é um utility token do ecossistema LUKASH. A reserva respalda o sistema; ela não constitui promessa de retornos nem título negociável. Nada neste documento constitui aconselhamento financeiro. Resultados de simulações passadas não garantem desempenho futuro.*
