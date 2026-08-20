//! Constantes del protocolo LUKASH (valores de lanzamiento del Blueprint v4.3 §13).
//! Todos los montos USD usan 6 decimales (1 USD = 1_000_000).

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
