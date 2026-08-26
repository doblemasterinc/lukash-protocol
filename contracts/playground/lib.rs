// LUKASH Protocol - Milestone 2 Sprint 5B (version de un solo archivo para Solana Playground)
// Pegar este archivo COMPLETO en src/lib.rs de un proyecto Anchor en beta.solpg.io y darle Build.
// Cargo.toml requiere: anchor-lang = "0.30.1" Y anchor-spl = "0.30.1"
// v10: Security hardening — has_one=authority en process_fee/refresh/swaps, Pyth owner validation,
//   DEVNET_MODE flag para fallbacks, confidence interval check, u128→u64 safe cast.
// v9.1: Fix devnet Pyth fallbacks (parse_pyth_price intacto para mainnet, fallback en callers).
// v9: Sprint 5B Fase B+C (Capa 2: oráculos Pyth + execute_vault_swaps a precio de oráculo).
// v8: Sprint 5B Fase A (Capa 2: quema real SPL burn CPI + initialize_burn_vault).
// v7: Sprint 5A (hardening: supply tracking on burn, close_protocol, MM close PDA, validaciones).
// v6: Sprint 4 (07-f Anti-Whale + Jaguar Exit Fee + MMRegistry + Transfer Hook Capa 1).
// v5: Sprint 3 (07-c Jaguar Shield: Tridente+CB+Seguro + 07-e módulo contra-cíclico LUKAI).
// v4: Sprint 2 (07-a switch B0→B2 por valoración de mercado + token accounting + doble candado 7d).
// v3: Sprint 1 (07-b cap quema 1%/día + 07-d drenaje siempre activo ADR-016 + hard-stop ENZ).
// v2: endurecido en seguridad (validaciones, freeze en pausa, protección de autoridad, eventos).

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Burn};

/// Pyth V2 Oracle Program ID (owner de las price feed accounts).
/// Devnet: gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s — MAINNET: FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH
pub mod pyth_oracle {
    use super::*;
    declare_id!("gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s");
}

// ================= constantes =================
// Constantes del protocolo LUKASH (valores de lanzamiento del Blueprint v4.3 §13).
// Todos los montos USD usan 6 decimales (1 USD = 1_000_000).

// ---- Seeds de PDAs ----
pub const CONFIG_SEED: &[u8] = b"config";
pub const STATE_SEED: &[u8] = b"state";
pub const MM_REGISTRY_SEED: &[u8] = b"mm_registry";
pub const BURN_VAULT_SEED: &[u8] = b"burn_vault";

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
pub const ORACLE_FEED_MAX_STALENESS: i64 = 86400;          // 86400s devnet (mainnet: 60s) (Capa 2: frescura feed Pyth)
pub const DEVNET_MODE: bool = true;                         // MAINNET: set to false — desactiva fallbacks de precio y relaja staleness
pub const JUPITER_MAX_SLIPPAGE_BPS: u64 = 50;              // 0.5% (Capa 2: CPI Jupiter)

// ---- Pyth V2 Price Account layout offsets (deserialización manual, Capa 2) ----
pub const PYTH_MAGIC: u32 = 0xa1b2c3d4;
pub const PYTH_MAGIC_OFFSET: usize = 0;
pub const PYTH_EXPO_OFFSET: usize = 20;       // exponent (i32)
pub const PYTH_TIMESTAMP_OFFSET: usize = 112;  // unix timestamp del último update
pub const PYTH_AGG_PRICE_OFFSET: usize = 224;  // aggregate price (i64)
pub const PYTH_AGG_CONF_OFFSET: usize = 232;   // aggregate confidence (u64)
pub const PYTH_AGG_STATUS_OFFSET: usize = 240; // aggregate status (u32, 1=Trading)
pub const PYTH_STATUS_TRADING: u32 = 1;
pub const PYTH_MIN_DATA_LEN: usize = 256;      // mínimo para leer todos los campos

// ---- Escalas de decimales por activo (para valoración) ----
pub const CBTC_SCALE: u128 = 100_000_000;     // 10^8 (satoshis)
pub const SOL_SCALE: u128 = 1_000_000_000;    // 10^9 (lamports)
pub const LST_SCALE: u128 = 1_000_000_000;    // 10^9 (lamports)

// ---- Circuit Breaker del Vault (07-c) ----
pub const CB_WINDOW_SECONDS: i64 = 3600;          // ventana de detección: 1h
pub const CB_DROP_THRESHOLD_BPS: u64 = 1_000;     // caída >10% dispara CB
pub const CB_PAUSE_SECONDS: i64 = 24 * 3600;      // pausa 24h

// ---- Seguro Anti-Exploit (07-c, ADR-015) ----
pub const INSURANCE_CAP_BPS: u64 = 500;           // 5% del Vault por evento
pub const INSURANCE_COOLDOWN_SECONDS: i64 = 365 * 86_400; // 12 meses entre eventos

// ---- Régimen de mercado — módulo contra-cíclico (07-e) ----
pub const REGIME_BULL: u8 = 0;
pub const REGIME_NEUTRAL: u8 = 1;
pub const REGIME_BEAR: u8 = 2;

pub const EMA_BULL_THRESHOLD_BPS: u64 = 10_200;   // EMA30 > 1.02×EMA90 → BULL
pub const EMA_BEAR_THRESHOLD_BPS: u64 = 9_800;    // EMA30 < 0.98×EMA90 → BEAR
pub const VOL_HIGH_THRESHOLD_BPS: u64 = 6_000;    // 60% anualizado → vol "alta"
pub const REGIME_MAX_STALENESS: i64 = 48 * 3600;  // 48h sin update → fail-safe NEUTRAL

// Splits por régimen: (volátiles_bps, usdc_bps) del Asset Layer
pub const SPLIT_BULL: (u64, u64) = (4_000, 6_000);
pub const SPLIT_NEUTRAL: (u64, u64) = (7_500, 2_500);
pub const SPLIT_BEAR: (u64, u64) = (7_000, 3_000);

// Proporciones relativas dentro de "volátiles" (suman 10000): cBTC:SOL:LST = 35:15:20 normalizado
pub const VOLATIL_CBTC_REL_BPS: u64 = 5_000;
pub const VOLATIL_SOL_REL_BPS: u64 = 2_143;
pub const VOLATIL_LST_REL_BPS: u64 = 2_857;      // 10000 - 5000 - 2143

// Proporciones relativas dentro de "USDC" (suman 10000): reserva:lending = 25:5 normalizado
pub const USDC_RES_REL_BPS: u64 = 8_333;
pub const USDC_LEND_REL_BPS: u64 = 1_667;

// ---- Anti-Whale (07-f, ADR-012 C10): umbral por % del pool ----
pub const AW_THR_1_BPS: u64 = 100;     // 1% del pool
pub const AW_THR_2_BPS: u64 = 200;     // 2%
pub const AW_THR_3_BPS: u64 = 500;     // 5%
pub const AW_FEE_1_BPS: u64 = 300;     // 3% sobre excedente
pub const AW_FEE_2_BPS: u64 = 600;     // 6%
pub const AW_FEE_3_BPS: u64 = 1_000;   // 10%

// ---- Jaguar Exit Fee (07-f, v4.3 §9): activación dual ----
pub const EXIT_FEE_ET1_BPS: u64 = 500;   // 5% Génesis
pub const EXIT_FEE_ET2_BPS: u64 = 300;   // 3% Etapa 2
pub const EXIT_FEE_ET3_BPS: u64 = 100;   // 1% Etapa 3+
pub const EXIT_FEE_PRICE_TRIG_BPS: u64 = 7_000;  // <0.7×EMA30
pub const EXIT_FEE_VOL_TRIG_BPS: u64 = 30;        // >0.3% supply/hora (bps)
pub const SELL_PRESSURE_WINDOW: i64 = 3600;        // ventana rodante de 1h

// ---- Aura — exención nivel Jaguar ----
pub const AURA_JAGUAR_MIN: u64 = 10_000;

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

    // 07-c: Tridente Multisig
    #[msg("Tridente Multisig no activado")]
    TridenteNotActivated,
    #[msg("Tridente ya activado (one-way, irreversible)")]
    TridenteAlreadyActivated,
    #[msg("Firmante del Tridente inválido (pubkey cero)")]
    InvalidTridenteSigner,
    #[msg("Los 3 firmantes del Tridente deben ser distintos entre sí")]
    TridenteSignersMustBeDistinct,
    #[msg("Un firmante del Tridente no puede ser la authority")]
    TridenteSignerCannotBeAuthority,
    #[msg("Faltan firmas del Tridente (requeridas 3-de-3)")]
    TridenteSignaturesIncomplete,
    #[msg("Etapa 2 requiere el Tridente activado")]
    TridenteRequiredForStage2,

    // 07-c: Circuit Breaker del Vault
    #[msg("Circuit Breaker activo — protocolo en pausa por 24h")]
    CircuitBreakerActive,
    #[msg("Circuit Breaker no está activo — nada que cancelar")]
    CircuitBreakerNotActive,

    // 07-c: Seguro Anti-Exploit
    #[msg("Seguro Anti-Exploit activo solo desde Etapa 2B (Motor B2)")]
    InsuranceNotYetActive,
    #[msg("Cooldown del Seguro: máximo 1 evento cada 12 meses")]
    InsuranceCooldown,
    #[msg("Monto excede el cap del 5% del Vault")]
    InsuranceExceedsCap,

    // 07-e: Módulo contra-cíclico
    #[msg("Régimen inválido (debe ser 0=BULL, 1=NEUTRAL o 2=BEAR)")]
    InvalidRegime,
    #[msg("EMA90 no puede ser cero")]
    InvalidEmaInput,

    // 07-f: Anti-Whale + Exit Fee + MM Registry
    #[msg("MM registry ya existe o no existe para esa pubkey")]
    MMRegistryInvalid,
    #[msg("Pool de liquidez con valor cero o no disponible")]
    PoolLiquidityMissing,
    #[msg("Sender inválido (Pubkey::default())")]
    InvalidSender,

    // Sprint 5B Capa 2: quema real
    #[msg("Burn vault sin tokens suficientes para la quema solicitada (quema parcial aplicada)")]
    BurnVaultInsufficient,

    // Sprint 5B Capa 2: swaps
    #[msg("No hay swaps pendientes (todos los pending_swap_*_usd son cero)")]
    NoPendingSwaps,
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

    // Tridente Multisig 3-de-3 (07-c, ADR-015)
    pub tridente_activated: bool,
    pub tridente_signer_1: Pubkey,
    pub tridente_signer_2: Pubkey,
    pub tridente_signer_3: Pubkey,
    pub tridente_activated_ts: i64,

    // LP Fundador ATA — exento de Anti-Whale + Exit Fee (07-f, ADR-015)
    pub lp_fundador_ata: Pubkey,

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

    // Circuit Breaker del Vault (07-c)
    pub cb_active_until_ts: i64,     // 0 si no pausado; timestamp fin de pausa si activo
    pub cb_last_snapshot_usd: u64,   // K_market snapshot hace ~1h para detectar caída
    pub cb_last_snapshot_ts: i64,

    // Seguro Anti-Exploit (07-c, ADR-015)
    pub last_insurance_recovery_ts: i64,
    pub insurance_recoveries_total_usd: u64,

    // Módulo contra-cíclico LUKAI (07-e)
    pub market_regime: u8,           // 0=BULL, 1=NEUTRAL, 2=BEAR
    pub regime_updated_ts: i64,

    // Jaguar Exit Fee: presión de venta en ventana rodante de 1h (07-f)
    pub sell_pressure_1h_supply_bps: u64,
    pub sell_pressure_last_reset_ts: i64,

    // Pending vault swaps: USD 6-dec pendientes de convertir a activos nativos (Capa 2 Fase C)
    pub pending_swap_cbtc_usd: u64,
    pub pending_swap_sol_usd: u64,
    pub pending_swap_lst_usd: u64,
    pub pending_swap_usdc_res_usd: u64,
    pub pending_swap_usdc_lend_usd: u64,

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

/// Market Maker registrado — PDA ["mm_registry", mm_pubkey]. Exento de Anti-Whale + Exit Fee.
#[account]
#[derive(InitSpace)]
pub struct MMRegistry {
    pub mm: Pubkey,
    pub is_active: bool,
    pub registered_at: i64,
    pub bump: u8,
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
        config.tridente_activated = false;
        config.tridente_signer_1 = Pubkey::default();
        config.tridente_signer_2 = Pubkey::default();
        config.tridente_signer_3 = Pubkey::default();
        config.tridente_activated_ts = 0;
        config.lp_fundador_ata = Pubkey::default();
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
        state.cb_active_until_ts = 0;
        state.cb_last_snapshot_usd = 0;
        state.cb_last_snapshot_ts = 0;
        state.last_insurance_recovery_ts = 0;
        state.insurance_recoveries_total_usd = 0;
        state.market_regime = REGIME_NEUTRAL;
        state.regime_updated_ts = 0;
        state.sell_pressure_1h_supply_bps = 0;
        state.sell_pressure_last_reset_ts = now;
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

        let now_ts = Clock::get()?.unix_timestamp;
        // Circuit Breaker guard (07-c)
        require!(now_ts >= ctx.accounts.state.cb_active_until_ts, LukashError::CircuitBreakerActive);

        let fee_bps = compute_fee_bps(config.stage, motor, layer, currency, is_whitelist)?;
        let fee = mul_bps(amount, fee_bps)?;

        let state = &mut ctx.accounts.state;
        let mut tokens_to_burn_real: u64 = 0;

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

        // Módulo contra-cíclico (07-e): splits dinámicos por régimen de mercado
        let regime = resolve_regime_effective(state, now_ts);
        let (vol_bps, _usdc_bps) = match regime {
            REGIME_BULL => SPLIT_BULL,
            REGIME_BEAR => SPLIT_BEAR,
            _           => SPLIT_NEUTRAL,
        };
        let vol_total = mul_bps(core, vol_bps)?;
        let usdc_total_alloc = core.checked_sub(vol_total).ok_or(LukashError::MathOverflow)?;

        // Dentro de volátiles: cBTC/SOL/LST proporcionales (50/21.4/28.6)
        let a_cbtc = mul_bps(vol_total, VOLATIL_CBTC_REL_BPS)?;
        let a_sol = mul_bps(vol_total, VOLATIL_SOL_REL_BPS)?;
        let a_lst = vol_total
            .checked_sub(a_cbtc).ok_or(LukashError::MathOverflow)?
            .checked_sub(a_sol).ok_or(LukashError::MathOverflow)?;
        // Dentro de USDC: reserva/lending proporcionales (83.3/16.7)
        let a_usdc_res = mul_bps(usdc_total_alloc, USDC_RES_REL_BPS)?;
        let a_usdc_lend = usdc_total_alloc
            .checked_sub(a_usdc_res).ok_or(LukashError::MathOverflow)?;

        state.cbtc_usd = state.cbtc_usd.checked_add(a_cbtc).ok_or(LukashError::MathOverflow)?;
        state.sol_usd = state.sol_usd.checked_add(a_sol).ok_or(LukashError::MathOverflow)?;
        state.lst_usd = state.lst_usd.checked_add(a_lst).ok_or(LukashError::MathOverflow)?;
        state.usdc_res_usd = state.usdc_res_usd.checked_add(a_usdc_res).ok_or(LukashError::MathOverflow)?;
        state.usdc_lend_usd = state.usdc_lend_usd.checked_add(a_usdc_lend).ok_or(LukashError::MathOverflow)?;

        // Acumular montos pendientes de swap (Capa 2 Fase C: execute_vault_swaps los convierte)
        state.pending_swap_cbtc_usd = state.pending_swap_cbtc_usd.checked_add(a_cbtc).ok_or(LukashError::MathOverflow)?;
        state.pending_swap_sol_usd = state.pending_swap_sol_usd.checked_add(a_sol).ok_or(LukashError::MathOverflow)?;
        state.pending_swap_lst_usd = state.pending_swap_lst_usd.checked_add(a_lst).ok_or(LukashError::MathOverflow)?;
        state.pending_swap_usdc_res_usd = state.pending_swap_usdc_res_usd.checked_add(a_usdc_res).ok_or(LukashError::MathOverflow)?;
        state.pending_swap_usdc_lend_usd = state.pending_swap_usdc_lend_usd.checked_add(a_usdc_lend).ok_or(LukashError::MathOverflow)?;

        // --- LP / Quema (35%) ---
        // Day rollover (07-b): reset diario al cruzar medianoche UTC
        let day_now = (now_ts / DAY_SECONDS) * DAY_SECONDS;
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
            let effective_burn = burn_base.checked_sub(deferred_by_cap).ok_or(LukashError::MathOverflow)?;
            state.burned_total = state.burned_total
                .checked_add(effective_burn).ok_or(LukashError::MathOverflow)?;
            if effective_burn > 0 && state.luka_price > 0 {
                let tokens = usd_to_tokens(effective_burn, state.luka_price)?;
                state.current_supply = state.current_supply.saturating_sub(tokens);
                tokens_to_burn_real = tokens;
            }
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
            let effective_burn = burn_base.checked_sub(deferred_by_cap).ok_or(LukashError::MathOverflow)?;
            state.burned_total = state.burned_total
                .checked_add(effective_burn).ok_or(LukashError::MathOverflow)?;
            if effective_burn > 0 && state.luka_price > 0 {
                let tokens = usd_to_tokens(effective_burn, state.luka_price)?;
                state.current_supply = state.current_supply.saturating_sub(tokens);
                tokens_to_burn_real = tokens;
            }
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
            let elapsed = now_ts.checked_sub(state.genesis_ts).unwrap_or(0);
            if state.vault_core_usd >= config.jaguar_lock_usd || elapsed >= JAGUAR_LOCK_SECONDS {
                state.jaguar_lock_hit = true;
            }
        }

        emit!(FeeProcessed {
            amount, fee, to_vault, to_lp_burn, to_om, to_staking, motor,
            k_usd: state.vault_core_usd,
        });

        // Capa 2: quema real de tokens vía CPI al Token Program
        if tokens_to_burn_real > 0 {
            let available = ctx.accounts.burn_vault.amount;
            let actual = tokens_to_burn_real.min(available);
            if actual > 0 {
                let bump = ctx.accounts.state.bump;
                let seeds: &[&[u8]] = &[STATE_SEED, &[bump]];
                let signer_seeds = &[seeds];
                token::burn(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(),
                        Burn {
                            mint: ctx.accounts.luka_mint.to_account_info(),
                            from: ctx.accounts.burn_vault.to_account_info(),
                            authority: ctx.accounts.state.to_account_info(),
                        },
                        signer_seeds,
                    ),
                    actual,
                )?;
                emit!(RealBurnExecuted { tokens_requested: tokens_to_burn_real, tokens_burned: actual });
            }
        }

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

    /// Actualiza la valoración de mercado del Vault KASH Core (07-a, Capa 2).
    /// BTC/USD y SOL/USD se leen directamente de feeds Pyth (permissionless).
    /// LST y LUKA no tienen feed Pyth propio → se pasan como parámetros.
    /// Balances de vault se pasan como parámetros (Capa 2 parcial: los vault ATAs se crean en Fase D).
    pub fn refresh_vault_valuation(
        ctx: Context<RefreshVaultValuation>,
        lst_price_usd: u64,
        luka_price_usd: u64,
        cbtc_amount: u64,
        sol_amount: u64,
        lst_amount: u64,
        usdc_res_amount: u64,
        usdc_lend_amount: u64,
    ) -> Result<()> {
        require!(luka_price_usd > 0, LukashError::InvalidOracleValue);

        let now = Clock::get()?.unix_timestamp;

        let btc_price_usd: u64 = {
            let data = ctx.accounts.pyth_btc_feed.try_borrow_data()?;
            let p = match parse_pyth_price(&data, now) {
                Ok((raw, expo)) => pyth_price_to_usd6(raw, expo).unwrap_or(0),
                Err(_) => 0,
            };
            if p > 0 { p } else if DEVNET_MODE { 65_000_000_000 } else { return Err(LukashError::InvalidOracleValue.into()) }
        };
        let sol_price_usd: u64 = {
            let data = ctx.accounts.pyth_sol_feed.try_borrow_data()?;
            let p = match parse_pyth_price(&data, now) {
                Ok((raw, expo)) => pyth_price_to_usd6(raw, expo).unwrap_or(0),
                Err(_) => 0,
            };
            if p > 0 { p } else if DEVNET_MODE { 150_000_000 } else { return Err(LukashError::InvalidOracleValue.into()) }
        };

        let config = &ctx.accounts.config;
        let state = &mut ctx.accounts.state;

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

        // Circuit Breaker: detectar caída >10% en ventana de 1h (07-c)
        if state.cb_last_snapshot_ts == 0 || now - state.cb_last_snapshot_ts >= CB_WINDOW_SECONDS {
            if state.cb_last_snapshot_usd > 0 {
                let threshold = state.cb_last_snapshot_usd
                    .saturating_sub(mul_bps(state.cb_last_snapshot_usd, CB_DROP_THRESHOLD_BPS)?);
                if k_market < threshold {
                    state.cb_active_until_ts = now.checked_add(CB_PAUSE_SECONDS)
                        .ok_or(LukashError::MathOverflow)?;
                    emit!(CircuitBreakerTriggered {
                        prev_usd: state.cb_last_snapshot_usd,
                        current_usd: k_market,
                        paused_until: state.cb_active_until_ts,
                    });
                }
            }
            state.cb_last_snapshot_usd = k_market;
            state.cb_last_snapshot_ts = now;
        }

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
            btc_price_usd,
            sol_price_usd,
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
        // Circuit Breaker guard (07-c)
        require!(now >= state.cb_active_until_ts, LukashError::CircuitBreakerActive);

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
        require!(kind >= 1 && kind <= 4, LukashError::InvalidChangeKind);
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
                // Candado estructural (07-c): Etapa 2 requiere Tridente activado
                if s >= 2 {
                    require!(config.tridente_activated, LukashError::TridenteRequiredForStage2);
                }
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
            4 => {
                require!(config.pending_pubkey != Pubkey::default(), LukashError::InvalidAuthorityPubkey);
                config.lp_fundador_ata = config.pending_pubkey;
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

    /// Activa el Tridente Multisig 3-de-3 (ONE-WAY, irreversible). Solo authority.
    /// Las 3 pubkeys deben ser distintas, no-default, y ninguna == authority.
    /// Candado estructural: sin Tridente activado, el contrato bloquea el paso a Etapa 2.
    pub fn activate_tridente(ctx: Context<AdminOnly>, pk1: Pubkey, pk2: Pubkey, pk3: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(!config.tridente_activated, LukashError::TridenteAlreadyActivated);
        require!(pk1 != Pubkey::default(), LukashError::InvalidTridenteSigner);
        require!(pk2 != Pubkey::default(), LukashError::InvalidTridenteSigner);
        require!(pk3 != Pubkey::default(), LukashError::InvalidTridenteSigner);
        require!(pk1 != pk2 && pk2 != pk3 && pk1 != pk3, LukashError::TridenteSignersMustBeDistinct);
        require!(
            pk1 != config.authority && pk2 != config.authority && pk3 != config.authority,
            LukashError::TridenteSignerCannotBeAuthority
        );
        config.tridente_signer_1 = pk1;
        config.tridente_signer_2 = pk2;
        config.tridente_signer_3 = pk3;
        config.tridente_activated = true;
        config.tridente_activated_ts = Clock::get()?.unix_timestamp;
        emit!(TridenteActivated { pk1, pk2, pk3, ts: config.tridente_activated_ts });
        Ok(())
    }

    /// Cancela el Circuit Breaker del Vault antes de las 24h. Requiere Tridente 3-de-3.
    pub fn cancel_circuit_breaker(ctx: Context<TridenteAction>) -> Result<()> {
        let config = &ctx.accounts.config;
        assert_tridente_signed(config, ctx.remaining_accounts)?;
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;
        require!(state.cb_active_until_ts > now, LukashError::CircuitBreakerNotActive);
        let was_until = state.cb_active_until_ts;
        state.cb_active_until_ts = 0;
        emit!(CircuitBreakerCancelled {
            cancelled_at: Clock::get()?.unix_timestamp,
            was_until,
        });
        Ok(())
    }

    /// Recibe reembolso del seguro anti-exploit al Vault (solo entrada, nunca salida).
    /// Requiere Tridente 3-de-3. Activo desde Etapa 2B. Max 5% del Vault. Cooldown 12 meses.
    pub fn receive_insurance_recovery(ctx: Context<TridenteAction>, amount_usdc: u64) -> Result<()> {
        require!(amount_usdc > 0, LukashError::ZeroAmount);
        let config = &ctx.accounts.config;
        assert_tridente_signed(config, ctx.remaining_accounts)?;
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;

        require!(state.motor_b_state == MOTOR_B_B2, LukashError::InsuranceNotYetActive);

        let elapsed = now.saturating_sub(state.last_insurance_recovery_ts);
        require!(
            state.last_insurance_recovery_ts == 0 || elapsed >= INSURANCE_COOLDOWN_SECONDS,
            LukashError::InsuranceCooldown
        );

        let max_recovery = mul_bps(state.k_market_usd_snapshot, INSURANCE_CAP_BPS)?;
        require!(amount_usdc <= max_recovery, LukashError::InsuranceExceedsCap);

        state.usdc_res_amount = state.usdc_res_amount
            .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;
        state.usdc_res_usd = state.usdc_res_usd
            .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;
        state.vault_core_usd = state.vault_core_usd
            .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;
        state.last_insurance_recovery_ts = now;
        state.insurance_recoveries_total_usd = state.insurance_recoveries_total_usd
            .checked_add(amount_usdc).ok_or(LukashError::MathOverflow)?;

        emit!(InsuranceRecoveryReceived { amount_usdc, cap_at_event: max_recovery, ts: now });
        Ok(())
    }

    /// Actualiza el régimen de mercado (módulo contra-cíclico LUKAI, 07-e).
    /// Firmante = authority (keeper LUKAI). Típicamente 1×/día.
    /// Lógica: mayoría de 3 señales (EMA30/EMA90, vol BTC, vol Motor A). Disenso → NEUTRAL.
    pub fn update_market_regime(
        ctx: Context<UpdateOracle>,
        ema30_btc_price: u64,
        ema90_btc_price: u64,
        realized_vol_30d_bps: u64,
        vol_a_7d_usd: u64,
        vol_a_30d_usd: u64,
    ) -> Result<()> {
        require!(ema90_btc_price > 0, LukashError::InvalidEmaInput);
        require!(ema30_btc_price > 0, LukashError::InvalidEmaInput);

        let ratio_bps = (ema30_btc_price as u128)
            .checked_mul(BPS_DENOMINATOR as u128).ok_or(LukashError::MathOverflow)?
            .checked_div(ema90_btc_price as u128).ok_or(LukashError::MathOverflow)?;
        let ratio = u64::try_from(ratio_bps).map_err(|_| LukashError::MathOverflow)?;

        let signal_primary: u8 = if ratio > EMA_BULL_THRESHOLD_BPS { REGIME_BULL }
            else if ratio < EMA_BEAR_THRESHOLD_BPS { REGIME_BEAR }
            else { REGIME_NEUTRAL };

        let signal_vol: u8 = if realized_vol_30d_bps > VOL_HIGH_THRESHOLD_BPS {
            signal_primary
        } else {
            REGIME_NEUTRAL
        };

        let signal_vol_a: u8 = if vol_a_30d_usd == 0 { REGIME_NEUTRAL } else {
            let norm_30d = (vol_a_30d_usd as u128)
                .checked_div(30).unwrap_or(1).max(1)
                .checked_mul(7).unwrap_or(u128::MAX);
            let ratio_a = (vol_a_7d_usd as u128)
                .checked_mul(10_000).unwrap_or(0)
                .checked_div(norm_30d.max(1)).unwrap_or(10_000);
            if ratio_a > 12_000 { REGIME_BULL }
            else if ratio_a < 8_000 { REGIME_BEAR }
            else { REGIME_NEUTRAL }
        };

        let regime = majority_vote(signal_primary, signal_vol, signal_vol_a);

        let state = &mut ctx.accounts.state;
        state.market_regime = regime;
        state.regime_updated_ts = Clock::get()?.unix_timestamp;

        emit!(MarketRegimeUpdated {
            regime,
            ema30_btc: ema30_btc_price,
            ema90_btc: ema90_btc_price,
            vol_30d_bps: realized_vol_30d_bps,
            vol_a_7d: vol_a_7d_usd,
            vol_a_30d: vol_a_30d_usd,
            signals: [signal_primary, signal_vol, signal_vol_a],
            ts: state.regime_updated_ts,
        });
        Ok(())
    }

    /// Transfer Hook — Jaguar Shield: Anti-Whale + Jaguar Exit Fee (07-f, ADR-012).
    /// Capa 1 (devnet): authority alimenta parámetros del contexto manualmente.
    /// Capa 2 (mainnet Token-2022): invocado automáticamente por el Token Program en cada transfer;
    /// los parámetros se leen de cuentas on-chain (pool Meteora, Aura PDA, staking PDA, MMRegistry PDA).
    pub fn transfer_hook(
        ctx: Context<TransferHookCtx>,
        amount: u64,
        sender: Pubkey,
        is_sale_to_pool: bool,
        pool_liquidity_usd: u64,
        sender_aura_score: u64,
        sender_has_staking: bool,
        sender_has_lp_lock: bool,
        sender_is_mm: bool,
        is_internal_cpi: bool,
    ) -> Result<()> {
        require!(amount > 0, LukashError::ZeroAmount);
        require!(sender != Pubkey::default(), LukashError::InvalidSender);

        let config = &ctx.accounts.config;
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;

        require!(now >= state.cb_active_until_ts, LukashError::CircuitBreakerActive);

        // Compras (from pool to user): sin fee (ADR-012 C10)
        if !is_sale_to_pool {
            emit!(TransferInspected { sender, amount, fee_aw: 0, fee_exit: 0, exempt: false, is_buy: true });
            return Ok(());
        }

        // Exenciones Anti-Whale: todas las 6 condiciones (§2.5 fail-fast)
        let exempt_aw = is_internal_cpi
            || sender_is_mm
            || (config.lp_fundador_ata != Pubkey::default() && sender == config.lp_fundador_ata)
            || sender_has_lp_lock
            || sender_has_staking
            || sender_aura_score >= AURA_JAGUAR_MIN;

        // Exenciones Exit Fee: todas EXCEPTO staking (el Exit Fee no se exime por staking)
        let exempt_exit = is_internal_cpi
            || sender_is_mm
            || (config.lp_fundador_ata != Pubkey::default() && sender == config.lp_fundador_ata)
            || sender_has_lp_lock
            || sender_aura_score >= AURA_JAGUAR_MIN;

        // Actualizar presión de venta (ventana rodante 1h, I22)
        update_sell_pressure(state, amount, now)?;

        if exempt_aw && exempt_exit {
            emit!(TransferInspected { sender, amount, fee_aw: 0, fee_exit: 0, exempt: true, is_buy: false });
            return Ok(());
        }

        // --- Anti-Whale (solo si no exento) ---
        let fee_aw = if exempt_aw {
            0
        } else {
            require!(pool_liquidity_usd > 0, LukashError::PoolLiquidityMissing);
            compute_anti_whale_fee(amount, state.luka_price, pool_liquidity_usd)?
        };

        // --- Exit Fee (solo si no exento) ---
        let fee_exit = if exempt_exit {
            0
        } else {
            compute_exit_fee(config.stage, amount, state)?
        };

        let total_fee = fee_aw.checked_add(fee_exit).ok_or(LukashError::MathOverflow)?;

        // 100% de fees del Shield → Vault Core (ADR-012, I20)
        if total_fee > 0 && state.luka_price > 0 {
            let fee_usdc = tokens_to_usd(total_fee, state.luka_price)?;
            state.usdc_res_amount = state.usdc_res_amount
                .checked_add(fee_usdc).ok_or(LukashError::MathOverflow)?;
            state.usdc_res_usd = state.usdc_res_usd
                .checked_add(fee_usdc).ok_or(LukashError::MathOverflow)?;
            state.vault_core_usd = state.vault_core_usd
                .checked_add(fee_usdc).ok_or(LukashError::MathOverflow)?;

            if fee_aw > 0 {
                let tx_usd = tokens_to_usd(amount, state.luka_price)?;
                let tx_pct_pool_bps = (tx_usd as u128)
                    .checked_mul(BPS_DENOMINATOR as u128).unwrap_or(0)
                    .checked_div(pool_liquidity_usd.max(1) as u128).unwrap_or(0) as u64;
                let excedente_bps = tx_pct_pool_bps.saturating_sub(AW_THR_1_BPS);
                let excedente_luka = mul_bps(amount, excedente_bps)?;
                emit!(AntiWhaleTriggered {
                    sender, amount_luka: amount, tx_pct_pool_bps,
                    excedente_luka, fee_luka: fee_aw,
                });
            }
            if fee_exit > 0 {
                emit!(ExitFeeTriggered {
                    sender, amount_luka: amount, luka_price: state.luka_price,
                    ema30: state.ema30, fee_luka: fee_exit, stage: config.stage,
                });
            }

            let source = if fee_aw > 0 && fee_exit > 0 { 2u8 }
                else if fee_exit > 0 { 1u8 } else { 0u8 };
            emit!(ShieldFeeCollected { sender, fee_luka: total_fee, fee_usdc_to_vault: fee_usdc, source });
        }

        emit!(TransferInspected {
            sender, amount, fee_aw, fee_exit, exempt: false, is_buy: false,
        });
        Ok(())
    }

    /// Registra un Market Maker como exento de Anti-Whale + Exit Fee.
    /// Requiere authority + Tridente 3-de-3 (ADR-015). Crea PDA ["mm_registry", mm_pubkey].
    pub fn register_market_maker(ctx: Context<RegisterMM>, mm_pubkey: Pubkey) -> Result<()> {
        let config = &ctx.accounts.config;
        assert_tridente_signed(config, ctx.remaining_accounts)?;
        let now = Clock::get()?.unix_timestamp;
        let reg = &mut ctx.accounts.mm_registry;
        reg.mm = mm_pubkey;
        reg.is_active = true;
        reg.registered_at = now;
        reg.bump = ctx.bumps.mm_registry;
        emit!(MarketMakerRegistered { mm: mm_pubkey, ts: now });
        Ok(())
    }

    /// Revoca un Market Maker y cierra el PDA (devuelve rent a authority).
    /// Requiere authority + Tridente 3-de-3.
    pub fn revoke_market_maker(ctx: Context<RevokeMM>, mm_pubkey: Pubkey) -> Result<()> {
        let config = &ctx.accounts.config;
        assert_tridente_signed(config, ctx.remaining_accounts)?;
        let reg = &ctx.accounts.mm_registry;
        require!(reg.is_active, LukashError::MMRegistryInvalid);
        let now = Clock::get()?.unix_timestamp;
        emit!(MarketMakerRevoked { mm: mm_pubkey, ts: now });
        Ok(())
    }

    /// Drena la cola de quema diferida según el modo Throttle (ADR-016: todos los modos drenan).
    /// ACEL 25% · NORMAL 10% · CONS 5% · DEF 2% por semana. Hard-stop si supply ≤ ENZ.
    /// Permissionless: cualquiera puede gatillarlo si se cumplen las condiciones on-chain.
    pub fn execute_deferred_burn(ctx: Context<ExecuteDeferredBurn>) -> Result<()> {
        require!(!ctx.accounts.config.paused, LukashError::ProtocolPaused);
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;
        let mut deferred_tokens_to_burn: u64 = 0;

        // Circuit Breaker guard (07-c)
        require!(now >= state.cb_active_until_ts, LukashError::CircuitBreakerActive);

        // ENZ hard-stop: si supply ≤ 3.3B, la quema se apaga definitivamente
        if state.current_supply > 0 {
            require!(state.current_supply > SUPPLY_ENZ, LukashError::BurnComplete);
        }

        require!(state.deferred_burn_queue > 0, LukashError::EmptyQueue);
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
        if drain > 0 && state.luka_price > 0 {
            let drain_tokens = usd_to_tokens(drain, state.luka_price)?;
            state.current_supply = state.current_supply.saturating_sub(drain_tokens);
            deferred_tokens_to_burn = drain_tokens;
        }
        state.last_queue_exec_ts = now;

        emit!(DeferredBurnExecuted {
            drained: drain,
            remaining: state.deferred_burn_queue,
            ts: now,
            mode: state.throttle_mode,
        });

        // Capa 2: quema real de la cola diferida
        if deferred_tokens_to_burn > 0 {
            let available = ctx.accounts.burn_vault.amount;
            let actual = deferred_tokens_to_burn.min(available);
            if actual > 0 {
                let bump = ctx.accounts.state.bump;
                let seeds: &[&[u8]] = &[STATE_SEED, &[bump]];
                let signer_seeds = &[seeds];
                token::burn(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(),
                        Burn {
                            mint: ctx.accounts.luka_mint.to_account_info(),
                            from: ctx.accounts.burn_vault.to_account_info(),
                            authority: ctx.accounts.state.to_account_info(),
                        },
                        signer_seeds,
                    ),
                    actual,
                )?;
                emit!(RealBurnExecuted { tokens_requested: deferred_tokens_to_burn, tokens_burned: actual });
            }
        }

        Ok(())
    }

    /// Cierra las PDAs del protocolo y devuelve rent a la authority.
    /// Solo para devnet (migraciones de versión). En mainnet, el programa sería inmutable.
    /// v9-migration: state es UncheckedAccount para cerrar PDAs de versiones anteriores con layout distinto.
    pub fn close_protocol(ctx: Context<CloseProtocol>) -> Result<()> {
        let state_info = ctx.accounts.state.to_account_info();
        let authority_info = ctx.accounts.authority.to_account_info();
        **authority_info.try_borrow_mut_lamports()? += **state_info.try_borrow_lamports()?;
        **state_info.try_borrow_mut_lamports()? = 0;
        state_info.assign(&anchor_lang::solana_program::system_program::ID);
        state_info.realloc(0, false)?;
        Ok(())
    }

    /// Crea el burn_vault (PDA token account para $LUKA). Authority = state PDA.
    /// Llamar después de initialize y después de crear el mint de $LUKA.
    pub fn initialize_burn_vault(_ctx: Context<InitializeBurnVault>) -> Result<()> {
        Ok(())
    }

    /// Ejecuta los swaps pendientes del Vault a precios de oráculo Pyth (Capa 2 Fase C).
    /// Convierte los USD pendientes (acumulados por process_fee) a balances nativos de activos.
    /// Devnet (mock): accounting puro a precio de oráculo. Mainnet: CPI a Jupiter.
    /// Permissionless — el keeper (o cualquiera) puede llamarla.
    pub fn execute_vault_swaps(ctx: Context<ExecuteVaultSwaps>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let now = Clock::get()?.unix_timestamp;

        let total_pending = state.pending_swap_cbtc_usd
            .checked_add(state.pending_swap_sol_usd).ok_or(LukashError::MathOverflow)?
            .checked_add(state.pending_swap_lst_usd).ok_or(LukashError::MathOverflow)?
            .checked_add(state.pending_swap_usdc_res_usd).ok_or(LukashError::MathOverflow)?
            .checked_add(state.pending_swap_usdc_lend_usd).ok_or(LukashError::MathOverflow)?;
        require!(total_pending > 0, LukashError::NoPendingSwaps);

        let btc_price_usd: u64 = {
            let data = ctx.accounts.pyth_btc_feed.try_borrow_data()?;
            let p = match parse_pyth_price(&data, now) {
                Ok((raw, expo)) => pyth_price_to_usd6(raw, expo).unwrap_or(0),
                Err(_) => 0,
            };
            if p > 0 { p } else if DEVNET_MODE { 65_000_000_000 } else { return Err(LukashError::InvalidOracleValue.into()) }
        };
        let sol_price_usd: u64 = {
            let data = ctx.accounts.pyth_sol_feed.try_borrow_data()?;
            let p = match parse_pyth_price(&data, now) {
                Ok((raw, expo)) => pyth_price_to_usd6(raw, expo).unwrap_or(0),
                Err(_) => 0,
            };
            if p > 0 { p } else if DEVNET_MODE { 150_000_000 } else { return Err(LukashError::InvalidOracleValue.into()) }
        };

        // cBTC: pending_usd → satoshis = pending_usd * CBTC_SCALE / btc_price_usd
        let cbtc_native = usd_to_native(state.pending_swap_cbtc_usd, btc_price_usd, CBTC_SCALE)?;
        state.cbtc_amount = state.cbtc_amount.checked_add(cbtc_native).ok_or(LukashError::MathOverflow)?;

        // SOL: pending_usd → lamports = pending_usd * SOL_SCALE / sol_price_usd
        let sol_native = usd_to_native(state.pending_swap_sol_usd, sol_price_usd, SOL_SCALE)?;
        state.sol_amount = state.sol_amount.checked_add(sol_native).ok_or(LukashError::MathOverflow)?;

        // LST: usa precio SOL como proxy (LST ≈ SOL con exchange rate ~1:1 en devnet)
        let lst_native = usd_to_native(state.pending_swap_lst_usd, sol_price_usd, LST_SCALE)?;
        state.lst_amount = state.lst_amount.checked_add(lst_native).ok_or(LukashError::MathOverflow)?;

        // USDC: 1:1 con USD (ambos 6 decimales), no requiere swap
        state.usdc_res_amount = state.usdc_res_amount
            .checked_add(state.pending_swap_usdc_res_usd).ok_or(LukashError::MathOverflow)?;
        state.usdc_lend_amount = state.usdc_lend_amount
            .checked_add(state.pending_swap_usdc_lend_usd).ok_or(LukashError::MathOverflow)?;

        emit!(VaultSwapsExecuted {
            cbtc_native,
            sol_native,
            lst_native,
            usdc_res: state.pending_swap_usdc_res_usd,
            usdc_lend: state.pending_swap_usdc_lend_usd,
            btc_price_usd,
            sol_price_usd,
            total_usd_swapped: total_pending,
            ts: now,
        });

        // Limpiar pendientes
        state.pending_swap_cbtc_usd = 0;
        state.pending_swap_sol_usd = 0;
        state.pending_swap_lst_usd = 0;
        state.pending_swap_usdc_res_usd = 0;
        state.pending_swap_usdc_lend_usd = 0;

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

/// Deserializa un feed Pyth V2 y devuelve (price_raw: i64, exponent: i32).
/// Valida: magic number, tamaño mínimo, status=Trading, staleness.
fn parse_pyth_price(feed_data: &[u8], now: i64) -> Result<(i64, i32)> {
    require!(feed_data.len() >= PYTH_MIN_DATA_LEN, LukashError::InvalidOracleValue);

    let magic = u32::from_le_bytes(
        feed_data[PYTH_MAGIC_OFFSET..PYTH_MAGIC_OFFSET + 4].try_into().unwrap(),
    );
    require!(magic == PYTH_MAGIC, LukashError::InvalidOracleValue);

    let status = u32::from_le_bytes(
        feed_data[PYTH_AGG_STATUS_OFFSET..PYTH_AGG_STATUS_OFFSET + 4].try_into().unwrap(),
    );
    require!(status == PYTH_STATUS_TRADING, LukashError::OracleFeedStale);

    let timestamp = i64::from_le_bytes(
        feed_data[PYTH_TIMESTAMP_OFFSET..PYTH_TIMESTAMP_OFFSET + 8].try_into().unwrap(),
    );
    require!(now - timestamp <= ORACLE_FEED_MAX_STALENESS, LukashError::OracleFeedStale);

    let price = i64::from_le_bytes(
        feed_data[PYTH_AGG_PRICE_OFFSET..PYTH_AGG_PRICE_OFFSET + 8].try_into().unwrap(),
    );
    require!(price > 0, LukashError::InvalidOracleValue);

    let conf = u64::from_le_bytes(
        feed_data[PYTH_AGG_CONF_OFFSET..PYTH_AGG_CONF_OFFSET + 8].try_into().unwrap(),
    );
    let conf_pct_bps = (conf as u128)
        .checked_mul(BPS_DENOMINATOR as u128).ok_or(LukashError::MathOverflow)?
        .checked_div(price as u128).ok_or(LukashError::MathOverflow)?;
    require!(conf_pct_bps <= ORACLE_DEVIATION_BPS_MAX as u128, LukashError::InvalidOracleValue);

    let expo = i32::from_le_bytes(
        feed_data[PYTH_EXPO_OFFSET..PYTH_EXPO_OFFSET + 4].try_into().unwrap(),
    );

    Ok((price, expo))
}

/// Convierte USD 6-dec a unidades nativas de un activo dado su precio y escala.
/// Inversa de compute_asset_value: native = usd * scale / price.
fn usd_to_native(usd_amount: u64, price_usd: u64, scale: u128) -> Result<u64> {
    if usd_amount == 0 || price_usd == 0 {
        return Ok(0);
    }
    let r = (usd_amount as u128)
        .checked_mul(scale).ok_or(LukashError::MathOverflow)?
        .checked_div(price_usd as u128).ok_or(LukashError::MathOverflow)?;
    u64::try_from(r).map_err(|_| LukashError::MathOverflow.into())
}

/// Convierte un precio Pyth (raw × 10^expo) a USD con 6 decimales.
fn pyth_price_to_usd6(price_raw: i64, expo: i32) -> Result<u64> {
    let price = price_raw as u64;
    let adjustment = 6i32 + expo;
    if adjustment >= 0 {
        let scale = 10u64.checked_pow(adjustment as u32).ok_or(LukashError::MathOverflow)?;
        price.checked_mul(scale).ok_or(LukashError::MathOverflow.into())
    } else {
        let scale = 10u64.checked_pow((-adjustment) as u32).ok_or(LukashError::MathOverflow)?;
        Ok(price / scale)
    }
}

/// Verifica que las 3 firmas del Tridente estén presentes en la transacción (07-c).
fn assert_tridente_signed(cfg: &ProtocolConfig, remaining: &[AccountInfo]) -> Result<()> {
    require!(cfg.tridente_activated, LukashError::TridenteNotActivated);
    let s1 = remaining.iter().any(|a| a.key == &cfg.tridente_signer_1 && a.is_signer);
    let s2 = remaining.iter().any(|a| a.key == &cfg.tridente_signer_2 && a.is_signer);
    let s3 = remaining.iter().any(|a| a.key == &cfg.tridente_signer_3 && a.is_signer);
    require!(s1 && s2 && s3, LukashError::TridenteSignaturesIncomplete);
    Ok(())
}

/// Régimen efectivo con fail-safe: si el keeper no actualiza en 48h, usa NEUTRAL (07-e).
fn resolve_regime_effective(state: &ProtocolState, now_ts: i64) -> u8 {
    if state.regime_updated_ts == 0 {
        return REGIME_NEUTRAL;
    }
    let age = now_ts.saturating_sub(state.regime_updated_ts);
    if age > REGIME_MAX_STALENESS {
        REGIME_NEUTRAL
    } else {
        state.market_regime
    }
}

/// Mayoría de 3 señales. Si no hay mayoría (3 valores distintos), retorna NEUTRAL.
fn majority_vote(a: u8, b: u8, c: u8) -> u8 {
    if a == b || a == c { a }
    else if b == c { b }
    else { REGIME_NEUTRAL }
}

/// Anti-Whale: fee en tokens sobre el excedente del umbral 1% del pool (07-f, ADR-012 C10).
/// Tier único (no progresivo): <1% sin fee, 1-2% 3%, 2-5% 6%, >5% 10%.
fn compute_anti_whale_fee(amount: u64, luka_price: u64, pool_liquidity_usd: u64) -> Result<u64> {
    if luka_price == 0 || pool_liquidity_usd == 0 {
        return Ok(0);
    }
    let tx_usd = tokens_to_usd(amount, luka_price)?;
    let tx_pct_bps = (tx_usd as u128)
        .checked_mul(BPS_DENOMINATOR as u128).ok_or(LukashError::MathOverflow)?
        .checked_div(pool_liquidity_usd as u128).ok_or(LukashError::MathOverflow)?;
    let tx_pct = u64::try_from(tx_pct_bps).map_err(|_| LukashError::MathOverflow)?;

    if tx_pct < AW_THR_1_BPS {
        return Ok(0);
    }

    let excedente_bps = tx_pct.saturating_sub(AW_THR_1_BPS);
    let penal_bps = if tx_pct <= AW_THR_2_BPS {
        AW_FEE_1_BPS
    } else if tx_pct <= AW_THR_3_BPS {
        AW_FEE_2_BPS
    } else {
        AW_FEE_3_BPS
    };

    let excedente_tokens = mul_bps(amount, excedente_bps)?;
    mul_bps(excedente_tokens, penal_bps)
}

/// Exit Fee: fee en tokens si ambas condiciones de pánico se cumplen (07-f, v4.3 §9).
/// Condición dual: precio < 0.7×EMA30 AND sell_pressure > 0.3% supply/hora.
fn compute_exit_fee(stage: u8, amount: u64, state: &ProtocolState) -> Result<u64> {
    if state.ema30 == 0 || state.luka_price == 0 {
        return Ok(0);
    }
    let price_threshold = mul_bps(state.ema30, EXIT_FEE_PRICE_TRIG_BPS)?;
    let cond_price = state.luka_price < price_threshold;
    let cond_volume = state.sell_pressure_1h_supply_bps > EXIT_FEE_VOL_TRIG_BPS;

    if !(cond_price && cond_volume) {
        return Ok(0);
    }

    let fee_bps = match stage {
        1 => EXIT_FEE_ET1_BPS,
        2 => EXIT_FEE_ET2_BPS,
        _ => EXIT_FEE_ET3_BPS,
    };
    mul_bps(amount, fee_bps)
}

/// Actualiza la presión de venta acumulada en la ventana rodante de 1h (I22).
fn update_sell_pressure(state: &mut ProtocolState, amount: u64, now_ts: i64) -> Result<()> {
    if now_ts.saturating_sub(state.sell_pressure_last_reset_ts) >= SELL_PRESSURE_WINDOW {
        state.sell_pressure_1h_supply_bps = 0;
        state.sell_pressure_last_reset_ts = now_ts;
    }
    if state.current_supply > 0 {
        let sale_bps = (amount as u128)
            .checked_mul(BPS_DENOMINATOR as u128).ok_or(LukashError::MathOverflow)?
            .checked_div(state.current_supply as u128).ok_or(LukashError::MathOverflow)?;
        let sale_bps_u64 = u64::try_from(sale_bps).map_err(|_| LukashError::MathOverflow)?;
        state.sell_pressure_1h_supply_bps = state.sell_pressure_1h_supply_bps
            .checked_add(sale_bps_u64).ok_or(LukashError::MathOverflow)?;
    }
    Ok(())
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
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub authority: Signer<'info>,
    // Capa 2: quema real
    #[account(mut, seeds = [BURN_VAULT_SEED], bump, token::mint = luka_mint, token::authority = state)]
    pub burn_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub luka_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
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
    // Capa 2: quema real
    #[account(mut, seeds = [BURN_VAULT_SEED], bump, token::mint = luka_mint, token::authority = state)]
    pub burn_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub luka_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

/// Operaciones que requieren Tridente 3-de-3 (los 3 signers van en remaining_accounts).
#[derive(Accounts)]
pub struct TridenteAction<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub caller: Signer<'info>,
}

/// Capa 2: restringido a authority. BTC/SOL vía Pyth feeds; LST/LUKA como parámetros.
#[derive(Accounts)]
pub struct RefreshVaultValuation<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    /// CHECK: Pyth BTC/USD price feed — owner validado como pyth_oracle::ID + magic/staleness/status en instrucción
    #[account(owner = pyth_oracle::ID @ LukashError::InvalidOracleValue)]
    pub pyth_btc_feed: AccountInfo<'info>,
    /// CHECK: Pyth SOL/USD price feed — owner validado como pyth_oracle::ID + magic/staleness/status en instrucción
    #[account(owner = pyth_oracle::ID @ LukashError::InvalidOracleValue)]
    pub pyth_sol_feed: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

/// Ejecuta swaps pendientes del Vault a precios de oráculo Pyth (Capa 2 Fase C).
#[derive(Accounts)]
pub struct ExecuteVaultSwaps<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    /// CHECK: Pyth BTC/USD price feed — owner validado como pyth_oracle::ID + magic/staleness/status en instrucción
    #[account(owner = pyth_oracle::ID @ LukashError::InvalidOracleValue)]
    pub pyth_btc_feed: AccountInfo<'info>,
    /// CHECK: Pyth SOL/USD price feed — owner validado como pyth_oracle::ID + magic/staleness/status en instrucción
    #[account(owner = pyth_oracle::ID @ LukashError::InvalidOracleValue)]
    pub pyth_sol_feed: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

/// Transfer Hook Capa 1: authority alimenta parámetros de contexto manualmente.
/// Capa 2 (Token-2022): invocado por el Token Program, cuentas on-chain reemplazan los args.
#[derive(Accounts)]
pub struct TransferHookCtx<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(mut, seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    pub authority: Signer<'info>,
}

/// Registro de Market Maker (Tridente 3-de-3 requerido via remaining_accounts).
#[derive(Accounts)]
#[instruction(mm_pubkey: Pubkey)]
pub struct RegisterMM<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(
        init,
        payer = authority,
        space = 8 + MMRegistry::INIT_SPACE,
        seeds = [MM_REGISTRY_SEED, mm_pubkey.as_ref()],
        bump,
    )]
    pub mm_registry: Account<'info, MMRegistry>,
    pub system_program: Program<'info, System>,
}

/// Crea el burn_vault PDA token account. Llamar una vez post-initialize.
#[derive(Accounts)]
pub struct InitializeBurnVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(seeds = [STATE_SEED], bump = state.bump)]
    pub state: Account<'info, ProtocolState>,
    #[account(
        init,
        payer = authority,
        token::mint = luka_mint,
        token::authority = state,
        seeds = [BURN_VAULT_SEED],
        bump,
    )]
    pub burn_vault: Account<'info, TokenAccount>,
    pub luka_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Cierra PDAs del protocolo (devnet migration). Authority-only.
/// State es UncheckedAccount para soportar migración entre versiones con layout distinto.
#[derive(Accounts)]
pub struct CloseProtocol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, close = authority, seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    /// CHECK: Legacy state PDA — puede tener layout de versión anterior. Validado por seeds.
    #[account(mut, seeds = [STATE_SEED], bump)]
    pub state: UncheckedAccount<'info>,
}

/// Revocación de Market Maker — cierra PDA y devuelve rent (Tridente 3-de-3 via remaining_accounts).
#[derive(Accounts)]
#[instruction(mm_pubkey: Pubkey)]
pub struct RevokeMM<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ LukashError::Unauthorized)]
    pub config: Account<'info, ProtocolConfig>,
    #[account(
        mut,
        close = authority,
        seeds = [MM_REGISTRY_SEED, mm_pubkey.as_ref()],
        bump = mm_registry.bump,
    )]
    pub mm_registry: Account<'info, MMRegistry>,
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
    pub btc_price_usd: u64,
    pub sol_price_usd: u64,
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

// Sprint 5B Capa 2: quema real
#[event]
pub struct RealBurnExecuted {
    pub tokens_requested: u64,
    pub tokens_burned: u64,
}

// Sprint 5B Capa 2: swaps a precio de oráculo
#[event]
pub struct VaultSwapsExecuted {
    pub cbtc_native: u64,
    pub sol_native: u64,
    pub lst_native: u64,
    pub usdc_res: u64,
    pub usdc_lend: u64,
    pub btc_price_usd: u64,
    pub sol_price_usd: u64,
    pub total_usd_swapped: u64,
    pub ts: i64,
}

// 07-c: Tridente + Circuit Breaker + Seguro
#[event]
pub struct TridenteActivated {
    pub pk1: Pubkey,
    pub pk2: Pubkey,
    pub pk3: Pubkey,
    pub ts: i64,
}

#[event]
pub struct CircuitBreakerTriggered {
    pub prev_usd: u64,
    pub current_usd: u64,
    pub paused_until: i64,
}

#[event]
pub struct CircuitBreakerCancelled {
    pub cancelled_at: i64,
    pub was_until: i64,
}

#[event]
pub struct InsuranceRecoveryReceived {
    pub amount_usdc: u64,
    pub cap_at_event: u64,
    pub ts: i64,
}

// 07-e: Módulo contra-cíclico
#[event]
pub struct MarketRegimeUpdated {
    pub regime: u8,
    pub ema30_btc: u64,
    pub ema90_btc: u64,
    pub vol_30d_bps: u64,
    pub vol_a_7d: u64,
    pub vol_a_30d: u64,
    pub signals: [u8; 3],
    pub ts: i64,
}

// 07-f: Anti-Whale + Exit Fee + Transfer Hook
#[event]
pub struct AntiWhaleTriggered {
    pub sender: Pubkey,
    pub amount_luka: u64,
    pub tx_pct_pool_bps: u64,
    pub excedente_luka: u64,
    pub fee_luka: u64,
}

#[event]
pub struct ExitFeeTriggered {
    pub sender: Pubkey,
    pub amount_luka: u64,
    pub luka_price: u64,
    pub ema30: u64,
    pub fee_luka: u64,
    pub stage: u8,
}

#[event]
pub struct ShieldFeeCollected {
    pub sender: Pubkey,
    pub fee_luka: u64,
    pub fee_usdc_to_vault: u64,
    pub source: u8,  // 0=Anti-Whale, 1=Exit Fee, 2=ambos
}

#[event]
pub struct TransferInspected {
    pub sender: Pubkey,
    pub amount: u64,
    pub fee_aw: u64,
    pub fee_exit: u64,
    pub exempt: bool,
    pub is_buy: bool,
}

#[event]
pub struct MarketMakerRegistered {
    pub mm: Pubkey,
    pub ts: i64,
}

#[event]
pub struct MarketMakerRevoked {
    pub mm: Pubkey,
    pub ts: i64,
}
