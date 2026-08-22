"""
LUKASH — Capa económica y de mercado (lo que el contrato NO modela).

El contrato (engine.py) sabe de fees, distribución, Vault, Throttle y cola.
NO sabe de precio, supply ni P_KASH — eso es off-chain / oráculo. Aquí vive
el lazo de realimentación que decide si el diseño se sostiene:

    volumen → fees → {quema, buy-pressure} → supply↓ / precio → EMA30 → Throttle → quema...

Toda la parametrización es EXPLÍCITA y conservadora. Este es un "modelo de
simulación de motores v1"; sus cifras NO son las cifras Monte Carlo oficiales
(ver ADR-008 / auditoría H10). Sirve para responder: ¿el protocolo funciona?
"""
from dataclasses import dataclass, field
import math
import engine as E


# ======================= parámetros del escenario =======================
@dataclass
class ScenarioParams:
    name: str = "base"
    days: int = 1825  # 5 años

    # --- Precio / liquidez ---
    tge_price_usd: float = 0.0001       # precio TGE
    lp_depth_usd0: float = 300_000.0    # profundidad inicial del pool (USD por lado)
    price_impact_k: float = 0.35        # sensibilidad del precio al flujo neto (adimensional)
    ema_alpha: float = 2 / (30 + 1)     # EMA30

    # --- Volumen Motor A (trading DEX, SOL) ---
    volA_base_usd: float = 5_000_000.0  # volumen diario base de Motor A al inicio (USD)
    volA_growth: float = 0.0            # crecimiento diario compuesto adicional (0 = plano salvo regímenes)

    # --- Adopción (usuarios de la App → Motores B/C/D) ---
    users_cap: float = 500_000.0        # techo logístico de usuarios
    users_k: float = 0.010              # velocidad de la logística
    users_midpoint_day: float = 540.0   # día del punto de inflexión (~mes 18)
    vol_per_user_usd: float = 8.0       # volumen diario por usuario activo (USD), motores internos

    # --- Mezcla del Motor D por capa (fracción del volumen interno que va a cada capa) ---
    #   capa 0 (0%), 1 (1.5%), 2 (3%/3.5%), 3A (1.5%), 3B (2%)
    d_layer_mix: tuple = (0.15, 0.10, 0.55, 0.10, 0.10)
    d_luka_fraction: float = 0.7        # fracción de D pagada en $LUKA (currency=0) vs SOL/USDC

    # --- Presión de venta / compra orgánica ---
    organic_buy_frac: float = 0.6       # fracción del volumen A que es dinero nuevo entrando (buy neto base)
    base_sell_frac: float = 0.004       # % del market cap circulante vendido por día (toma de ganancia base)
    reflex_sell_mult: float = 3.0       # multiplicador de venta cuando precio < EMA30 (reflexividad / pánico)

    # --- Régimen de mercado por tramos (día_inicio, día_fin, mult_volumen, sesgo_precio_diario) ---
    #   mult_volumen escala el volumen; sesgo_precio es deriva exógena diaria del precio.
    regimes: tuple = (
        (0, 1825, 1.0, 0.0),  # neutral todo el horizonte por defecto
    )

    # --- Toggles de mecánica (para probar hipótesis del diseño) ---
    apply_daily_burn_cap: bool = True   # cap 1% supply/día (spec v4.3 §13, NO en el contrato aún)
    daily_burn_cap_frac: float = 0.01
    app_launch_day: int = 90            # Etapa 1→2A (lanzamiento de la App)
    seed_liquidity_recirc: bool = True  # en B2, la recirculación LP suma profundidad al pool


def logistic_users(p: ScenarioParams, day: int) -> float:
    if day < p.app_launch_day:
        return 0.0
    x = p.users_k * (day - p.users_midpoint_day)
    return p.users_cap / (1.0 + math.exp(-x))


def regime_at(p: ScenarioParams, day: int):
    for (d0, d1, volm, pxbias) in p.regimes:
        if d0 <= day < d1:
            return volm, pxbias
    return 1.0, 0.0


# ======================= estado de mercado =======================
@dataclass
class MarketState:
    price: float = 0.0
    ema30: float = 0.0
    supply: float = 10_000_000_000.0   # 10B tokens
    lp_depth_usd: float = 0.0
    # acumuladores para reporte
    tokens_burned_total: float = 0.0
    day: int = 0


# ======================= driver diario =======================
class Simulation:
    """
    Orquesta un día del protocolo:
      1. calcula volúmenes por motor según etapa/adopción/régimen
      2. llama a engine.process_fee por cada motor activo (misma lógica on-chain)
      3. traduce los flujos USD a presión de compra/venta → nuevo precio
      4. convierte la quema USD en tokens quemados → nuevo supply
      5. actualiza EMA30 y el Throttle (engine.update_oracle_state)
      6. dispara switch B0→B2, drena cola diferida, avanza etapa
    """

    def __init__(self, p: ScenarioParams):
        self.p = p
        self.st = E.ProtocolState(genesis_ts=0, last_queue_exec_ts=0)
        self.m = MarketState(price=p.tge_price_usd, ema30=p.tge_price_usd,
                             lp_depth_usd=p.lp_depth_usd0)
        self.history = []  # lista de dicts por día

    # ---- volúmenes por motor ----
    def _volumes(self, day: int):
        p = self.p
        volm, _ = regime_at(p, day)
        stage = self.st.stage
        volA = p.volA_base_usd * volm * ((1 + p.volA_growth) ** day)
        users = logistic_users(p, day)
        internal_vol = users * p.vol_per_user_usd * volm
        # Reparto de volumen interno: Motor B ~40%, Motor D ~50%, (Motor C aparece en Et.3)
        volB = internal_vol * 0.40 if stage in (2, 3, 4) else 0.0
        volD = internal_vol * 0.50 if stage in (2, 3, 4) else 0.0
        volC = internal_vol * 0.60 if stage in (3, 4) else 0.0  # pagos masivos, alto volumen bajo fee
        return volA, volB, volC, volD, users

    # ---- un día ----
    def step(self, day: int):
        p, st, m = self.p, self.st, self.m
        now_ts = day * 24 * 60 * 60
        st.genesis_ts = 0
        m.day = day

        volA, volB, volC, volD, users = self._volumes(day)

        buy_pressure = 0.0   # USD que compran $LUKA (quema + staking + recirc)
        fee_total_usd = 0.0
        fees_by_motor = {"A": 0, "B": 0, "C": 0, "D": 0}
        burn_usd_today = 0.0

        def run_motor(vol_usd, motor, layer=0, currency=1):
            nonlocal buy_pressure, fee_total_usd, burn_usd_today
            if vol_usd <= 0:
                return 0
            amount = int(round(vol_usd * E.USD))
            if amount <= 0:
                return 0
            try:
                r = E.process_fee(st, amount, motor, layer, currency, False, now_ts)
            except ValueError:
                return 0
            fee_total_usd += r.fee / E.USD
            # buy pressure: quema (compra $LUKA para quemar) + staking (compra $LUKA) + recirc (vuelve al pool)
            buy_pressure += (r.burn_now + r.to_staking + r.recirculated) / E.USD
            burn_usd_today += r.burn_now / E.USD
            return r.fee

        # Motor A (SOL) — siempre
        fees_by_motor["A"] = run_motor(volA, 0, currency=1)
        # Motor B ($LUKA) — Etapa 2A+
        fees_by_motor["B"] = run_motor(volB, 1, currency=0)
        # Motor C (USDC) — Etapa 3+
        fees_by_motor["C"] = run_motor(volC, 2, currency=1)
        # Motor D (multi, por capa) — Etapa 2A+
        if volD > 0:
            for i, frac in enumerate(p.d_layer_mix):
                layer_vol = volD * frac
                # capa i -> layer index (0,1,2,3A=3,3B=4)
                layer = i  # 0,1,2,3,4 ya mapea 0/1/2/3A/3B
                cur = 0 if (i in (1, 2, 3) and (i != 2 or True)) else 1
                # currency: capa1 y 3A son $LUKA; capa2 mezcla; capa3B es SOL/USDC
                if i == 2:
                    # dividir capa 2 en $LUKA y SOL/USDC
                    run_motor(layer_vol * p.d_luka_fraction, 3, layer=2, currency=0)
                    run_motor(layer_vol * (1 - p.d_luka_fraction), 3, layer=2, currency=1)
                    continue
                cur = 0 if i in (1, 3) else 1  # capa1/3A en $LUKA, capa0/3B currency indiferente/SOL
                fees_by_motor["D"] += run_motor(layer_vol, 3, layer=layer, currency=cur)

        # ---- cap de quema diaria (spec §13; el contrato NO lo tiene aún) ----
        burn_cap_hit = False
        if p.apply_daily_burn_cap and m.price > 0:
            max_burn_tokens = p.daily_burn_cap_frac * m.supply
            max_burn_usd = max_burn_tokens * m.price
            if burn_usd_today > max_burn_usd:
                burn_cap_hit = True
                burn_usd_today = max_burn_usd  # el exceso se difiere (simplificación del "al día siguiente")

        # ---- Yield del Vault (Split de Yield 70/30) ----
        # Solo la porción con rendimiento: LST 20% + USDC lending 5% = 25% del Core, blended ~7% APY.
        yield_daily = (st.vault_core_usd * (E.VAULT_LST_BPS + E.VAULT_USDC_LEND_BPS) // E.BPS_DENOMINATOR) * 700 // E.BPS_DENOMINATOR // 365
        st.vault_core_usd += (yield_daily * 70) // 100      # 70% reinversión compuesta
        st.r_op_usd += (yield_daily * 30) // 100            # 30% a R_op (idle)
        # Vault Sociedad 100% capitalizable
        st.vault_sociedad_usd += (st.vault_sociedad_usd * 700 // E.BPS_DENOMINATOR) // 365

        # ---- presión de venta (exógena + reflexiva) ----
        _, px_bias = regime_at(p, day)
        mktcap = m.supply * m.price
        sell_frac = p.base_sell_frac
        if m.price < m.ema30:
            sell_frac *= p.reflex_sell_mult  # pánico / reflexividad
        sell_pressure = sell_frac * mktcap
        organic_buy = p.organic_buy_frac * volA * regime_at(p, day)[0] * 0.02  # dinero nuevo (fracción del vol A)

        # ---- impacto de precio ----
        # profundidad del pool: crece con recirculación en B2
        if p.seed_liquidity_recirc:
            m.lp_depth_usd += (st.recirculated_step / E.USD) * 0.5
        depth = max(m.lp_depth_usd, 1.0)
        net_flow = (buy_pressure + organic_buy) - sell_pressure
        price_change = p.price_impact_k * net_flow / depth
        # limitar cambios diarios extremos (circuit-breaker de mercado ±30%)
        price_change = max(-0.30, min(0.30, price_change))
        new_price = m.price * (1.0 + price_change) + px_bias

        # ---- P_KASH: piso institucional ----
        p_kash = (st.vault_core_usd / E.USD) / max(m.supply, 1.0)
        new_price = max(new_price, p_kash)  # el muro: nunca por debajo del respaldo
        new_price = max(new_price, 1e-9)
        m.price = new_price

        # ---- quema efectiva → supply ----
        tokens_burned = 0.0
        if m.price > 0:
            tokens_burned = burn_usd_today / m.price
        new_supply = max(3_300_000_000.0, m.supply - tokens_burned)
        tokens_burned = m.supply - new_supply
        m.tokens_burned_total += tokens_burned
        m.supply = new_supply

        # ---- EMA30 ----
        m.ema30 = p.ema_alpha * m.price + (1 - p.ema_alpha) * m.ema30

        # ---- actualizar oráculo/Throttle en el contrato (precio en 6 dec entero) ----
        price_i = max(1, int(round(m.price * E.USD)))
        ema_i = max(1, int(round(m.ema30 * E.USD)))
        E.update_oracle_state(st, price_i, ema_i)

        # ---- switch B0→B2 y avance de etapa ----
        if st.stage == 1 and day >= p.app_launch_day:
            st.stage = 2
        if st.stage == 2 and st.motor_b_state == E.MOTOR_B_B0:
            switched = E.switch_motor_b(st, now_ts)
        if st.vault_core_usd >= E.ETAPA3_USD and st.stage < 3:
            st.stage = 3
        if m.supply <= 3_300_000_000.0 and st.stage < 4:
            st.stage = 4

        # ---- drenar cola diferida (semanal) ----
        drained = E.execute_deferred_burn(st, now_ts)
        if drained > 0 and m.price > 0:
            dt = (drained / E.USD) / m.price
            dt = min(dt, m.supply - 3_300_000_000.0)
            m.supply -= dt
            m.tokens_burned_total += dt

        # ---- registro ----
        self.history.append({
            "day": day, "stage": st.stage, "motor_b": st.motor_b_state,
            "throttle": st.throttle_mode, "throttle_name": E.THROTTLE_NAMES[st.throttle_mode],
            "price": m.price, "ema30": m.ema30, "p_kash": p_kash,
            "price_over_pkash": (m.price / p_kash) if p_kash > 0 else float("nan"),
            "supply": m.supply, "tokens_burned_total": m.tokens_burned_total,
            "vault_core_usd": st.vault_core_usd / E.USD,
            "vault_sociedad_usd": st.vault_sociedad_usd / E.USD,
            "r_op_usd": st.r_op_usd / E.USD,
            "burned_total_usd": st.burned_total / E.USD,
            "recirculated_total_usd": st.recirculated_total / E.USD,
            "staking_total_usd": st.staking_total / E.USD,
            "om_total_usd": st.om_total / E.USD,
            "deferred_queue_usd": st.deferred_burn_queue / E.USD,
            "fee_total_usd": fee_total_usd, "users": users,
            "buy_pressure": buy_pressure, "sell_pressure": sell_pressure,
            "lp_depth_usd": m.lp_depth_usd, "burn_cap_hit": burn_cap_hit,
            "jaguar_lock_hit": st.jaguar_lock_hit,
        })

    def run(self):
        for day in range(1, self.p.days + 1):
            self.step(day)
        return self.history
