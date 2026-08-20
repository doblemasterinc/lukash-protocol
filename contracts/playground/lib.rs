// LUKASH Protocol - Milestone 1 (version de un solo archivo para Solana Playground)
// Pegar este archivo COMPLETO en src/lib.rs de un proyecto Anchor en beta.solpg.io y darle Build.

use anchor_lang::prelude::*;

// ================= constantes =================
// Constantes del protocolo LUKASH (valores de lanzamiento del Blueprint v4.3 §13).
// Todos los montos USD usan 6 decimales (1 USD = 1_000_000).

// ---- Seeds de PDAs ----
pub const CONFIG_SEED: &[u8] = b"config";
pub const STATE_SEED: &[u8] = b"state";

// ---- Escala ----
pub const BPS_DENOMINATOR: u64 = 10_000; // 100.00%
pub const USD_DECIMALS: u32 = 6;

// ---- Distribución universal 35/35/15/15 (suma 10000) ----
pub const DIST_VAULT_BPS: u64 = 3_500;
pub const DIST_LP_BURN_BPS: u64 = 3_500;
pub const DIST_OM_BPS: u64 = 1_500;
pub const DIST_STAKING_BPS: u64 = 1_500;

// ---- Split de Fee del Asset Layer: Core / Sociedad (70/30) ----
pub const FEE_SPLIT_CORE_BPS: u64 = 7_000;
pub const FEE_SPLIT_SOCIEDAD_BPS: u64 = 3_000;

// ---- Composición objetivo del Vault Core (suma 10000) ----
pub const VAULT_CBTC_BPS: u64 = 3_500;
pub const VAULT_SOL_BPS: u64 = 1_500;
pub const VAULT_LST_BPS: u64 = 2_000;
pub const VAULT_USDC_RES_BPS: u64 = 2_500;
pub const VAULT_USDC_LEND_BPS: u64 = 500;

// ---- Umbrales de valor (sobre el KASH Core), USD 6 dec ----
pub const K_MIN_USD: u64 = 25_000_000_000_000; // $25M
pub const JAGUAR_LOCK_USD: u64 = 30_000_000_000_000; // $30M
pub const ETAPA3_USD: u64 = 50_000_000_000_000; // $50M
pub const SCALE_USD: u64 = 100_000_000_000_000; // $100M

// ---- Jaguar Lock por tiempo: 12 meses ----
pub const JAGUAR_LOCK_SECONDS: i64 = 365 * 24 * 60 * 60;

// ---- Throttle: umbrales como % de la EMA30 (bps) y % de quema (bps) ----
pub const THROTTLE_ACCEL_BPS: u64 = 12_000; // P > 1.2x EMA30
pub const THROTTLE_CONS_BPS: u64 = 8_000; // P < 0.8x EMA30
pub const THROTTLE_DEF_BPS: u64 = 5_000; // P < 0.5x EMA30

pub const BURN_ACCEL_BPS: u64 = 12_500; // 125%
pub const BURN_NORMAL_BPS: u64 = 10_000; // 100%
pub const BURN_CONS_BPS: u64 = 6_000; // 60%
pub const BURN_DEF_BPS: u64 = 2_500; // 25%

// ---- Timelock de gobernanza ----
pub const TIMELOCK_SECONDS: i64 = 48 * 60 * 60; // 48h

// ---- Cola de quema diferida ----
pub const WEEK_SECONDS: i64 = 7 * 24 * 60 * 60; // 604800
pub const QUEUE_DRAIN_BPS: u64 = 1_000; // 10% de la cola por semana

// ---- Modos del Motor B ----
pub const MOTOR_B_B0: u8 = 0;
pub const MOTOR_B_B2: u8 = 1;

// ---- Modos del Throttle ----
pub const THROTTLE_ACCELERATED: u8 = 0;
pub const THROTTLE_NORMAL: u8 = 1;
pub const THROTTLE_CONSERVATIVE: u8 = 2;
pub const THROTTLE_DEFENSIVE: u8 = 3;


// ================= errores =================

#[error_code]
pub enum LukashError {
    #[msg("Overflow aritmético")]
    MathOverflow,
    #[msg("La distribución no suma el monto del fee (invariante 35/35/15/15 roto)")]
    DistributionInvariant,
    #[msg("Motor o capa inválidos")]
    InvalidMotorOrLayer,
    #[msg("El motor no está activo en la etapa actual del protocolo")]
    MotorNotActiveInStage,
    #[msg("Etapa del protocolo inválida")]
    InvalidStage,
    #[msg("Solo la autoridad (Tridente Multisig) puede ejecutar esto")]
    Unauthorized,
    #[msg("El protocolo está en pausa (Circuit Breaker)")]
    ProtocolPaused,
    #[msg("No hay un cambio de parámetro encolado")]
    NoPendingChange,
    #[msg("El Timelock de 48h aún no ha transcurrido")]
    TimelockNotElapsed,
    #[msg("El Motor B ya está en estado B2")]
    AlreadyB2,
    #[msg("K(t) aún no alcanza K_min; no se puede conmutar a B2")]
    KminNotReached,
    #[msg("La composición del Vault no suma 100% (10000 bps)")]
    VaultCompositionInvalid,
    #[msg("Monto inválido (cero)")]
    ZeroAmount,
    #[msg("El precio no está normalizado (Throttle no NORMAL/ACELERADO)")]
    ThrottleNotNormalized,
    #[msg("Aún no ha pasado una semana desde la última ejecución de la cola")]
    QueueCooldown,
    #[msg("No hay quemas diferidas en cola")]
    EmptyQueue,
}

// ================= estado (cuentas) =================

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

// ================= programa =================
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lukash_protocol {
    use super::*;

    /// Inicializa el protocolo. La autoridad (firmante) representa al Tridente Multisig.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Validación: la composición del Vault debe sumar 100% (10000 bps).
        let vault_sum = VAULT_CBTC_BPS + VAULT_SOL_BPS + VAULT_LST_BPS + VAULT_USDC_RES_BPS + VAULT_USDC_LEND_BPS;
        require!(vault_sum == BPS_DENOMINATOR, LukashError::VaultCompositionInvalid);
        // Validación: la distribución debe sumar 100%.
        require!(
            DIST_VAULT_BPS + DIST_LP_BURN_BPS + DIST_OM_BPS + DIST_STAKING_BPS == BPS_DENOMINATOR,
            LukashError::DistributionInvariant
        );

        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.stage = 1; // Génesis
        config.paused = false;
        config.k_min_usd = K_MIN_USD;
        config.jaguar_lock_usd = JAGUAR_LOCK_USD;
        config.etapa3_usd = ETAPA3_USD;
        config.vault_cbtc_bps = VAULT_CBTC_BPS;
        config.vault_sol_bps = VAULT_SOL_BPS;
        config.vault_lst_bps = VAULT_LST_BPS;
        config.vault_usdc_res_bps = VAULT_USDC_RES_BPS;
        config.vault_usdc_lend_bps = VAULT_USDC_LEND_BPS;
        config.fee_split_core_bps = FEE_SPLIT_CORE_BPS;
        config.timelock_seconds = TIMELOCK_SECONDS;
        config.pending_kind = 0;
        config.pending_value = 0;
        config.pending_pubkey = Pubkey::default();
        config.pending_execute_after = 0;
        config.bump = ctx.bumps.config;

        let state = &mut ctx.accounts.state;
        state.genesis_ts = now;
        state.motor_b_state = MOTOR_B_B0;
        state.throttle_mode = THROTTLE_NORMAL;
        state.luka_price = 0;
        state.ema30 = 0;
        state.vault_core_usd = 0;
        state.vault_sociedad_usd = 0;
        state.r_op_usd = 0;
        state.cbtc_usd = 0;
        state.sol_usd = 0;
        state.lst_usd = 0;
        state.usdc_res_usd = 0;
        state.usdc_lend_usd = 0;
        state.burned_total = 0;
        state.recirculated_total = 0;
        state.staking_total = 0;
        state.om_total = 0;
        state.deferred_burn_queue = 0;
        state.jaguar_lock_hit = false;
        state.last_queue_exec_ts = now;
        state.bump = ctx.bumps.state;

        Ok(())
    }

    /// Universal Fee Extractor: calcula el fee de la transacción y lo distribuye atómicamente 35/35/15/15.
    /// `amount` = notional USD (6 dec). `motor`: 0=A,1=B,2=C,3=D. `layer` (solo D): 0/1/2/3(=3A)/4(=3B).
    /// `currency`: 0=$LUKA, 1=SOL/USDC. `is_whitelist`: fee reducido.
    pub fn process_fee(
        ctx: Context<ProcessFee>,
        amount: u64,
        motor: u8,
        layer: u8,
        currency: u8,
        is_whitelist: bool,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.paused, LukashError::ProtocolPaused);
        require!(amount > 0, LukashError::ZeroAmount);

        let fee_bps = compute_fee_bps(config.stage, motor, layer, currency, is_whitelist)?;
        let fee = mul_bps(amount, fee_bps)?;

        let state = &mut ctx.accounts.state;

        // Capa 0 (exención total) u otros fees nulos: no hay nada que distribuir.
        if fee == 0 {
            emit!(FeeProcessed {
                amount, fee: 0, to_vault: 0, to_lp_burn: 0, to_om: 0, to_staking: 0,
                motor, k_usd: state.vault_core_usd,
            });
            return Ok(());
        }

        // --- Distribución 35/35/15/15 (el resto va a staking para no perder dust) ---
        let to_vault = mul_bps(fee, DIST_VAULT_BPS)?;
        let to_lp_burn = mul_bps(fee, DIST_LP_BURN_BPS)?;
        let to_om = mul_bps(fee, DIST_OM_BPS)?;
        let to_staking = fee
            .checked_sub(to_vault).ok_or(LukashError::MathOverflow)?
            .checked_sub(to_lp_burn).ok_or(LukashError::MathOverflow)?
            .checked_sub(to_om).ok_or(LukashError::MathOverflow)?;

        // Invariante: la suma exacta debe ser el fee.
        let sum = to_vault
            .checked_add(to_lp_burn).ok_or(LukashError::MathOverflow)?
            .checked_add(to_om).ok_or(LukashError::MathOverflow)?
            .checked_add(to_staking).ok_or(LukashError::MathOverflow)?;
        require!(sum == fee, LukashError::DistributionInvariant);

        // --- Asset Layer (35%): split Core/Sociedad y asignación por activo del Core ---
        let core = mul_bps(to_vault, config.fee_split_core_bps)?;
        let sociedad = to_vault.checked_sub(core).ok_or(LukashError::MathOverflow)?;
        state.vault_core_usd = state.vault_core_usd.checked_add(core).ok_or(LukashError::MathOverflow)?;
        state.vault_sociedad_usd = state.vault_sociedad_usd.checked_add(sociedad).ok_or(LukashError::MathOverflow)?;

        let a_cbtc = mul_bps(core, config.vault_cbtc_bps)?;
        let a_sol = mul_bps(core, config.vault_sol_bps)?;
        let a_lst = mul_bps(core, config.vault_lst_bps)?;
        let a_usdc_res = mul_bps(core, config.vault_usdc_res_bps)?;
        let a_usdc_lend = core
            .checked_sub(a_cbtc).ok_or(LukashError::MathOverflow)?
            .checked_sub(a_sol).ok_or(LukashError::MathOverflow)?
            .checked_sub(a_lst).ok_or(LukashError::MathOverflow)?
            .checked_sub(a_usdc_res).ok_or(LukashError::MathOverflow)?;
        state.cbtc_usd = state.cbtc_usd.checked_add(a_cbtc).ok_or(LukashError::MathOverflow)?;
        state.sol_usd = state.sol_usd.checked_add(a_sol).ok_or(LukashError::MathOverflow)?;
        state.lst_usd = state.lst_usd.checked_add(a_lst).ok_or(LukashError::MathOverflow)?;
        state.usdc_res_usd = state.usdc_res_usd.checked_add(a_usdc_res).ok_or(LukashError::MathOverflow)?;
        state.usdc_lend_usd = state.usdc_lend_usd.checked_add(a_usdc_lend).ok_or(LukashError::MathOverflow)?;

        // --- LP / Quema (35%) ---
        // Motor A y C: el tramo va a quema/LP completo (sin Throttle).
        // Motor B y D: en B0 quema modulada por Throttle (resto a cola diferida); en B2 recircula.
        if motor == 0 || motor == 2 {
            state.burned_total = state.burned_total.checked_add(to_lp_burn).ok_or(LukashError::MathOverflow)?;
        } else if state.motor_b_state == MOTOR_B_B0 {
            let burn_bps = throttle_burn_bps(state.throttle_mode).min(BPS_DENOMINATOR);
            let burn_now = mul_bps(to_lp_burn, burn_bps)?;
            let deferred = to_lp_burn.checked_sub(burn_now).ok_or(LukashError::MathOverflow)?;
            state.burned_total = state.burned_total.checked_add(burn_now).ok_or(LukashError::MathOverflow)?;
            state.deferred_burn_queue = state.deferred_burn_queue.checked_add(deferred).ok_or(LukashError::MathOverflow)?;
        } else {
            state.recirculated_total = state.recirculated_total.checked_add(to_lp_burn).ok_or(LukashError::MathOverflow)?;
        }

        // --- O&M (15%) y Staking (15%) ---
        state.om_total = state.om_total.checked_add(to_om).ok_or(LukashError::MathOverflow)?;
        state.staking_total = state.staking_total.checked_add(to_staking).ok_or(LukashError::MathOverflow)?;

        // --- Hito Jaguar Lock: KASH Core >= $30M O 12 meses ---
        let now = Clock::get()?.unix_timestamp;
        if !state.jaguar_lock_hit {
            let elapsed = now.checked_sub(state.genesis_ts).unwrap_or(0);
            if state.vault_core_usd >= config.jaguar_lock_usd || elapsed >= JAGUAR_LOCK_SECONDS {
                state.jaguar_lock_hit = true;
            }
        }

        emit!(FeeProcessed {
            amount, fee, to_vault, to_lp_burn, to_om, to_staking, motor,
            k_usd: state.vault_core_usd,
        });
        Ok(())
    }

    /// El orquestador (keeper) actualiza precio y EMA30 desde el oráculo, y recalcula el modo del Throttle.
    /// En producción esto lo firma un rol keeper con validación Pyth+Switchboard; aquí lo firma la autoridad.
    pub fn update_oracle_state(ctx: Context<UpdateOracle>, luka_price: u64, ema30: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.luka_price = luka_price;
        state.ema30 = ema30;
        state.throttle_mode = throttle_mode_from_price(luka_price, ema30);
        emit!(OracleUpdated { luka_price, ema30, throttle_mode: state.throttle_mode });
        Ok(())
    }

    /// Conmuta el Motor B de B0 a B2 cuando K(t) >= K_min. Permissionless: cualquiera puede gatillarlo
    /// una vez cumplida la condición on-chain (es trustless — solo verifica el estado del Vault).
    pub fn switch_motor_b(ctx: Context<SwitchMotorB>) -> Result<()> {
        let config = &ctx.accounts.config;
        let state = &mut ctx.accounts.state;
        require!(state.motor_b_state == MOTOR_B_B0, LukashError::AlreadyB2);
        require!(state.vault_core_usd >= config.k_min_usd, LukashError::KminNotReached);
        state.motor_b_state = MOTOR_B_B2;
        let now = Clock::get()?.unix_timestamp;
        emit!(MotorBSwitched { k_usd: state.vault_core_usd, ts: now });
        Ok(())
    }

    /// Encola un cambio de parámetro crítico (Timelock 48h). kind: 1=stage, 2=k_min, 3=authority.
    pub fn queue_admin_change(ctx: Context<AdminOnly>, kind: u8, value: u64, new_pubkey: Pubkey) -> Result<()> {
        require!(kind >= 1 && kind <= 3, LukashError::InvalidStage);
        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;
        config.pending_kind = kind;
        config.pending_value = value;
        config.pending_pubkey = new_pubkey;
        config.pending_execute_after = now.checked_add(config.timelock_seconds).ok_or(LukashError::MathOverflow)?;
        Ok(())
    }

    /// Ejecuta el cambio encolado una vez transcurrido el Timelock.
    pub fn execute_admin_change(ctx: Context<AdminOnly>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;
        require!(config.pending_kind != 0, LukashError::NoPendingChange);
        require!(now >= config.pending_execute_after, LukashError::TimelockNotElapsed);
        match config.pending_kind {
            1 => {
                let s = u8::try_from(config.pending_value).map_err(|_| LukashError::InvalidStage)?;
                require!(s >= 1 && s <= 4, LukashError::InvalidStage);
                config.stage = s;
            }
            2 => config.k_min_usd = config.pending_value,
            3 => config.authority = config.pending_pubkey,
            _ => return err!(LukashError::NoPendingChange),
        }
        config.pending_kind = 0;
        config.pending_value = 0;
        config.pending_pubkey = Pubkey::default();
        config.pending_execute_after = 0;
        Ok(())
    }

    /// Circuit Breaker: pausa/reactiva el protocolo (solo autoridad).
    pub fn set_pause(ctx: Context<AdminOnly>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        Ok(())
    }

    /// Drena la cola de quema diferida (≤10%/semana) cuando el precio se ha normalizado.
    /// Permissionless: cualquiera puede gatillarlo si se cumplen las condiciones on-chain (es trustless).
    pub fn execute_deferred_burn(ctx: Context<ExecuteDeferredBurn>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // Solo se drena cuando el precio volvió a zona NORMAL o ACELERADO.
        require!(
            state.throttle_mode == THROTTLE_NORMAL || state.throttle_mode == THROTTLE_ACCELERATED,
            LukashError::ThrottleNotNormalized
        );
        require!(state.deferred_burn_queue > 0, LukashError::EmptyQueue);
        let now = Clock::get()?.unix_timestamp;
        require!(
            now.checked_sub(state.last_queue_exec_ts).unwrap_or(0) >= WEEK_SECONDS,
            LukashError::QueueCooldown
        );
        // Drena hasta el 10% de la cola actual (mínimo 1 para no quedar atascado con cola pequeña).
        let drain = mul_bps(state.deferred_burn_queue, QUEUE_DRAIN_BPS)?
            .max(1)
            .min(state.deferred_burn_queue);
        state.deferred_burn_queue = state
            .deferred_burn_queue
            .checked_sub(drain)
            .ok_or(LukashError::MathOverflow)?;
        state.burned_total = state
            .burned_total
            .checked_add(drain)
            .ok_or(LukashError::MathOverflow)?;
        state.last_queue_exec_ts = now;
        emit!(DeferredBurnExecuted {
            drained: drain,
            remaining: state.deferred_burn_queue,
            ts: now
        });
        Ok(())
    }
}

// ------------------------- Helpers (lógica pura, testeable) -------------------------

/// amount * bps / 10000 con checked math (u128 intermedio).
fn mul_bps(amount: u64, bps: u64) -> Result<u64> {
    let r = (amount as u128)
        .checked_mul(bps as u128).ok_or(LukashError::MathOverflow)?
        .checked_div(BPS_DENOMINATOR as u128).ok_or(LukashError::MathOverflow)?;
    u64::try_from(r).map_err(|_| LukashError::MathOverflow.into())
}

/// Fee en bps según etapa, motor, capa (para D), divisa y whitelist. Valida activación por etapa.
fn compute_fee_bps(stage: u8, motor: u8, layer: u8, currency: u8, is_wl: bool) -> Result<u64> {
    match motor {
        0 => Ok(match stage {
            1 => if is_wl { 250 } else { 400 },
            _ => if is_wl { 150 } else { 250 },
        }),
        1 => {
            require!(stage >= 2, LukashError::MotorNotActiveInStage); // Motor B desde Etapa 2A
            Ok(if is_wl { 150 } else { 250 })
        }
        2 => {
            require!(stage >= 3, LukashError::MotorNotActiveInStage); // Motor C desde Etapa 3
            Ok(50)
        }
        3 => {
            require!(stage >= 2, LukashError::MotorNotActiveInStage); // Motor D desde Etapa 2A
            match layer {
                0 => Ok(0),                                          // Capa 0: exención total
                1 => Ok(150),                                        // Capa 1: DeFi interno premium
                2 => Ok(if currency == 0 { 300 } else { 350 }),      // Capa 2: 3% $LUKA / 3.5% SOL-USDC
                3 => Ok(150),                                        // Capa 3A: DeFi externo en $LUKA
                4 => Ok(200),                                        // Capa 3B: DeFi externo SOL/USDC
                _ => err!(LukashError::InvalidMotorOrLayer),
            }
        }
        _ => err!(LukashError::InvalidMotorOrLayer),
    }
}

/// % de quema base del Motor B según el modo del Throttle (bps).
fn throttle_burn_bps(mode: u8) -> u64 {
    match mode {
        THROTTLE_ACCELERATED => BURN_ACCEL_BPS,
        THROTTLE_NORMAL => BURN_NORMAL_BPS,
        THROTTLE_CONSERVATIVE => BURN_CONS_BPS,
        THROTTLE_DEFENSIVE => BURN_DEF_BPS,
        _ => BURN_NORMAL_BPS,
    }
}

/// Modo del Throttle según precio vs EMA30. ACEL >1.2x · NORMAL 0.8-1.2x · CONS 0.5-0.8x · DEF <0.5x.
fn throttle_mode_from_price(price: u64, ema30: u64) -> u8 {
    if ema30 == 0 {
        return THROTTLE_NORMAL;
    }
    let ratio_bps = (price as u128)
        .saturating_mul(BPS_DENOMINATOR as u128)
        .checked_div(ema30 as u128)
        .unwrap_or(BPS_DENOMINATOR as u128);
    if ratio_bps > THROTTLE_ACCEL_BPS as u128 {
        THROTTLE_ACCELERATED
    } else if ratio_bps >= THROTTLE_CONS_BPS as u128 {
        THROTTLE_NORMAL
    } else if ratio_bps >= THROTTLE_DEF_BPS as u128 {
        THROTTLE_CONSERVATIVE
    } else {
        THROTTLE_DEFENSIVE
    }
}

// ------------------------- Contextos de cuentas -------------------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + ProtocolConfig::INIT_SPACE, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(init, payer = authority, space = 8 + ProtocolState::INIT_SPACE, seeds = [STATE_SEED], bump)]
    pub state: Account<'info, ProtocolState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessFee<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateOracle<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SwitchMotorB<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteDeferredBurn<'info> {
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub caller: Signer<'info>,
}

// ------------------------- Eventos -------------------------

#[event]
pub struct FeeProcessed {
    pub amount: u64,
    pub fee: u64,
    pub to_vault: u64,
    pub to_lp_burn: u64,
    pub to_om: u64,
    pub to_staking: u64,
    pub motor: u8,
    pub k_usd: u64,
}

#[event]
pub struct MotorBSwitched {
    pub k_usd: u64,
    pub ts: i64,
}

#[event]
pub struct OracleUpdated {
    pub luka_price: u64,
    pub ema30: u64,
    pub throttle_mode: u8,
}

#[event]
pub struct DeferredBurnExecuted {
    pub drained: u64,
    pub remaining: u64,
    pub ts: i64,
}
