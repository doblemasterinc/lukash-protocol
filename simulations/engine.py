"""
LUKASH — Motor de simulación FIEL AL CONTRATO (lib.rs v10.2, Protocolo v4.3).

Este módulo replica, línea por línea, la aritmética entera del smart contract
`contracts/playground/lib.rs`. Todos los montos USD se manejan en enteros de
6 decimales (1 USD = 1_000_000), igual que on-chain, para reproducir el MISMO
comportamiento de redondeo (división entera, dust a staking, cola diferida).

NO añade nada que el contrato no haga. La capa económica (precio, supply,
P_KASH, realimentación de mercado) vive en `market.py` — el contrato no la tiene.

Objetivo: que "correr la simulación" sea equivalente a "ejecutar el contrato"
en cuanto a fees, distribución, conmutación B0/B2, Throttle y cola de quema.
"""
from dataclasses import dataclass, field

# ================= constantes (espejo de lib.rs §constantes) =================
BPS_DENOMINATOR = 10_000
USD = 1_000_000  # 6 decimales

# Distribución universal 35/35/15/15
DIST_VAULT_BPS = 3_500
DIST_LP_BURN_BPS = 3_500
DIST_OM_BPS = 1_500
DIST_STAKING_BPS = 1_500

# Split de Fee del Asset Layer: Core / Sociedad (70/30)
FEE_SPLIT_CORE_BPS = 7_000

# Composición objetivo del Vault Core (suma 10000)
VAULT_CBTC_BPS = 3_500
VAULT_SOL_BPS = 1_500
VAULT_LST_BPS = 2_000
VAULT_USDC_RES_BPS = 2_500
VAULT_USDC_LEND_BPS = 500

# Umbrales de valor (sobre el KASH Core), USD 6 dec
K_MIN_USD = 25_000_000 * USD
KASH_LOCK_USD = 30_000_000 * USD
ETAPA3_USD = 50_000_000 * USD
SCALE_USD = 100_000_000 * USD

KASH_LOCK_SECONDS = 365 * 24 * 60 * 60

# Throttle: umbrales como % de la EMA30 (bps) y % de quema (bps)
THROTTLE_ACCEL_BPS = 12_000  # P > 1.2x EMA30
THROTTLE_CONS_BPS = 8_000    # P < 0.8x EMA30
THROTTLE_DEF_BPS = 5_000     # P < 0.5x EMA30

BURN_ACCEL_BPS = 12_500  # 125%
BURN_NORMAL_BPS = 10_000  # 100%
BURN_CONS_BPS = 6_000     # 60%
BURN_DEF_BPS = 2_500      # 25%

TIMELOCK_SECONDS = 48 * 60 * 60
WEEK_SECONDS = 7 * 24 * 60 * 60
QUEUE_DRAIN_ACCEL_BPS = 2_500  # 25%/sem — euforia (ADR-016)
QUEUE_DRAIN_NORMAL_BPS = 1_000  # 10%/sem — estable
QUEUE_DRAIN_CONS_BPS = 500      #  5%/sem — bajista
QUEUE_DRAIN_DEF_BPS = 200       #  2%/sem — depresión

# Modos del Motor B
MOTOR_B_B0 = 0
MOTOR_B_B2 = 1

# Modos del Throttle
THROTTLE_ACCELERATED = 0
THROTTLE_NORMAL = 1
THROTTLE_CONSERVATIVE = 2
THROTTLE_DEFENSIVE = 3

THROTTLE_NAMES = {0: "ACELERADO", 1: "NORMAL", 2: "CONSERVADOR", 3: "DEFENSIVO"}


# ------------------------- helpers puros (espejo de lib.rs) -------------------------
def mul_bps(amount: int, bps: int) -> int:
    """amount * bps / 10000 con división entera (u128 intermedio en Rust)."""
    return (amount * bps) // BPS_DENOMINATOR


def compute_fee_bps(stage: int, motor: int, layer: int, currency: int, is_wl: bool) -> int:
    """Fee en bps. Lanza ValueError si el motor no está activo en la etapa (== require! en Rust)."""
    if motor == 0:  # Motor A
        if stage == 1:
            return 250 if is_wl else 400
        return 150 if is_wl else 250
    elif motor == 1:  # Motor B
        if stage < 2:
            raise ValueError("MotorNotActiveInStage: B requiere Etapa>=2A")
        return 150 if is_wl else 250
    elif motor == 2:  # Motor C
        if stage < 3:
            raise ValueError("MotorNotActiveInStage: C requiere Etapa>=3")
        return 50
    elif motor == 3:  # Motor D
        if stage < 2:
            raise ValueError("MotorNotActiveInStage: D requiere Etapa>=2A")
        if layer == 0:
            return 150  # ADR-023: Capa 0 eliminada → fee mínimo 1.5% (Tótem Nativo)
        elif layer == 1:
            return 150
        elif layer == 2:
            return 300 if currency == 0 else 350
        elif layer == 3:
            return 150
        elif layer == 4:
            return 200
        raise ValueError("InvalidMotorOrLayer")
    raise ValueError("InvalidMotorOrLayer")


def throttle_burn_bps(mode: int) -> int:
    return {
        THROTTLE_ACCELERATED: BURN_ACCEL_BPS,
        THROTTLE_NORMAL: BURN_NORMAL_BPS,
        THROTTLE_CONSERVATIVE: BURN_CONS_BPS,
        THROTTLE_DEFENSIVE: BURN_DEF_BPS,
    }.get(mode, BURN_NORMAL_BPS)


def throttle_mode_from_price(price: int, ema30: int) -> int:
    """Modo del Throttle según precio vs EMA30. Espejo exacto de la función Rust."""
    if ema30 == 0:
        return THROTTLE_NORMAL
    ratio_bps = (price * BPS_DENOMINATOR) // ema30
    if ratio_bps > THROTTLE_ACCEL_BPS:
        return THROTTLE_ACCELERATED
    elif ratio_bps >= THROTTLE_CONS_BPS:
        return THROTTLE_NORMAL
    elif ratio_bps >= THROTTLE_DEF_BPS:
        return THROTTLE_CONSERVATIVE
    return THROTTLE_DEFENSIVE


# ------------------------- estado del protocolo (espejo de ProtocolState/Config) -------------------------
@dataclass
class ProtocolState:
    # Config
    stage: int = 1
    paused: bool = False
    k_min_usd: int = K_MIN_USD
    kash_lock_usd: int = KASH_LOCK_USD

    # Runtime
    genesis_ts: int = 0
    motor_b_state: int = MOTOR_B_B0
    throttle_mode: int = THROTTLE_NORMAL
    luka_price: int = 0
    ema30: int = 0

    # Vault
    vault_core_usd: int = 0
    vault_sociedad_usd: int = 0
    r_op_usd: int = 0
    cbtc_usd: int = 0
    sol_usd: int = 0
    lst_usd: int = 0
    usdc_res_usd: int = 0
    usdc_lend_usd: int = 0

    # Acumuladores de flujo
    burned_total: int = 0        # USD de $LUKA comprado y quemado
    recirculated_total: int = 0  # USD recirculado al LP (B2)
    staking_total: int = 0
    om_total: int = 0
    deferred_burn_queue: int = 0  # USD en cola de quema diferida

    # Hitos
    kash_lock_hit: bool = False
    last_queue_exec_ts: int = 0

    # --- métricas de instrumentación (no están en el contrato; para el análisis) ---
    burn_now_usd_step: int = 0       # cuánto se quemó en el último process_fee
    deferred_added_step: int = 0     # cuánto entró a la cola en el último process_fee
    recirculated_step: int = 0

    def invariant_composition_ok(self) -> bool:
        return (VAULT_CBTC_BPS + VAULT_SOL_BPS + VAULT_LST_BPS +
                VAULT_USDC_RES_BPS + VAULT_USDC_LEND_BPS) == BPS_DENOMINATOR

    def invariant_distribution_ok(self) -> bool:
        return (DIST_VAULT_BPS + DIST_LP_BURN_BPS + DIST_OM_BPS +
                DIST_STAKING_BPS) == BPS_DENOMINATOR


@dataclass
class FeeResult:
    amount: int
    fee: int
    to_vault: int = 0
    to_lp_burn: int = 0
    to_om: int = 0
    to_staking: int = 0
    core: int = 0
    sociedad: int = 0
    burn_now: int = 0
    deferred: int = 0
    recirculated: int = 0


# ------------------------- instrucciones del contrato (espejo de #[program]) -------------------------
def process_fee(st: ProtocolState, amount: int, motor: int, layer: int,
                currency: int, is_whitelist: bool, now_ts: int) -> FeeResult:
    """
    Universal Fee Extractor. Espejo EXACTO de process_fee en lib.rs.
    `amount` = notional USD (6 dec). Devuelve el desglose para instrumentación.
    Lanza ValueError donde el contrato haría require!/err!.
    """
    if st.paused:
        raise ValueError("ProtocolPaused")
    if amount <= 0:
        raise ValueError("ZeroAmount")
    if currency > 1:
        raise ValueError("InvalidCurrency")

    fee_bps = compute_fee_bps(st.stage, motor, layer, currency, is_whitelist)
    fee = mul_bps(amount, fee_bps)

    st.burn_now_usd_step = 0
    st.deferred_added_step = 0
    st.recirculated_step = 0

    if fee == 0:
        return FeeResult(amount=amount, fee=0)

    # --- Distribución 35/35/15/15 (resto a staking para no perder dust) ---
    to_vault = mul_bps(fee, DIST_VAULT_BPS)
    to_lp_burn = mul_bps(fee, DIST_LP_BURN_BPS)
    to_om = mul_bps(fee, DIST_OM_BPS)
    to_staking = fee - to_vault - to_lp_burn - to_om

    # Invariante: la suma exacta debe ser el fee.
    assert to_vault + to_lp_burn + to_om + to_staking == fee, "DistributionInvariant"

    # --- Asset Layer (35%): split Core/Sociedad + asignación por activo del Core ---
    core = mul_bps(to_vault, st.__dict__.get("fee_split_core_bps", FEE_SPLIT_CORE_BPS))
    sociedad = to_vault - core
    st.vault_core_usd += core
    st.vault_sociedad_usd += sociedad

    a_cbtc = mul_bps(core, VAULT_CBTC_BPS)
    a_sol = mul_bps(core, VAULT_SOL_BPS)
    a_lst = mul_bps(core, VAULT_LST_BPS)
    a_usdc_res = mul_bps(core, VAULT_USDC_RES_BPS)
    a_usdc_lend = core - a_cbtc - a_sol - a_lst - a_usdc_res
    st.cbtc_usd += a_cbtc
    st.sol_usd += a_sol
    st.lst_usd += a_lst
    st.usdc_res_usd += a_usdc_res
    st.usdc_lend_usd += a_usdc_lend

    # --- LP / Quema (35%) ---
    burn_now = 0
    deferred = 0
    recirc = 0
    if motor == 0 or motor == 2:
        # Motor A y C: quema/LP completo, sin Throttle.
        st.burned_total += to_lp_burn
        burn_now = to_lp_burn
    elif st.motor_b_state == MOTOR_B_B0:
        burn_bps = min(throttle_burn_bps(st.throttle_mode), BPS_DENOMINATOR)
        burn_now = mul_bps(to_lp_burn, burn_bps)
        deferred = to_lp_burn - burn_now
        st.burned_total += burn_now
        st.deferred_burn_queue += deferred
    else:
        st.recirculated_total += to_lp_burn
        recirc = to_lp_burn

    st.burn_now_usd_step = burn_now
    st.deferred_added_step = deferred
    st.recirculated_step = recirc

    # --- O&M (15%) y Staking (15%) ---
    st.om_total += to_om
    st.staking_total += to_staking

    # --- Hito KASH Lock: KASH Core >= $30M O 12 meses ---
    if not st.kash_lock_hit:
        elapsed = max(0, now_ts - st.genesis_ts)
        if st.vault_core_usd >= st.kash_lock_usd or elapsed >= KASH_LOCK_SECONDS:
            st.kash_lock_hit = True

    return FeeResult(amount=amount, fee=fee, to_vault=to_vault, to_lp_burn=to_lp_burn,
                     to_om=to_om, to_staking=to_staking, core=core, sociedad=sociedad,
                     burn_now=burn_now, deferred=deferred, recirculated=recirc)


def update_oracle_state(st: ProtocolState, luka_price: int, ema30: int) -> None:
    """Espejo de update_oracle_state: fija precio/EMA30 y recalcula el Throttle."""
    if luka_price <= 0 or ema30 <= 0:
        raise ValueError("InvalidOracleValue")
    st.luka_price = luka_price
    st.ema30 = ema30
    st.throttle_mode = throttle_mode_from_price(luka_price, ema30)


def switch_motor_b(st: ProtocolState, now_ts: int) -> bool:
    """Conmuta B0→B2 si K(t) >= K_min. Devuelve True si conmutó."""
    if st.paused:
        raise ValueError("ProtocolPaused")
    if st.motor_b_state != MOTOR_B_B0:
        return False  # AlreadyB2
    if st.vault_core_usd < st.k_min_usd:
        return False  # KminNotReached
    st.motor_b_state = MOTOR_B_B2
    return True


def execute_deferred_burn(st: ProtocolState, now_ts: int) -> int:
    """
    Drena la cola según modo Throttle (ADR-016: TODOS los modos drenan).
    ACEL 25% · NORMAL 10% · CONS 5% · DEF 2% por semana. Espejo de lib.rs v10.2.
    Devuelve el monto drenado (0 si no se cumplen condiciones).
    """
    if st.paused:
        return 0
    if st.deferred_burn_queue <= 0:
        return 0
    if (now_ts - st.last_queue_exec_ts) < WEEK_SECONDS:
        return 0
    drain_bps = {
        THROTTLE_ACCELERATED: QUEUE_DRAIN_ACCEL_BPS,
        THROTTLE_NORMAL: QUEUE_DRAIN_NORMAL_BPS,
        THROTTLE_CONSERVATIVE: QUEUE_DRAIN_CONS_BPS,
        THROTTLE_DEFENSIVE: QUEUE_DRAIN_DEF_BPS,
    }.get(st.throttle_mode, QUEUE_DRAIN_DEF_BPS)
    drain = max(1, mul_bps(st.deferred_burn_queue, drain_bps))
    drain = min(drain, st.deferred_burn_queue)
    st.deferred_burn_queue -= drain
    st.burned_total += drain
    st.last_queue_exec_ts = now_ts
    return drain
