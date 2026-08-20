use anchor_lang::prelude::*;

/// Parámetros gobernados del protocolo. Cambian solo vía Timelock 48h + autoridad (Tridente Multisig).
#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    pub authority: Pubkey, // Tridente Multisig
    pub stage: u8,         // 1=Génesis, 2=App (2A/2B según motor_b_state), 3=Soberanía, 4=DAO
    pub paused: bool,      // Circuit Breaker global

    // Umbrales de valor (KASH Core, USD 6 dec)
    pub k_min_usd: u64,
    pub jaguar_lock_usd: u64,
    pub etapa3_usd: u64,

    // Composición objetivo del Vault Core (bps, debe sumar 10000)
    pub vault_cbtc_bps: u64,
    pub vault_sol_bps: u64,
    pub vault_lst_bps: u64,
    pub vault_usdc_res_bps: u64,
    pub vault_usdc_lend_bps: u64,

    // Split de Fee del Asset Layer (Core/Sociedad)
    pub fee_split_core_bps: u64,

    // Timelock
    pub timelock_seconds: i64,

    // Cambio de parámetro encolado (scaffold: soporta stage y k_min)
    pub pending_kind: u8,          // 0=ninguno, 1=stage, 2=k_min, 3=authority
    pub pending_value: u64,        // nuevo valor (para stage/k_min)
    pub pending_pubkey: Pubkey,    // nueva autoridad (para kind=3)
    pub pending_execute_after: i64,

    pub bump: u8,
}

/// Estado de ejecución (runtime). Lo actualizan las instrucciones de fees y el orquestador (keeper).
#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    pub genesis_ts: i64,

    // Motor B y Throttle
    pub motor_b_state: u8, // 0=B0, 1=B2
    pub throttle_mode: u8, // 0=ACEL,1=NORMAL,2=CONS,3=DEF

    // Oráculo (USD 6 dec por token, escalado)
    pub luka_price: u64,
    pub ema30: u64,

    // Vault
    pub vault_core_usd: u64,     // = K(t), la métrica de salud
    pub vault_sociedad_usd: u64,
    pub r_op_usd: u64,
    pub cbtc_usd: u64,
    pub sol_usd: u64,
    pub lst_usd: u64,
    pub usdc_res_usd: u64,
    pub usdc_lend_usd: u64,

    // Acumuladores de flujo
    pub burned_total: u64,
    pub recirculated_total: u64,
    pub staking_total: u64,
    pub om_total: u64,
    pub deferred_burn_queue: u64,

    // Hitos
    pub jaguar_lock_hit: bool,
    pub last_queue_exec_ts: i64,

    pub bump: u8,
}

/// Resultado de una distribución 35/35/15/15 (para eventos y verificación).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct Distribution {
    pub fee: u64,
    pub to_vault: u64,
    pub to_lp_burn: u64,
    pub to_om: u64,
    pub to_staking: u64,
}
