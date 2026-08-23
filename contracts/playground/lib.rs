// LUKASH Protocol - Milestone 2 Sprint 2 (version de un solo archivo para Solana Playground)
// Pegar este archivo COMPLETO en src/lib.rs de un proyecto Anchor en beta.solpg.io y darle Build.
// v4: Sprint 2 (07-a switch B0→B2 por valoración de mercado + token accounting + doble candado 7d).
// v3: Sprint 1 (07-b cap quema 1%/día + 07-d drenaje siempre activo ADR-016 + hard-stop ENZ).
// v2: endurecido en seguridad (validaciones, freeze en pausa, protección de autoridad, eventos).

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
pub const DAY_SECONDS: i64 = 86_400;

// ---- Cap de quema diaria (07-b, Protocolo v4.3 §13) ----
pub const DAILY_BURN_CAP_BPS: u64 = 100; // 1.00% del supply/día

// ---- Drenaje de cola por modo Throttle (07-d, ADR-016: la quema nunca se detiene hasta ENZ) ----
pub const QUEUE_DRAIN_ACCEL_BPS: u64 = 2_500;  // 25%/sem — euforia
pub const QUEUE_DRAIN_NORMAL_BPS: u64 = 1_000;  // 10%/sem — estable
pub const QUEUE_DRAIN_CONS_BPS: u64 = 500;             //  5%/sem — bajista
pub const QUEUE_DRAIN_DEF_BPS: u64 = 200;       //  2%/sem — depresión

// ---- Hard-stop ENZ (supply mínimo absoluto, 07-d) ----
pub const SUPPLY_ENZ: u64 = 3_300_000_000_000_000; // 3.3B × 10^6 decimals
pub const INITIAL_SUPPLY: u64 = 10_000_000_000_000_000; // 10B × 10^6 decimals

// ---- Oráculo y valoración (07-a) ----
pub const VALUATION_MAX_STALENESS: i64 = 900;              // 15 min: snapshot debe ser fresco para switch
pub const K_MIN_PERSISTENCE_SECONDS: i64 = 7 * 24 * 3600;  // 7 días: anti-pump transitorio
pub const ORACLE_DEVIATION_BPS_MAX: u64 = 200;             // 2% (Capa 2: Pyth vs Switchboard)
pub const ORACLE_FEED_MAX_STALENESS: i64 = 60;             // 60s (Capa 2: frescura feed Pyth)
pub const JUPITER_MAX_SLIPPAGE_BPS: u64 = 50;              // 0.5% (Capa 2: CPI Jupiter)

// ---- Escalas de decimales por activo (para valoración) ----
pub const CBTC_SCALE: u128 = 100_000_000;     // 10^8 (satoshis)
pub const SOL_SCALE: u128 = 1_000_000_000;    // 10^9 (lamports)
pub const LST_SCALE: u128 = 1_000_000_000;    // 10^9 (lamports)

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
    #[msg("Burn complete: supply has reached ENZ minimum (3.3B)")]
    BurnComplete,
    #[msg("Aún no ha pasado una semana desde la última ejecución de la cola")]
    QueueCooldown,
    #[msg("No hay quemas diferidas en cola")]
    EmptyQueue,
    #[msg("Valor de oráculo inválido (precio o EMA30 en cero)")]
    InvalidOracleValue,
    #[msg("Tipo de cambio de parámetro inválido")]
    InvalidChangeKind,
    #[msg("Autoridad inválida (no puede ser la dirección por defecto)")]
    InvalidAuthorityPubkey,
    #[msg("Divisa inválida (debe ser 0=LUKA o 1=SOL/USDC)")]
    InvalidCurrency,
    #[msg("Valoración del Vault demasiado vieja (>15 min). Llamar refresh_vault_valuation primero")]
    ValuationStale,
    #[msg("K_min alcanzado pero no persistente 7 días — protección anti-pump transitorio")]
    KminNotPersistent,
    #[msg("Oráculos Pyth y Switchboard divergen >2% (Capa 2)")]
    OracleDeviationTooHigh,
    #[msg("Feed del oráculo demasiado viejo >60s (Capa 2)")]
    OracleFeedStale,
    #[msg("CPI a Jupiter falló por slippage (Capa 2)")]
    SwapSlippageExceeded,
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

    // Sprint 1 (07-b cap quema + 07-d drenaje/ENZ)
    pub burned_today_tokens: u64,
    pub burn_day_start_ts: i64,
    pub current_supply: u64,

    // Balances por bucket en unidades nativas (07-a: token accounting real)
    // Capa 1: alimentados por authority vía refresh_vault_valuation.
    // Capa 2: actualizados por CPI a Jupiter en process_fee.
    pub cbtc_amount: u64,      // satoshis (8 dec) de cBTC
    pub sol_amount: u64,       // lamports (9 dec) de SOL nativo
    pub lst_amount: u64,       // lamports de JitoSOL+mSOL agregados
    pub usdc_res_amount: u64,  // micro-USDC (6 dec) reserva inmediata
    pub usdc_lend_amount: u64, // micro-USDC depositado en Kamino/Marginfi

    // Snapshot de valoración de mercado del Vault (07-a)
    pub k_market_usd_snapshot: u64, // USD 6-dec, calculado en refresh_vault_valuation
    pub k_market_snapshot_ts: i64,  // timestamp del último snapshot

    // Persistencia del umbral K_min para switch B0→B2 (07-a)
    pub k_min_reached_since_ts: i64, // 0 si K_market < K_min actualmente

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
declare_id!("AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy");

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
        state.burned_today_tokens = 0;
        state.burn_day_start_ts = now;
        state.current_supply = INITIAL_SUPPLY;
        state.cbtc_amount = 0;
        state.sol_amount = 0;
        state.lst_amount = 0;
        state.usdc_res_amount = 0;
        state.usdc_lend_amount = 0;
        state.k_market_usd_snapshot = 0;
        state.k_market_snapshot_ts = 0;
        state.k_min_reached_since_ts = 0;
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
        require!(currency <= 1, LukashError::InvalidCurrency);

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
        let now = Clock::get()?.unix_timestamp;

        // Day rollover (07-b): reset diario al cruzar medianoche UTC
        let day_now = (now / DAY_SECONDS) * DAY_SECONDS;
        if state.burn_day_start_ts < day_now {
            state.burned_today_tokens = 0;
            state.burn_day_start_ts = day_now;
        }

        // ENZ hard-stop (07-d): supply ≤ 3.3B → tramo LP va al Vault, no se quema
        if state.current_supply > 0 && state.current_supply <= SUPPLY_ENZ {
            state.vault_core_usd = state.vault_core_usd
                .checked_add(to_lp_burn).ok_or(LukashError::MathOverflow)?;
        } else if motor == 0 {
            // Motor A: quema completa (sin Throttle). Motor C (motor=2) no llega aquí:
            // solo existe en Etapa 3 (post-ENZ), donde el guard anterior lo captura.
            let burn_base = to_lp_burn;
            let deferred_by_cap = apply_burn_cap(state, burn_base)?;
            state.burned_total = state.burned_total
                .checked_add(burn_base.checked_sub(deferred_by_cap).ok_or(LukashError::MathOverflow)?)
                .ok_or(LukashError::MathOverflow)?;
            if deferred_by_cap > 0 {
                state.deferred_burn_queue = state.deferred_burn_queue
                    .checked_add(deferred_by_cap).ok_or(LukashError::MathOverflow)?;
            }
        } else if state.motor_b_state == MOTOR_B_B2 {
            // Motor B en B2: recircula (no quema)
            state.recirculated_total = state.recirculated_total
                .checked_add(to_lp_burn).ok_or(LukashError::MathOverflow)?;
        } else {
            // Motor B (B0) y D: modulado por Throttle + cap diario
            let burn_bps = throttle_burn_bps(state.throttle_mode).min(BPS_DENOMINATOR);
            let burn_base = mul_bps(to_lp_burn, burn_bps)?;
            let deferred_by_throttle = to_lp_burn
                .checked_sub(burn_base).ok_or(LukashError::MathOverflow)?;
            let deferred_by_cap = apply_burn_cap(state, burn_base)?;
            state.burned_total = state.burned_total
                .checked_add(burn_base.checked_sub(deferred_by_cap).ok_or(LukashError::MathOverflow)?)
                .ok_or(LukashError::MathOverflow)?;
            let total_deferred = deferred_by_throttle
                .checked_add(deferred_by_cap).ok_or(LukashError::MathOverflow)?;
            if total_deferred > 0 {
                state.deferred_burn_queue = state.deferred_burn_queue
                    .checked_add(total_deferred).ok_or(LukashError::MathOverflow)?;
            }
        }

        // --- O&M (15%) y Staking (15%) ---
        state.om_total = state.om_total.checked_add(to_om).ok_or(LukashError::MathOverflow)?;
        state.staking_total = state.staking_total.checked_add(to_staking).ok_or(LukashError::MathOverflow)?;

        // --- Hito Jaguar Lock: KASH Core >= $30M O 12 meses ---
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
    pub fn update_oracle_state(ctx: Context<UpdateOracle>, luka_price: u64, ema30: u64, current_supply: u64) -> Result<()> {
        require!(luka_price > 0 && ema30 > 0, LukashError::InvalidOracleValue);
        let state = &mut ctx.accounts.state;
        state.luka_price = luka_price;
        state.ema30 = ema30;
        if current_supply > 0 {
            state.current_supply = current_supply;
        }
        state.throttle_mode = throttle_mode_from_price(luka_price, ema30);
        emit!(OracleUpdated { luka_price, ema30, throttle_mode: state.throttle_mode });
        Ok(())
    }

    /// Actualiza la valoración de mercado del Vault KASH Core (07-a).
    /// Capa 1 (devnet): authority alimenta precios y balances manualmente.
    /// Capa 2 (mainnet): se reemplaza por lectura directa de feeds Pyth/Switchboard
    /// + token account balances → instrucción permissionless.
    pub fn refresh_vault_valuation(
        ctx: Context<RefreshVaultValuation>,
        btc_price_usd: u64,
        sol_price_usd: u64,
        lst_price_usd: u64,
        luka_price_usd: u64,
        cbtc_amount: u64,
        sol_amount: u64,
        lst_amount: u64,
        usdc_res_amount: u64,
        usdc_lend_amount: u64,
    ) -> Result<()> {
        require!(btc_price_usd > 0 && sol_price_usd > 0, LukashError::InvalidOracleValue);
        require!(luka_price_usd > 0, LukashError::InvalidOracleValue);

        let config = &ctx.accounts.config;
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;

        state.cbtc_amount = cbtc_amount;
        state.sol_amount = sol_amount;
        state.lst_amount = lst_amount;
        state.usdc_res_amount = usdc_res_amount;
        state.usdc_lend_amount = usdc_lend_amount;

        let cbtc_usd = compute_asset_value(cbtc_amount, btc_price_usd, CBTC_SCALE)?;
        let sol_usd = compute_asset_value(sol_amount, sol_price_usd, SOL_SCALE)?;
        let lst_usd = compute_asset_value(lst_amount, lst_price_usd, LST_SCALE)?;
        let usdc_total = usdc_res_amount
            .checked_add(usdc_lend_amount).ok_or(LukashError::MathOverflow)?;

        let k_market = cbtc_usd
            .checked_add(sol_usd).ok_or(LukashError::MathOverflow)?
            .checked_add(lst_usd).ok_or(LukashError::MathOverflow)?
            .checked_add(usdc_total).ok_or(LukashError::MathOverflow)?;

        state.k_market_usd_snapshot = k_market;
        state.k_market_snapshot_ts = now;
        state.luka_price = luka_price_usd;

        if k_market >= config.k_min_usd {
            if state.k_min_reached_since_ts == 0 {
                state.k_min_reached_since_ts = now;
            }
        } else {
            state.k_min_reached_since_ts = 0;
        }

        emit!(VaultValuationRefreshed {
            k_market_usd: k_market,
            cbtc_usd_share: cbtc_usd,
            sol_usd_share: sol_usd,
            lst_usd_share: lst_usd,
            usdc_total,
            k_min_reached_since_ts: state.k_min_reached_since_ts,
            ts: now,
        });
        Ok(())
    }

    /// Conmuta el Motor B de B0 a B2 (ONE-WAY, irreversible). Permissionless.
    /// Doble candado (07-a):
    ///   A) Valoración fresca (< 15 min desde refresh_vault_valuation).
    ///   B) Persistencia: K_market >= K_min por 7 días continuos (anti-pump transitorio).
    pub fn switch_motor_b(ctx: Context<SwitchMotorB>) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.paused, LukashError::ProtocolPaused);

        let state = &mut ctx.accounts.state;
        require!(state.motor_b_state == MOTOR_B_B0, LukashError::AlreadyB2);

        let now = Clock::get()?.unix_timestamp;

        // Candado A: valoración fresca (< 15 min)
        require!(
            now.checked_sub(state.k_market_snapshot_ts).unwrap_or(i64::MAX)
                <= VALUATION_MAX_STALENESS,
            LukashError::ValuationStale
        );

        // K_market >= K_min
        require!(
            state.k_market_usd_snapshot >= config.k_min_usd,
            LukashError::KminNotReached
        );

        // Candado B: persistencia >= 7 días continuos
        require!(
            state.k_min_reached_since_ts > 0,
            LukashError::KminNotPersistent
        );
        require!(
            now.checked_sub(state.k_min_reached_since_ts).unwrap_or(0)
                >= K_MIN_PERSISTENCE_SECONDS,
            LukashError::KminNotPersistent
        );

        state.motor_b_state = MOTOR_B_B2;

        emit!(MotorBSwitched {
            k_market_usd: state.k_market_usd_snapshot,
            k_costo_usd: state.vault_core_usd,
            ts: now,
            persistencia_dias: (now - state.k_min_reached_since_ts) / 86_400,
        });
        Ok(())
    }

    /// Encola un cambio de parámetro crítico (Timelock 48h). kind: 1=stage, 2=k_min, 3=authority.
    pub fn queue_admin_change(ctx: Context<AdminOnly>, kind: u8, value: u64, new_pubkey: Pubkey) -> Result<()> {
        require!(kind >= 1 && kind <= 3, LukashError::InvalidChangeKind);
        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;
        config.pending_kind = kind;
        config.pending_value = value;
        config.pending_pubkey = new_pubkey;
        config.pending_execute_after = now.checked_add(config.timelock_seconds).ok_or(LukashError::MathOverflow)?;
        emit!(AdminChangeQueued { kind, value, execute_after: config.pending_execute_after });
        Ok(())
    }

    /// Ejecuta el cambio encolado una vez transcurrido el Timelock.
    pub fn execute_admin_change(ctx: Context<AdminOnly>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let config = &mut ctx.accounts.config;
        require!(config.pending_kind != 0, LukashError::NoPendingChange);
        require!(now >= config.pending_execute_after, LukashError::TimelockNotElapsed);
        let kind = config.pending_kind;
        match kind {
            1 => {
                let s = u8::try_from(config.pending_value).map_err(|_| LukashError::InvalidStage)?;
                require!(s >= 1 && s <= 4, LukashError::InvalidStage);
                config.stage = s;
            }
            2 => {
                require!(config.pending_value > 0, LukashError::InvalidChangeKind);
                config.k_min_usd = config.pending_value;
            }
            3 => {
                require!(config.pending_pubkey != Pubkey::default(), LukashError::InvalidAuthorityPubkey);
                config.authority = config.pending_pubkey;
            }
            _ => return err!(LukashError::NoPendingChange),
        }
        config.pending_kind = 0;
        config.pending_value = 0;
        config.pending_pubkey = Pubkey::default();
        config.pending_execute_after = 0;
        emit!(AdminChangeExecuted { kind });
        Ok(())
    }

    /// Circuit Breaker: pausa/reactiva el protocolo (solo autoridad).
    pub fn set_pause(ctx: Context<AdminOnly>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        emit!(PauseSet { paused });
        Ok(())
    }

    /// Drena la cola de quema diferida según el modo Throttle (ADR-016: todos los modos drenan).
    /// ACEL 25% · NORMAL 10% · CONS 5% · DEF 2% por semana. Hard-stop si supply ≤ ENZ.
    /// Permissionless: cualquiera puede gatillarlo si se cumplen las condiciones on-chain.
    pub fn execute_deferred_burn(ctx: Context<ExecuteDeferredBurn>) -> Result<()> {
        require!(!ctx.accounts.config.paused, LukashError::ProtocolPaused);
        let state = &mut ctx.accounts.state;

        // ENZ hard-stop: si supply ≤ 3.3B, la quema se apaga definitivamente
        if state.current_supply > 0 {
            require!(state.current_supply > SUPPLY_ENZ, LukashError::BurnComplete);
        }

        require!(state.deferred_burn_queue > 0, LukashError::EmptyQueue);
        let now = Clock::get()?.unix_timestamp;
        require!(
            now.checked_sub(state.last_queue_exec_ts).unwrap_or(0) >= WEEK_SECONDS,
            LukashError::QueueCooldown
        );

        // Day rollover (07-b)
        let day_now = (now / DAY_SECONDS) * DAY_SECONDS;
        if state.burn_day_start_ts < day_now {
            state.burned_today_tokens = 0;
            state.burn_day_start_ts = day_now;
        }

        // Drenaje según modo Throttle — TODOS los modos drenan (ADR-016)
        let drain_bps = match state.throttle_mode {
            THROTTLE_ACCELERATED  => QUEUE_DRAIN_ACCEL_BPS,
            THROTTLE_NORMAL       => QUEUE_DRAIN_NORMAL_BPS,
            THROTTLE_CONSERVATIVE => QUEUE_DRAIN_CONS_BPS,
            THROTTLE_DEFENSIVE    => QUEUE_DRAIN_DEF_BPS,
            _                     => QUEUE_DRAIN_DEF_BPS,
        };
        let mut drain = mul_bps(state.deferred_burn_queue, drain_bps)?
            .max(1)
            .min(state.deferred_burn_queue);

        // ENZ boundary: no bajar supply por debajo de 3.3B
        if state.current_supply > 0 && state.luka_price > 0 {
            let drain_tokens = usd_to_tokens(drain, state.luka_price)?;
            let max_burnable = state.current_supply.saturating_sub(SUPPLY_ENZ);
            if drain_tokens > max_burnable {
                drain = tokens_to_usd(max_burnable, state.luka_price)?;
            }
            if drain == 0 {
                return err!(LukashError::BurnComplete);
            }

            // Cap diario (07-b)
            let capped_tokens = usd_to_tokens(drain, state.luka_price)?;
            let cap_today = mul_bps(state.current_supply, DAILY_BURN_CAP_BPS)?;
            let remaining_cap = cap_today.saturating_sub(state.burned_today_tokens);
            if capped_tokens > remaining_cap && remaining_cap > 0 {
                drain = tokens_to_usd(remaining_cap, state.luka_price)?;
                state.burned_today_tokens = state.burned_today_tokens
                    .checked_add(remaining_cap).ok_or(LukashError::MathOverflow)?;
            } else if remaining_cap == 0 {
                // Cap lleno hoy, no drenar — cooldown para la próxima semana
                state.last_queue_exec_ts = now;
                emit!(DailyCapReached {
                    cap_tokens: cap_today,
                    burned_today: state.burned_today_tokens,
                    deferred_tokens: 0,
                });
                return Ok(());
            } else {
                state.burned_today_tokens = state.burned_today_tokens
                    .checked_add(capped_tokens).ok_or(LukashError::MathOverflow)?;
            }
        }

        state.deferred_burn_queue = state.deferred_burn_queue
            .checked_sub(drain).ok_or(LukashError::MathOverflow)?;
        state.burned_total = state.burned_total
            .checked_add(drain).ok_or(LukashError::MathOverflow)?;
        state.last_queue_exec_ts = now;

        emit!(DeferredBurnExecuted {
            drained: drain,
            remaining: state.deferred_burn_queue,
            ts: now,
            mode: state.throttle_mode,
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

/// Aplica el cap diario de quema (07-b). Retorna los USD que exceden el cap (deben ir a cola).
/// Si el oráculo no está activo (luka_price == 0), retorna 0 (quema todo, backward compat).
fn apply_burn_cap(state: &mut ProtocolState, burn_usd: u64) -> Result<u64> {
    if state.luka_price == 0 || state.current_supply == 0 {
        return Ok(0);
    }
    let tokens_to_burn = usd_to_tokens(burn_usd, state.luka_price)?;
    let cap_today = mul_bps(state.current_supply, DAILY_BURN_CAP_BPS)?;
    let remaining_cap = cap_today.saturating_sub(state.burned_today_tokens);
    let burn_ok_tokens = tokens_to_burn.min(remaining_cap);

    state.burned_today_tokens = state.burned_today_tokens
        .checked_add(burn_ok_tokens).ok_or(LukashError::MathOverflow)?;

    if burn_ok_tokens < tokens_to_burn {
        let deferred_tokens = tokens_to_burn
            .checked_sub(burn_ok_tokens).ok_or(LukashError::MathOverflow)?;
        let burn_ok_usd = tokens_to_usd(burn_ok_tokens, state.luka_price)?;
        let deferred_usd = burn_usd
            .checked_sub(burn_ok_usd).ok_or(LukashError::MathOverflow)?;
        emit!(DailyCapReached {
            cap_tokens: cap_today,
            burned_today: state.burned_today_tokens,
            deferred_tokens,
        });
        Ok(deferred_usd)
    } else {
        Ok(0)
    }
}

/// Convierte USD (6 dec) → tokens (6 dec) dado el precio de LUKA (USD 6-dec por 1 LUKA).
fn usd_to_tokens(usd_amount: u64, price: u64) -> Result<u64> {
    if price == 0 {
        return err!(LukashError::MathOverflow);
    }
    let r = (usd_amount as u128)
        .checked_mul(1_000_000u128).ok_or(LukashError::MathOverflow)?
        .checked_div(price as u128).ok_or(LukashError::MathOverflow)?;
    u64::try_from(r).map_err(|_| LukashError::MathOverflow.into())
}

/// Convierte tokens (6 dec) → USD (6 dec) dado el precio de LUKA (USD 6-dec por 1 LUKA).
fn tokens_to_usd(token_amount: u64, price: u64) -> Result<u64> {
    let r = (token_amount as u128)
        .checked_mul(price as u128).ok_or(LukashError::MathOverflow)?
        .checked_div(1_000_000u128).ok_or(LukashError::MathOverflow)?;
    u64::try_from(r).map_err(|_| LukashError::MathOverflow.into())
}

/// Calcula el valor USD (6-dec) de un activo dado su cantidad en unidades nativas y precio USD/unidad.
/// amount = unidades nativas (satoshis, lamports, micro-USDC).
/// price_usd = USD 6-dec por 1 unidad entera del activo.
/// scale = 10^decimals del activo (CBTC_SCALE, SOL_SCALE, etc.).
fn compute_asset_value(amount: u64, price_usd: u64, scale: u128) -> Result<u64> {
    if amount == 0 {
        return Ok(0);
    }
    let r = (amount as u128)
        .checked_mul(price_usd as u128).ok_or(LukashError::MathOverflow)?
        .checked_div(scale).ok_or(LukashError::MathOverflow)?;
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
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub caller: Signer<'info>,
}

/// Capa 1: authority alimenta precios y balances manualmente.
/// Capa 2: se reemplaza por lectura directa de Pyth/Switchboard + token accounts → permissionless.
#[derive(Accounts)]
pub struct RefreshVaultValuation<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub authority: Signer<'info>,
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
    pub k_market_usd: u64,
    pub k_costo_usd: u64,
    pub ts: i64,
    pub persistencia_dias: i64,
}

#[event]
pub struct VaultValuationRefreshed {
    pub k_market_usd: u64,
    pub cbtc_usd_share: u64,
    pub sol_usd_share: u64,
    pub lst_usd_share: u64,
    pub usdc_total: u64,
    pub k_min_reached_since_ts: i64,
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
    pub mode: u8,
}

#[event]
pub struct DailyCapReached {
    pub cap_tokens: u64,
    pub burned_today: u64,
    pub deferred_tokens: u64,
}

#[event]
pub struct PauseSet {
    pub paused: bool,
}

#[event]
pub struct AdminChangeQueued {
    pub kind: u8,
    pub value: u64,
    pub execute_after: i64,
}

#[event]
pub struct AdminChangeExecuted {
    pub kind: u8,
}
