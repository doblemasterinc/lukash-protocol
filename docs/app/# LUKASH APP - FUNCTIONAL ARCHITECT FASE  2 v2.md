## MÓDULO: DUAL_FINANCIAL_ROUTER
- **IF (Transaction_Type == P2P_Transfer OR Market_Purchase):**
    - [cite_start]Execute `Apply_Stage_Fee` (Fase 1 Table)[cite: 69, 120].
    - [cite_start]IF `input_asset != $LUKA`: Execute `Atomic_Swap` + `Apply_Stage_Fee` (Layer 1).
- **IF (Transaction_Type == Service_Payment OR App_Revenue):**
    - [cite_start]Execute `Apply_30/30/40_Split`[cite: 70, 130].

## MÓDULO: MARKETPLACE_DYNAMICS
- **Product_Market:** Set 3% commission on sales -> Route to `30/30/40_Split`.
- **Event_Market:** - `Success_Fee` (3%) & `Inventory_Fee` (1.5%) -> Route to `30/30/40_Split`.
- **Bets_Rake:** 3% on winner pot -> Route to `30/30/40_Split`.

## MÓDULO: ISO_20022_INTEGRATION
- [cite_start]**Requirement:** Inject XML structured data in `memo` field of ALL Solana transactions[cite: 90, 171].
- [cite_start]**Data:** `sender_id`, `receiver_id`, `amount`, `purpose_code`, `timestamp`[cite: 92, 171].