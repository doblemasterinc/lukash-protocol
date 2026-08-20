# LUKASH APP - FUNCTIONAL ARCHITECTURE (FINAL)

## 1. CORE TRANSACTIONAL LOGIC
- **P2P/Social Features (Vacas, Apuestas):** Apply `Stage_Fee` (Genesis: 5.5%, etc.) on `Deposit` and `Withdraw` actions.
- **AI Services (LUK Pro/Fun), Gaming Actions (Jungle Arena Energy), Rake:** Apply `30/30/40_Split` on 100% of the payment amount.
- **Batch Processing:** Accumulate fees/splits in `Holding_Vault` and process on-chain when `Threshold_Value` (e.g., 20 SOL equivalent) is met.

## 2. CNFT & GAMING ENGINE (JUNGLE ARENA)
- **CNFT Minting:** Automatically mint a new cNFT for each new user account.
- **CNFT Evolution Trigger:** Update cNFT metadata (image, level, stats) based on `User_Activity_Score` (derived from `transaction_volume`, `LUKA_hold_duration`, `AI_usage_count`).
- **Energy System:**
    - Implement `Energy` as an internal, non-transferable token/credit within the game contract.
    - `Energy_Purchase` function: Users pay $LUKA, which triggers `30/30/40_Split`.
    - `Action_Cost`: Each game action (attack, defend, move) deducts `Energy`.
- **Squads & Staking:**
    - `Squad_Creation` & `Join_Squad` functions.
    - `Battle_Stake` function: Requires locking $LUKA. `Stage_Fee` is applied.
- **Reward Distribution:**
    - `Prey_Drop` function: LUK AI identifies social challenges or geo-coordinates for reward drops (from Marketing/Staking pools).
    - `Reward_Claim` function: Successful completion of `Raid` by `Squad` triggers reward distribution. A small `Rake` (e.g., 2-5%) on claimed rewards applies `30/30/40_Split`.

## 3. LUK AI INTEGRATION
- **Service Request Function:** `LUK_AI_Service(service_type, user_prompt, payment_amount)`. Triggers `30/30/40_Split`.
- **ISO 20022 Parsing:** LUK AI module must have capabilities to parse XML data from `memo` fields for `LUK_Pro` financial reports.

## 4. ISO 20022 COMPLIANCE
- **Memo Extension:** All SPL transfers initiated via LUKASH App must automatically embed a standardized XML string into the `memo` field of the Solana transaction.
    - **Structure:** Follows ISO 20022 guidelines for payment messages (e.g., `pain.001.001.xx` or `pacs.008.001.xx` where applicable).
    - **Data Points:** Includes `sender_id`, `receiver_id`, `amount`, `currency`, `purpose_code`, `timestamp`.
    - **Purpose:** Platform compliance for future institutional integration and enabling `LUK_Pro` features.

## 5. SECURITY & OPTIMIZATION
- **Anti-Whale Tax (20%):** On sell/transfer > 0.5% Circulating Supply. Excess to `Vault_KASH_Core`.
- **Anti-Bot Cooldown (60s):** For sell transactions.
- **Slippage Guard (1%):** For all swaps.
- **Oracle Integration:** `Pyth_Network` for real-time `Vault_TVL` calculation and `Oracle_Bets` resolution.
- **Multisig Control:** For `Marketing_Vault`, `O&M_Vault`, and `Staking_Vault` withdrawals.