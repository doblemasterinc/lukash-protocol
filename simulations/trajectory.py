"""
LUKASH — Simulador de UNA trayectoria de 5 años, conduciendo el MOTOR FIEL AL
CONTRATO (engine.py = lib.rs) con la capa económica (economic.py).

Cada día:
  1. régimen Markov + precios BTC/SOL (apreciación real del Vault)
  2. yield del Vault (reinversión 70% / R_op 30%)   [keeper off-chain]
  3. volumen por motor → engine.process_fee  (distribución 35/35/15/15 EXACTA on-chain)
  4. quema USD→tokens con cap 1%/día → supply
  5. switch B0→B2 (K≥K_min), drenaje de cola diferida, avance de etapa
  6. precio $LUKA = P_KASH × prima; EMA30; Throttle para el día siguiente
  7. presión de vesting; detección de espiral (trampa B0) y ruina

DECISIONES DE FIDELIDAD (documentadas):
  · K(t) para decisiones usa el Vault VALORIZADO (apreciación de activos), como en
    el informe. El contrato Milestone-1 guarda vault_core_usd = fees depositados
    (sin apreciar) → se registra `dia_switch_contrato` para cuantificar la brecha
    (el switch on-chain necesitará valuación por oráculo en Milestone 2).
  · La asignación al Vault usa la composición FIJA del contrato (cBTC35/SOL15/
    LST20/USDCr25/lend5). El módulo contra-cíclico de LUKAI (rebalanceo por régimen)
    NO está en el contrato Milestone-1 → no se simula aquí (se valida el contrato tal cual).
"""
import numpy as np
import engine as E
import economic as EC


def _K_valorizado(st, pbtc_r, psol_r):
    """Vault Core valorizado en USD (apreciación de activos). Buckets en micro-USD → USD."""
    cbtc = st.cbtc_usd / E.USD
    sol = st.sol_usd / E.USD
    lst = st.lst_usd / E.USD
    usdc_r = st.usdc_res_usd / E.USD
    usdc_l = st.usdc_lend_usd / E.USD
    return cbtc * pbtc_r + (sol + lst) * psol_r + usdc_r + usdc_l


def simular(esc, params, rng, throttle_thresholds=None, shocks=None):
    """
    esc: dict de escenario de marketing (vol_tge, vol_pico, dias_pico, app_dia, hitos usuarios...).
    params: overrides de parámetros (fee_a, k_min, asset_layer_bps, yield_apy, launch_day...).
    throttle_thresholds: (cons_ratio, def_ratio) para SIM de Throttle; None = valores del contrato.
    shocks: dict opcional para estrés {vol_mult(day)->float, yield_mult, vault_sub(day)->usd, ...}.
    Devuelve dict con series y métricas de la trayectoria.
    """
    n = EC.ECO["dias"]
    tge = EC.ECO["tge_precio"]
    si = EC.ECO["supply_total"]
    objetivo = EC.ECO["supply_objetivo"]

    # --- parámetros ajustables ---
    p = {
        "fee_a1": 0.040, "fee_a2": 0.025, "pct_wl": 0.30,
        "k_min_usd": 25_000_000, "yield_apy": EC.ECO["yield_apy"],
        "app_dia": esc.get("app_dia", 90),
        "daily_burn_cap": 0.01,
        # multiplicador de fee de Motor A para SENSIBILIDAD (1.0 = fiel al contrato).
        # el fee es lineal en el notional → escalar el notional pasado a process_fee
        # equivale exactamente a escalar la tasa del fee, sin tocar el engine.
        "fee_a_mult": 1.0,
    }
    p.update(params or {})

    # --- estado del contrato ---
    st = E.ProtocolState(genesis_ts=0, last_queue_exec_ts=0)
    st.k_min_usd = int(p["k_min_usd"] * E.USD)
    # semilla del Vault (Fase 1): $200K repartidos por composición del contrato
    vi = EC.ECO["vault_ini"]
    st.cbtc_usd = int(vi * 0.35 * E.USD)
    st.sol_usd = int(vi * 0.15 * E.USD)
    st.lst_usd = int(vi * 0.20 * E.USD)
    st.usdc_res_usd = int(vi * 0.25 * E.USD)
    st.usdc_lend_usd = int(vi * 0.05 * E.USD)
    st.vault_core_usd = int(vi * E.USD)

    # throttle override (para SIM 4)
    if throttle_thresholds is not None:
        cons_r, def_r = throttle_thresholds
        E.THROTTLE_CONS_BPS_OVR = int(cons_r * 10000)
        E.THROTTLE_DEF_BPS_OVR = int(def_r * 10000)
    else:
        cons_r, def_r = 0.80, 0.50

    def throttle_mode(price, ema30):
        if ema30 <= 0:
            return E.THROTTLE_NORMAL
        r = price / ema30
        if r > 1.20:
            return E.THROTTLE_ACCELERATED
        elif r >= cons_r:
            return E.THROTTLE_NORMAL
        elif r >= def_r:
            return E.THROTTLE_CONSERVATIVE
        return E.THROTTLE_DEFENSIVE

    # --- sendas de mercado ---
    reg = EC.markov(n, rng, trans_mult=(shocks or {}).get("trans_mult"))
    pbtc_r = EC.precios_btc(n, reg, rng)
    psol_r = EC.precios_sol(n, reg, pbtc_r, rng)

    supply = EC.ECO["circulante_ini"]
    p_luka = tge
    ema30 = tge
    p_pico = tge
    prev_burned_usd = st.burned_total  # micro-USD

    # series
    K_s = np.zeros(n); pk_s = np.zeros(n); pl_s = np.zeros(n)
    sup_s = np.zeros(n); soc_s = np.zeros(n); rop_s = np.zeros(n)
    thr_s = np.zeros(n, dtype=int); cola_s = np.zeros(n); usr_s = np.zeros(n)
    burn_s = np.zeros(n); vol_s = np.zeros(n); stg_s = np.zeros(n, dtype=int)
    feeA_s = np.zeros(n); feeD_s = np.zeros(n)

    dia_b2 = None            # switch por K valorizado (diseño)
    dia_b2_contrato = None   # switch por vault_core_usd del contrato (sin apreciar)
    dia_etapa3 = None
    dia_enz = None
    esp_consec = 0
    espiral = False; ruina = False
    esp_dia = None; rui_dia = None

    hitos = esc["hitos"]

    for t in range(n):
        now_ts = t * 86400

        # 1. Vault valorizado y P_KASH (previo a fees del día)
        K = _K_valorizado(st, pbtc_r[t], psol_r[t])
        supply_eff = max(supply, objetivo)
        p_kash = K / supply_eff

        # 2. Etapa del contrato
        if t < p["app_dia"]:
            st.stage = 1
        elif st.stage < 2:
            st.stage = 2
        if K >= 50_000_000 and st.stage < 3:
            st.stage = 3
            if dia_etapa3 is None:
                dia_etapa3 = t
        if supply <= objetivo + 1 and st.stage < 4:
            st.stage = 4
            if dia_enz is None:
                dia_enz = t

        # 3. Throttle (usa precio/ema del día previo) → escribe modo en el estado del contrato
        st.throttle_mode = throttle_mode(p_luka, ema30)

        # 4. Yield del Vault (LST + lending), reinversión 70% / R_op 30%
        ymult = 1.0
        if shocks and "yield_mult" in shocks:
            ymult = shocks["yield_mult"](t)
        y_lst = st.lst_usd * p["yield_apy"] / 365 * ymult
        y_len = st.usdc_lend_usd * 0.06 / 365 * ymult
        st.lst_usd += int(y_lst * 0.70)
        st.usdc_lend_usd += int(y_len * 0.70)
        st.r_op_usd += int((y_lst + y_len) * 0.30)

        usr = EC.curva_usuarios(t, p["app_dia"], hitos)

        # ---- shock de volumen ----
        vmult = 1.0
        if shocks and "vol_mult" in shocks:
            vmult = shocks["vol_mult"](t)

        vol_total = 0.0

        def run(vol_usd, motor, layer=0, currency=1, is_wl=False):
            if vol_usd <= 0:
                return 0
            amount = int(round(vol_usd * E.USD))
            if amount <= 0:
                return 0
            try:
                return E.process_fee(st, amount, motor, layer, currency, is_wl, now_ts)
            except ValueError:
                return None

        # ── Motor A (SOL) — siempre. Split WL/no-WL ──
        vol_a = EC.vol_motor_a(t, reg[t], p_luka, esc, rng) * vmult
        vol_total += vol_a
        # fee_a_mult escala el notional SOLO para el cómputo del fee (sensibilidad de la tasa).
        vfa = vol_a * p["fee_a_mult"]
        rA_wl = run(vfa * p["pct_wl"], 0, currency=1, is_wl=True)
        rA = run(vfa * (1 - p["pct_wl"]), 0, currency=1, is_wl=False)
        feeA = ((rA_wl.fee if rA_wl else 0) + (rA.fee if rA else 0)) / E.USD
        feeA_s[t] = feeA

        # ── Motor B ($LUKA) + Motor D (multi) — Etapa 2A+ ──
        feeD = 0.0
        if st.stage >= 2 and usr > 0:
            vol_b = usr * 4.5 * 0.25 * vmult   # $4.5/usr/día, 25% DAU
            run(vol_b * p["pct_wl"], 1, currency=0, is_wl=True)
            run(vol_b * (1 - p["pct_wl"]), 1, currency=0, is_wl=False)

            vol_d = usr * 1.80 * 0.18 * vmult  # Manadas, capa 2 dominante
            # mezcla de capas D: 15% capa1, 70% capa2, 15% capa3A
            rd1 = run(vol_d * 0.15, 3, layer=1, currency=0)
            rd2a = run(vol_d * 0.70 * 0.7, 3, layer=2, currency=0)
            rd2b = run(vol_d * 0.70 * 0.3, 3, layer=2, currency=1)
            rd3 = run(vol_d * 0.15, 3, layer=3, currency=0)
            for rr in (rd1, rd2a, rd2b, rd3):
                if rr:
                    feeD += rr.fee / E.USD
        feeD_s[t] = feeD

        # ── Motor C (USDC) — Etapa 3+ ──
        if st.stage >= 3 and usr > 0:
            vol_c = usr * 0.50 * vmult
            run(vol_c, 2, currency=1)

        # 5. Exploit / sustracción directa del Vault (estrés)
        if shocks and "_vault_sub_frac" in shocks:
            fr = shocks["_vault_sub_frac"](t)   # fracción [0..1] a sustraer del Core
            if fr > 0:
                keep = max(0.0, 1.0 - fr)
                for f in ("cbtc_usd", "sol_usd", "lst_usd", "usdc_res_usd", "usdc_lend_usd"):
                    setattr(st, f, int(getattr(st, f) * keep))

        # 6. Quema USD→tokens con cap 1%/día
        burned_now_usd = (st.burned_total - prev_burned_usd) / E.USD
        prev_burned_usd = st.burned_total
        tokens_burned = 0.0
        if p_luka > 0 and supply > objetivo and burned_now_usd > 0:
            tok = burned_now_usd / p_luka
            cap = supply * p["daily_burn_cap"]
            tok = min(tok, cap, supply - objetivo)
            tokens_burned = max(0.0, tok)
            supply -= tokens_burned

        # 7. Switch B0→B2 (diseño: K valorizado ; contrato: vault_core_usd sin apreciar)
        if st.motor_b_state == E.MOTOR_B_B0 and K >= p["k_min_usd"]:
            st.motor_b_state = E.MOTOR_B_B2
            if dia_b2 is None:
                dia_b2 = t
        # brecha: cuándo el vault_core del contrato (sin apreciar) alcanzaría K_min (independiente del switch)
        if dia_b2_contrato is None and st.vault_core_usd >= st.k_min_usd:
            dia_b2_contrato = t

        # 8. Drenaje de cola diferida (semanal, si throttle normalizado)
        drained = E.execute_deferred_burn(st, now_ts)
        if drained > 0 and p_luka > 0 and supply > objetivo:
            dt = min((drained / E.USD) / p_luka, supply - objetivo)
            supply -= max(0.0, dt)
            tokens_burned += max(0.0, dt)

        # 9. Presión de vesting (devuelve tokens a circulación, 10% impacta)
        vt = EC.presion_vesting_tokens(t, reg[t])
        if p_luka < tge * 0.5:
            vt *= 1.0 + 0.15
        supply = min(supply + vt * 0.10, si)

        # 10. Recalcular K, P_KASH, precio, EMA30
        K = _K_valorizado(st, pbtc_r[t], psol_r[t])
        p_kash = K / max(supply, objetivo)
        p_luka = EC.precio_luka(p_kash, si, supply, reg[t], p_luka, tge)
        p_pico = max(p_pico, p_luka)
        ema30 = (2 / 31) * p_luka + (1 - 2 / 31) * ema30

        # 11. Espiral (trampa B0) y ruina
        if (t >= EC.ECO["espiral_dia_min"] and st.motor_b_state == E.MOTOR_B_B0 and
                K < p["k_min_usd"] * EC.ECO["espiral_vault_pct"] and
                p_luka < p_pico * EC.ECO["espiral_precio_pct"]):
            esp_consec += 1
            if esp_consec >= EC.ECO["espiral_dias_consec"] and not espiral:
                espiral = True; esp_dia = t
        else:
            esp_consec = max(0, esp_consec - 1)
        if t >= EC.ECO["ruina_dia_min"] and K < p["k_min_usd"] * EC.ECO["ruina_pct"] and not ruina:
            ruina = True; rui_dia = t

        # series
        K_s[t] = K; pk_s[t] = p_kash; pl_s[t] = p_luka
        sup_s[t] = supply; soc_s[t] = st.vault_sociedad_usd / E.USD
        rop_s[t] = st.r_op_usd / E.USD; thr_s[t] = st.throttle_mode
        cola_s[t] = st.deferred_burn_queue / E.USD; usr_s[t] = usr
        burn_s[t] = st.burned_total / E.USD; vol_s[t] = vol_total; stg_s[t] = st.stage

    return {
        "K_final": K_s[-1], "soc_final": soc_s[-1], "pk_final": pk_s[-1],
        "precio_final": pl_s[-1], "multiplo_x": pl_s[-1] / tge,
        "supply_final": sup_s[-1], "quemado_pct": (1 - sup_s[-1] / si) * 100,
        "dia_b2": dia_b2, "dia_b2_contrato": dia_b2_contrato,
        "dia_etapa3": dia_etapa3, "dia_enz": dia_enz,
        "espiral": espiral, "esp_dia": esp_dia, "ruina": ruina, "rui_dia": rui_dia,
        "usr_final": usr_s[-1], "cola_max": cola_s.max(), "cola_final": cola_s[-1],
        "dias_defensivo": int((thr_s == E.THROTTLE_DEFENSIVE).sum()),
        "st": st,  # estado final del contrato (para invariantes)
        # series
        "K_s": K_s, "pk_s": pk_s, "pl_s": pl_s, "sup_s": sup_s, "soc_s": soc_s,
        "rop_s": rop_s, "thr_s": thr_s, "cola_s": cola_s, "usr_s": usr_s,
        "burn_s": burn_s, "vol_s": vol_s, "stg_s": stg_s, "reg": reg,
        "feeA_s": feeA_s, "feeD_s": feeD_s,
    }
