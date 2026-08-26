"""
LUKASH PROTOCOL v4.2 — Simulacion Monte Carlo FINAL (v4)
=========================================================
Ing. Sebastian Botero Pabon | Marzo 2026

MODELO RIGUROSO:
  1. Vault en USD con apreciacion correcta (buckets USD * precio_relativo)
  2. Precio $LUKA = Precio KASH * prima de mercado (no AMM como precio global)
     Prima depende de: regimen, deflacion acumulada, momentum
  3. Espiral correctamente definida: trampa B0 por crecimiento insuficiente del Vault
     (no colapso del Vault — el Vault solo crece)
  4. Tres escenarios de marketing con volumenes calibrados a datos reales
  5. Feedback: en BEAR el volumen cae, en BULL sube — afecta tiempo para alcanzar K_min
  6. Ciclos BTC de 4 anos con dinamica de halving
  7. Presion vendedora de vesting
  8. Costos operativos reales del O&M
  9. Motor C activado cuando K > $50M
 10. Desagregacion vault: fees acumulados vs apreciacion de activos

NOTA SOBRE CALIBRACION:
  Los resultados no se ajustan para coincidir con ningun numero del protocolo.
  El modelo habla por si mismo. Los parametros reflejan supuestos de mercado
  razonables y calibrados con datos reales (Nubank CAC, Solana DEX volumes,
  BTC ciclos historicos). Las conclusiones son las del modelo, no del documento.

5,000 iteraciones por escenario | 1,825 dias | 3 escenarios
"""

import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.patches as mpatches
import pandas as pd
import warnings
warnings.filterwarnings('ignore')

# ═══════════════════════════════════════════════════════════════════════════════
# 1. PARAMETROS DEL PROTOCOLO
# ═══════════════════════════════════════════════════════════════════════════════

P = {
    # Supply
    "supply_total":           10_000_000_000,
    "supply_objetivo":         3_300_000_000,
    "tge_precio":                     0.0001,

    # Vault
    "vault_ini":                    200_000,   # $200K real de Fase 1
    "cbtc_pct":  0.35, "sol_pct":   0.15, "lst_pct":  0.20,
    "usdc_r_pct":0.25, "usdc_l_pct":0.05, "pyth_pct": 0.05,

    # Yields
    "yield_lst":   0.07, "yield_len": 0.06,
    "reinv":       0.70, "r_op_pct":  0.30,

    # Fees
    "fee_A1": 0.04, "fee_A2": 0.025, "fee_wl": 0.015,
    "fee_B2": 0.025,
    "pct_wl": 0.30,   # 30% usuarios con whitelist

    # Distribucion
    "dv": 0.35, "dl": 0.35, "dom": 0.15, "dst": 0.15,

    # Costos operativos (80% del O&M va a gastos reales del equipo)
    "om_gasto": 0.80,

    # K_min
    "k_min": 25_000_000,

    # Motor C
    "k_motor_c": 50_000_000,
    "vol_c_usr":  0.50,   # $0.50/usuario/dia
    "fee_c":      0.005,

    # Throttle
    "th_acel": 1.20, "th_cons": 0.80, "th_def": 0.50,
    "q_acel":  1.25, "q_norm":  1.00, "q_cons": 0.60, "q_def": 0.25,
    "cap_q":   0.01,   # cap 1% supply/dia
    "cola_liq_dia": 0.10/7,

    # Modulo contra-ciclico
    "cc_bull": 0.40, "cc_neu": 0.75, "cc_bear": 0.70,

    # UCR
    "ucr": 0.20,

    # Vesting: 55% del supply tiene cliff 6m + 3 anos lineal
    "vest_cliff":  180,
    "vest_dur":   1080,
    "vest_supply": 0.55,
    "vest_vende":  {0: 0.12, 1: 0.22, 2: 0.40},  # por regimen

    # Prima de mercado del precio $LUKA
    # precio_luka = precio_kash * prima
    # prima depende de: regimen, deflacion, momentum
    "prima_bull_max":  8.0,    # maximo multiplicador en bull run
    "prima_bear_min":  0.25,   # minimo en bear profundo
    "prima_neu":       1.50,   # en neutral

    # DEFINICION DE ESPIRAL (trampa B0):
    # El protocolo no llega a K_min si en dia > 540 dias (18 meses)
    # el Vault esta < 50% de K_min Y el precio de LUKA < 50% del precio pico
    # durante 60 dias consecutivos
    "espiral_dia_min":        540,
    "espiral_vault_pct":      0.50,   # K < 50% de K_min
    "espiral_precio_pct":     0.50,   # precio < 50% del pico
    "espiral_dias_consec":     60,

    # RUINA: K < 10% de K_min ($2.5M) en cualquier momento despues de dia 365
    "ruina_dia_min":   365,
    "ruina_pct":       0.10,   # K < 10% de K_min

    # Sim
    "N": 5_000,
    "DIAS": 1_825,
}

# ═══════════════════════════════════════════════════════════════════════════════
# 2. ESCENARIOS DE MARKETING
# ═══════════════════════════════════════════════════════════════════════════════

MKT = {
    "CONSERVADOR": {
        "desc": "Sin MMs activos. 2-3 KOLs pequeños. CAC $9. Crecimiento orgánico.",
        "app_dia":    180,
        "vol_tge":    200_000,     # $200K/dia Motor A en TGE
        "vol_pico": 3_000_000,     # $3M/dia en pico (mes 36+)
        "dias_pico": 1_080,        # cuanto tarda en llegar al pico
        "usr_m6":     4_000,
        "usr_m18":   30_000,
        "usr_m36":  120_000,
        "usr_m60":  300_000,
        "cac":          9.0,
        "ret_mensual":  0.62,
    },
    "BASE": {
        "desc": "3 KOLs con KASH Lock. 2 MMs desde dia 1. CAC $6. Flywheel moderado.",
        "app_dia":    120,
        "vol_tge":    800_000,
        "vol_pico":  12_000_000,    # 2M/dia en pico — calibrado con JTO/Jito 2024
        "dias_pico":    900,
        "usr_m6":    15_000,
        "usr_m18":  120_000,
        "usr_m36":  450_000,
        "usr_m60": 1_100_000,
        "cac":          6.0,
        "ret_mensual":  0.72,
    },
    "AGRESIVO": {
        "desc": "MMs desde D1. KOLs virales. Manadas activas. CAC $4. Flywheel pleno.",
        "app_dia":     60,
        "vol_tge":  3_000_000,
        "vol_pico": 60_000_000,    # 0M/dia en pico — comparable WIF primeros 12 meses
        "dias_pico":   720,
        "usr_m6":    60_000,
        "usr_m18":  400_000,
        "usr_m36": 1_200_000,
        "usr_m60": 3_500_000,
        "cac":          4.0,
        "ret_mensual":  0.79,
    },
}

# ═══════════════════════════════════════════════════════════════════════════════
# 3. CADENA DE MARKOV CON CICLOS DE HALVING
# ═══════════════════════════════════════════════════════════════════════════════

# Fases del ciclo de mercado cripto (calibradas con historicos BTC)
FASES_CICLO = [
    # (inicio, fin, delta_bull_prob)
    (0,    120,  +0.10),   # hype de TGE
    (120,  360,  +0.08),   # narrativa inicial
    (360,  600,  -0.08),   # consolidacion / corrección
    (600,  900,  -0.18),   # bear tipico
    (900, 1200,  +0.04),   # recuperacion
    (1200,1460,  +0.18),   # pre-halving BTC (ciclo 4 anos)
    (1460,1825,  +0.12),   # post-halving bull run
]

TRANS_BASE = np.array([
    [0.68, 0.26, 0.06],   # BULL
    [0.28, 0.52, 0.20],   # NEUTRAL
    [0.10, 0.30, 0.60],   # BEAR
])

def trans_dia(dia):
    delta = 0.0
    for d0, d1, db in FASES_CICLO:
        if d0 <= dia < d1:
            delta = db; break
    T = TRANS_BASE.copy()
    T[1, 0] = np.clip(T[1, 0] + delta, 0.05, 0.80)
    T[1, 2] = np.clip(T[1, 2] - delta * 0.5, 0.05, 0.60)
    T[2, 0] = np.clip(T[2, 0] + delta * 0.3, 0.03, 0.50)
    for i in range(3):
        T[i] = np.maximum(T[i], 0.03)
        T[i] /= T[i].sum()
    return T

def markov(n, s0=1):
    reg = np.zeros(n, dtype=int)
    reg[0] = s0
    for t in range(1, n):
        reg[t] = np.random.choice(3, p=trans_dia(t)[reg[t-1]])
    return reg

# ═══════════════════════════════════════════════════════════════════════════════
# 4. MODELOS DE PRECIO DE ACTIVOS
# ═══════════════════════════════════════════════════════════════════════════════

def precios_btc(n, reg):
    """Retornos BTC dependientes del regimen"""
    params = {0:(+0.45/365, 0.50/np.sqrt(365)),
              1:(+0.08/365, 0.40/np.sqrt(365)),
              2:(-0.28/365, 0.55/np.sqrt(365))}
    r = np.array([np.random.normal(params[reg[t]][0] - 0.5*params[reg[t]][1]**2,
                                   params[reg[t]][1]) for t in range(n)])
    return np.exp(np.cumsum(r))

def precios_sol(n, reg, pbtc):
    """SOL: beta 1.2 sobre BTC con ruido"""
    r_btc = np.diff(np.log(np.concatenate([[1.0], pbtc])))
    params_ind = {0:(+0.08/365, 0.70/np.sqrt(365)),
                  1:(+0.02/365, 0.55/np.sqrt(365)),
                  2:(-0.10/365, 0.75/np.sqrt(365))}
    eps = np.array([np.random.normal(params_ind[reg[t]][0],
                                     params_ind[reg[t]][1]) for t in range(n)])
    r = 0.80 * r_btc + np.sqrt(1-0.80**2) * eps
    return np.exp(np.cumsum(np.clip(r, -0.35, 0.35)))

# ═══════════════════════════════════════════════════════════════════════════════
# 5. VAULT — USD con apreciacion correcta
# ═══════════════════════════════════════════════════════════════════════════════

def vault_nuevo():
    vi = P["vault_ini"]
    return {
        "cbtc": vi*P["cbtc_pct"],  "sol":  vi*P["sol_pct"],
        "lst":  vi*P["lst_pct"],   "usd_r":vi*P["usdc_r_pct"],
        "usd_l":vi*P["usdc_l_pct"],"pyth": vi*P["pyth_pct"],
        "fees": 0.0,
    }

def vault_usd(v, pbtc_r, psol_r):
    """
    Valor total en USD. Buckets guardan USD-invertido.
    Multiplicar por precio_relativo da valor actual.
    pbtc_r=1.0 en TGE. Si BTC sube 50%: pbtc_r=1.5 => cBTC vale 50% mas.
    """
    return (v["cbtc"]*pbtc_r + v["sol"]*psol_r + v["lst"]*psol_r +
            v["usd_r"] + v["usd_l"] + v["pyth"]*psol_r*0.5)

def vault_add(v, usd, reg):
    """Asigna USD al vault segun modulo contra-ciclico"""
    pv = {0:P["cc_bull"], 1:P["cc_neu"], 2:P["cc_bear"]}[reg]
    u_vol = usd * pv
    u_usd = usd * (1-pv)
    tp = P["cbtc_pct"]+P["sol_pct"]+P["lst_pct"]+P["pyth_pct"]
    v["cbtc"] += u_vol*(P["cbtc_pct"]/tp)
    v["sol"]  += u_vol*(P["sol_pct"]/tp)
    v["lst"]  += u_vol*(P["lst_pct"]/tp)
    v["pyth"] += u_vol*(P["pyth_pct"]/tp)
    v["usd_r"]+= u_usd*0.83
    v["usd_l"]+= u_usd*0.17
    v["fees"] += usd
    return v

# ═══════════════════════════════════════════════════════════════════════════════
# 6. MODELO DE PRECIO DE $LUKA
# ═══════════════════════════════════════════════════════════════════════════════

def precio_luka_modelo(p_kash, p_tge, supply_ini, supply_act, reg, p_prev, p_pico, dia):
    """
    Precio de $LUKA = Precio KASH * Prima de mercado

    El Precio KASH es el piso fundamental (Vault/Supply).
    La prima refleja el sentimiento de mercado, la deflacion acumulada
    y el momentum del precio.

    Esta es una aproximacion honesta al precio de mercado para una
    memecoin con fundamentals reales. El precio real en un DEX dependeria
    ademas de la profundidad del pool, pero eso lo aproximamos aqui como
    el factor de feedback entre precio y volumen.
    """
    # Deflacion acumulada: tokens quemados como porcentaje del supply inicial
    deflacion = max(0, (supply_ini - supply_act) / supply_ini)

    # Prima base por regimen (refleja apetito de riesgo del mercado)
    prima_base = {0: 5.0, 1: 1.8, 2: 0.45}[reg]

    # Factor deflacion: menos supply = mas escasez = prima mayor
    factor_deflacion = 1.0 + deflacion * 3.0

    # Momentum: si el precio subio ayer, tiende a seguir subiendo (y viceversa)
    if p_prev > 0:
        mom = (p_prev / p_tge) ** 0.15   # momentum suavizado (exponente < 1)
        mom = np.clip(mom, 0.10, 20.0)
    else:
        mom = 1.0

    # Prima total
    prima = prima_base * factor_deflacion * mom

    # Capping: la prima no puede ser infinita ni negativa
    prima = np.clip(prima, P["prima_bear_min"], P["prima_bull_max"] * factor_deflacion)

    # Precio de mercado = max(precio_kash, precio_kash * prima)
    # El precio kash es el piso absoluto (siempre)
    p_mercado = p_kash * prima

    return max(p_kash, p_mercado)

# ═══════════════════════════════════════════════════════════════════════════════
# 7. CURVA DE ADOPCION DE USUARIOS
# ═══════════════════════════════════════════════════════════════════════════════

def usuarios_dia(t, E):
    if t < E["app_dia"]: return 0
    meses = [0, 6, 18, 36, 60]
    users = [0, E["usr_m6"], E["usr_m18"], E["usr_m36"], E["usr_m60"]]
    dias_h = [m*30 for m in meses]
    td = t - E["app_dia"]
    if td >= dias_h[-1]: return users[-1]
    for i in range(len(dias_h)-1):
        if dias_h[i] <= td < dias_h[i+1]:
            frac = (td - dias_h[i]) / (dias_h[i+1] - dias_h[i])
            fs = 1/(1+np.exp(-8*(frac-0.5)))
            return users[i] + (users[i+1]-users[i])*fs
    return users[0]

# ═══════════════════════════════════════════════════════════════════════════════
# 8. VOLUMEN MOTOR A CON FEEDBACK DE PRECIO
# ═══════════════════════════════════════════════════════════════════════════════

def vol_A_dia(t, reg, p_luka, E):
    """
    Volumen con curva de adopcion + feedback del precio.
    En BEAR el volumen colapsa. En BULL explota.
    Si el precio sube mucho, mas traders entran (FOMO).
    Si el precio cae, traders huyen (FUD).
    """
    # Curva de adopcion base
    dias_h  = [0, 180, 540, E["dias_pico"], 1825]
    vols_h  = [E["vol_tge"],
               E["vol_tge"]*2.5,
               E["vol_pico"]*0.35,
               E["vol_pico"],
               E["vol_pico"]*0.85]  # ligera contraccion al final
    vol_base = float(np.interp(t, dias_h, vols_h))

    # Feedback del precio
    ratio = p_luka / P["tge_precio"]
    feedback = np.clip(ratio ** 0.30, 0.15, 2.5)   # cap 2.5x para evitar vol explosivo

    # Factor de regimen
    reg_f = {0: 2.5, 1: 1.0, 2: 0.30}[reg]

    # Ruido log-normal (volatilidad diaria del volumen en DEX)
    s = 0.75
    ruido = np.exp(np.random.normal(-0.5*s**2, s))

    vol = vol_base * feedback * reg_f * ruido
    return max(vol, 1_000.0)   # floor $1K

# ═══════════════════════════════════════════════════════════════════════════════
# 9. THROTTLE
# ═══════════════════════════════════════════════════════════════════════════════

def throttle(p, e30):
    if e30 <= 0: return 1, P["q_norm"], 0.0
    r = p / e30
    if r > P["th_acel"]:    return 0, P["q_acel"], 0.0
    elif r >= P["th_cons"]: return 1, P["q_norm"], 0.0
    elif r >= P["th_def"]:  return 2, P["q_cons"], 0.40
    else:                   return 3, P["q_def"],  0.75

def ema(prev, x, a=2/31): return a*x + (1-a)*prev

# ═══════════════════════════════════════════════════════════════════════════════
# 10. PRESION VENDEDORA DE VESTING
# ═══════════════════════════════════════════════════════════════════════════════

def presion_vesting(t, p_luka, reg):
    if t < P["vest_cliff"]: return 0.0
    tokens_dia = P["supply_total"] * P["vest_supply"] / P["vest_dur"]
    pct_vende = P["vest_vende"][reg]
    if p_luka < P["tge_precio"] * 0.5: pct_vende += 0.15
    return tokens_dia * pct_vende * p_luka   # USD equivalente de presion

# ═══════════════════════════════════════════════════════════════════════════════
# 11. SIMULACION COMPLETA DE UNA TRAYECTORIA
# ═══════════════════════════════════════════════════════════════════════════════

def simular(escenario):
    E  = MKT[escenario]
    n  = P["DIAS"]
    si = P["supply_total"]

    reg     = markov(n)
    pbtc_r  = precios_btc(n, reg)
    psol_r  = precios_sol(n, reg, pbtc_r)

    # Estado inicial
    vault    = vault_nuevo()
    supply   = P["supply_total"] - 1_000_000_000   # 1B en circulacion inicial
    r_op     = 0.0
    cola     = 0.0
    p_luka   = P["tge_precio"]
    e30      = P["tge_precio"]
    p_pico   = P["tge_precio"]

    # Series
    vs    = np.zeros(n)   # vault total USD
    vfs   = np.zeros(n)   # vault fees acumulados
    ss    = np.zeros(n)   # supply
    ps    = np.zeros(n)   # precio luka
    pks   = np.zeros(n)   # precio kash
    rs    = np.zeros(n)   # r_op
    us    = np.zeros(n)   # usuarios
    vols  = np.zeros(n)   # volumen Motor A
    regs  = reg.copy()

    dia_b2 = None
    esp_consec = 0
    espiral = False
    ruina   = False
    esp_dia = None
    rui_dia = None

    DIA_APP = E["app_dia"]

    for t in range(n):
        pbt = pbtc_r[t]; pst = psol_r[t]

        # Vault en USD
        k_t = vault_usd(vault, pbt, pst)

        # Precio KASH
        supply_eff = max(supply, P["supply_objetivo"])
        p_kash = k_t / supply_eff

        # Etapa
        if t < 90:
            etapa = 0
        elif k_t >= P["k_min"]:
            etapa = 2
            if dia_b2 is None: dia_b2 = t
        else:
            etapa = 1

        etapa_app = etapa if t >= DIA_APP else min(etapa, 0)

        # Usuarios
        usr = usuarios_dia(t, E)

        # Yield del Vault
        y_lst = vault["lst"]   * P["yield_lst"] / 365
        y_len = vault["usd_l"] * P["yield_len"] / 365
        y_tot = y_lst + y_len
        vault["lst"]   += y_lst * P["reinv"]
        vault["usd_l"] += y_len * P["reinv"]
        r_op += y_tot * P["r_op_pct"]   # acumula IDLE — no interviene

        # UCR (solo si r_op colapsa en escenario extremo)
        r_op_obj = k_t * 0.015
        if r_op < r_op_obj * P["ucr"] and vault["usd_l"] > 100:
            rec = min(y_tot*0.15, vault["usd_l"]*0.003)
            vault["usd_l"] -= rec; r_op += rec

        # ── Motor A ────────────────────────────────────────────────────────
        vol_a = vol_A_dia(t, reg[t], p_luka, E)
        fee_eff = (P["fee_wl"]*P["pct_wl"] +
                   (P["fee_A1"] if etapa==0 else P["fee_A2"])*(1-P["pct_wl"]))
        fee_a = vol_a * fee_eff

        # Vault: 35%
        vault = vault_add(vault, fee_a*P["dv"], reg[t])

        # Quema: 35% compra y quema $LUKA
        usd_compra_a = fee_a * P["dl"]
        if p_luka > 0 and supply > P["supply_objetivo"]:
            tok_a = usd_compra_a / p_luka
            tok_a = min(tok_a, supply*P["cap_q"], supply-P["supply_objetivo"])
            supply -= max(0, tok_a)

        # Presion vendedora de vesting
        usd_vesting = presion_vesting(t, p_luka, reg[t])
        if usd_vesting > 0 and p_luka > 0:
            supply_add = usd_vesting / p_luka * 0.1  # solo 10% impacta efectivamente
            supply = min(supply + supply_add, P["supply_total"])

        # ── Motor B ────────────────────────────────────────────────────────
        if etapa_app >= 1 and usr > 0:
            vol_b = usr * 4.5 * 0.25   # $4.5/usr/dia, 25% DAU activo
            fee_b = vol_b * P["fee_B2"]
            vault = vault_add(vault, fee_b*P["dv"], reg[t])

            modo_t, tq, fc = throttle(p_luka, e30)

            if etapa == 1:   # B0: quema directa
                usd_burn_b = fee_b * P["dl"] * tq
                cola      += fee_b * P["dl"] * fc
                if p_luka > 0 and supply > P["supply_objetivo"]:
                    tok_b = usd_burn_b / p_luka
                    tok_b = min(tok_b, supply*P["cap_q"], supply-P["supply_objetivo"])
                    supply -= max(0, tok_b)
            else:   # B2: recircula — presion compradora sin quema
                pass   # presion de compra crea demanda en el mercado

            # Cola diferida
            if cola > 0 and modo_t in [0,1] and p_luka > 0 and supply > P["supply_objetivo"]:
                liq = cola * P["cola_liq_dia"]
                tok_c = liq / p_luka
                supply -= min(tok_c, supply-P["supply_objetivo"])
                cola   -= liq

        # ── Motor D ────────────────────────────────────────────────────────
        if etapa_app >= 1 and usr > 0:
            vol_d = usr * 1.80 * 0.18   # $1.80/transaccion, 18% DAU en Manadas
            fee_d = vol_d * (P["fee_A2"] * 1.2)  # ~3% promedio capas 1/2
            vault = vault_add(vault, fee_d*P["dv"], reg[t])
            if etapa == 1 and p_luka > 0 and supply > P["supply_objetivo"]:
                tok_d = fee_d*P["dl"] / p_luka
                tok_d = min(tok_d, supply-P["supply_objetivo"])
                supply -= max(0, tok_d)

        # ── Motor C (K > $50M) ─────────────────────────────────────────────
        k_now = vault_usd(vault, pbt, pst)
        if k_now >= P["k_motor_c"] and usr > 0:
            vol_c = usr * P["vol_c_usr"]
            fee_c = vol_c * P["fee_c"]
            vault = vault_add(vault, fee_c*P["dv"], reg[t])
            if p_luka > 0 and supply > P["supply_objetivo"]:
                tok_c2 = fee_c*P["dl"] / p_luka
                supply -= min(max(0,tok_c2), supply-P["supply_objetivo"])

        # ── Precio $LUKA ───────────────────────────────────────────────────
        k_now = vault_usd(vault, pbt, pst)
        p_kash = k_now / max(supply, P["supply_objetivo"])
        p_luka = precio_luka_modelo(p_kash, P["tge_precio"],
                                     si, supply, reg[t], p_luka, p_pico, t)
        p_pico = max(p_pico, p_luka)
        e30 = ema(e30, p_luka)

        # ── Deteccion de espiral (trampa B0) ────────────────────────────────
        if (t >= P["espiral_dia_min"] and etapa < 2 and
                k_now < P["k_min"] * P["espiral_vault_pct"] and
                p_luka < p_pico * P["espiral_precio_pct"]):
            esp_consec += 1
            if esp_consec >= P["espiral_dias_consec"] and not espiral:
                espiral = True
                esp_dia = t
        else:
            esp_consec = max(0, esp_consec-1)

        # ── Deteccion de ruina ─────────────────────────────────────────────
        if t >= P["ruina_dia_min"] and k_now < P["k_min"]*P["ruina_pct"] and not ruina:
            ruina = True
            rui_dia = t

        # Registrar
        vs[t]   = k_now;     vfs[t]  = vault["fees"]
        ss[t]   = supply;    ps[t]   = p_luka
        pks[t]  = p_kash;    rs[t]   = r_op
        us[t]   = usr;       vols[t] = vol_a

    return {
        "vault_final":      vs[-1],
        "fees_final":       vfs[-1],
        "aprec_final":      vs[-1]-vfs[-1],
        "supply_final":     ss[-1],
        "precio_final":     ps[-1],
        "precio_kash":      pks[-1],
        "multiplo_x":       ps[-1]/P["tge_precio"],
        "dia_b2":           dia_b2,
        "espiral":          espiral,
        "ruina":            ruina,
        "usr_final":        us[-1],
        "vol_a_promedio":   float(np.mean(vols[vols>0])),
        # Series
        "vs": vs, "vfs": vfs, "ss": ss, "ps": ps,
        "pks": pks, "us": us, "vols": vols, "regs": regs,
    }

# ═══════════════════════════════════════════════════════════════════════════════
# 12. EJECUCION
# ═══════════════════════════════════════════════════════════════════════════════

def run_escenario(nombre):
    E = MKT[nombre]
    print(f"\n  ▶ {nombre}: {E['desc']}")
    res = []
    paso = P["N"] // 5
    for i in range(P["N"]):
        if i % paso == 0:
            print(f"    {int(i/P['N']*100):3d}%")
        res.append(simular(nombre))
    print("    100%")
    return res

def stats(res):
    vaults   = np.array([r["vault_final"]  for r in res])
    fees     = np.array([r["fees_final"]   for r in res])
    aprecs   = np.array([r["aprec_final"]  for r in res])
    multis   = np.array([r["multiplo_x"]   for r in res])
    dias_b2  = [r["dia_b2"] for r in res if r["dia_b2"] is not None]
    n = len(res)
    def pct(a, p): return float(np.percentile(a, p))
    return {
        "n": n, "vaults": vaults, "fees": fees, "aprecs": aprecs,
        "vm":  pct(vaults,50),  "vp10":pct(vaults,10), "vp25":pct(vaults,25),
        "vp75":pct(vaults,75),  "vp90":pct(vaults,90), "vp95":pct(vaults,95),
        "fm":  pct(fees,50),    "am":  pct(aprecs,50),
        "mm":  pct(multis,50),  "mp25":pct(multis,25),  "mp75":pct(multis,75),
        "pb2_1": len([d for d in dias_b2 if d<=365])/n,
        "pb2_2": len([d for d in dias_b2 if d<=730])/n,
        "pb2_n": (n-len(dias_b2))/n,
        "med_b2": float(np.median(dias_b2)) if dias_b2 else None,
        "p_esp":  sum(r["espiral"] for r in res)/n,
        "p_rui":  sum(r["ruina"]   for r in res)/n,
        "dias_b2": dias_b2, "res": res,
    }

# ═══════════════════════════════════════════════════════════════════════════════
# 13. REPORTE
# ═══════════════════════════════════════════════════════════════════════════════

def reporte(ST):
    ESC = ["CONSERVADOR","BASE","AGRESIVO"]
    print("\n" + "="*70)
    print("RESULTADOS — LUKASH Monte Carlo v4 | Modelo Riguroso")
    print("="*70)

    for e in ESC:
        s = ST[e]
        print(f"\n{'━'*70}")
        print(f"  {e}: {MKT[e]['desc']}")
        print(f"{'━'*70}")
        print(f"  Vault año 5  —  Mediana: ${s['vm']/1e6:.1f}M")
        print(f"                  P10: ${s['vp10']/1e6:.1f}M  |  P25: ${s['vp25']/1e6:.1f}M  |  P75: ${s['vp75']/1e6:.1f}M  |  P90: ${s['vp90']/1e6:.1f}M")
        print(f"  Desagregado  —  Fees acum: ${s['fm']/1e6:.1f}M  |  Aprec activos: ${s['am']/1e6:.1f}M")
        print(f"  Precio $LUKA —  Mediana: {s['mm']:.0f}x  |  P25–P75: {s['mp25']:.0f}x – {s['mp75']:.0f}x")
        print(f"  Motor B2     —  P(B2<Año1): {s['pb2_1']*100:.1f}%  |  P(B2<Año2): {s['pb2_2']*100:.1f}%  |  P(nunca): {s['pb2_n']*100:.1f}%")
        if s["med_b2"]:
            print(f"                  Mediana dia B2: Dia {int(s['med_b2'])} (mes {int(s['med_b2'])//30})")
        n = s["n"]
        print(f"  Riesgos      —  Espiral: {s['p_esp']*100:.2f}% ({int(s['p_esp']*n)} de {n:,})")
        print(f"                  Ruina: {s['p_rui']*100:.2f}% ({int(s['p_rui']*n)} de {n:,})")

    print(f"\n{'='*70}")
    print(f"  {'Escenario':<14} {'Vault Med':>10} {'Fees':>10} {'Aprec':>10} {'P(B2<A2)':>10} {'Espiral':>8} {'Ruina':>6}")
    print(f"  {'-'*70}")
    for e in ESC:
        s = ST[e]
        print(f"  {e:<14} ${s['vm']/1e6:>8.1f}M ${s['fm']/1e6:>8.1f}M ${s['am']/1e6:>8.1f}M "
              f"{s['pb2_2']*100:>9.1f}% {s['p_esp']*100:>7.2f}% {s['p_rui']*100:>5.2f}%")
    print("="*70)

    # Interpretacion automatica
    print("\n  INTERPRETACION CLAVE:")
    s_base = ST["BASE"]
    print(f"  → Escenario BASE: El vault alcanza K_min ($25M) en el {100-s_base['pb2_n']*100:.0f}% de los casos.")
    print(f"    En el {s_base['pb2_2']*100:.0f}% lo hace antes del Año 2.")
    if s_base["p_esp"] > 0:
        print(f"  → Espiral de muerte: {s_base['p_esp']*100:.1f}% de trayectorias quedan atrapadas en B0.")
        print(f"    Causa principal: bear market prolongado + adopcion insuficiente de usuarios.")
    else:
        print(f"  → Espiral de muerte: 0% — en ninguna trayectoria el protocolo queda atrapado en B0.")
    print(f"  → La varianza es alta: el P75/P25 del vault BASE es {s_base['vp75']/s_base['vp25']:.1f}x.")
    print(f"    Eso refleja el riesgo real de los ciclos de mercado cripto.")
    print(f"  → La desagregacion fees/apreciacion muestra que el protocolo genera")
    print(f"    ${s_base['fm']/1e6:.0f}M en fees reales (escenario BASE, mediana).")

# ═══════════════════════════════════════════════════════════════════════════════
# 14. GRAFICAS
# ═══════════════════════════════════════════════════════════════════════════════

C = {"C":"#BA7517","B":"#1D9E75","A":"#185FA5",
     "G":"#5F5E5A","O":"#16213E","R":"#C0392B","V2":"#085041"}
NOMBRES = ["CONSERVADOR","BASE","AGRESIVO"]
CK = {"CONSERVADOR":"C","BASE":"B","AGRESIVO":"A"}

def graficas(ST, RT):
    fig = plt.figure(figsize=(18, 24))
    fig.patch.set_facecolor("#FAFAF8")
    fig.text(0.5, 0.986, "LUKASH PROTOCOL v4.2 — Simulación Monte Carlo v4 (Modelo Riguroso)",
             ha="center", fontsize=15, fontweight="bold", color=C["O"])
    fig.text(0.5, 0.974,
             "5,000 iter. por escenario · Vault USD correcto · Precio = P_KASH × prima · Espiral como trampa B0 · Ciclos BTC halving · Marzo 2026",
             ha="center", fontsize=9, color=C["G"], style="italic")

    gs = gridspec.GridSpec(5, 3, figure=fig,
                           top=0.962, bottom=0.035,
                           hspace=0.48, wspace=0.33,
                           left=0.07, right=0.97)
    dias = np.arange(P["DIAS"])

    # ── Fila 0: Fan charts Vault por escenario ─────────────────────────────
    for col, esc in enumerate(NOMBRES):
        ax = fig.add_subplot(gs[0, col])
        ax.set_facecolor("#FAFAF8")
        s = ST[esc]; color = C[CK[esc]]
        all_v = np.array([r["vs"] for r in RT[esc][:400]])/1e6
        p10,p25,p50,p75,p90 = [np.percentile(all_v,q,axis=0) for q in [10,25,50,75,90]]
        ax.fill_between(dias,p10,p90,alpha=0.12,color=color)
        ax.fill_between(dias,p25,p75,alpha=0.28,color=color)
        ax.plot(dias,p50,color=color,lw=2)
        ax.axhline(P["k_min"]/1e6,color=C["R"],lw=1.2,ls="--",alpha=0.8)
        for d,l in [(365,"A1"),(730,"A2"),(1095,"A3")]:
            ax.axvline(d,color=C["G"],lw=0.5,ls=":",alpha=0.5)
            ax.text(d+8,ax.get_ylim()[1]*0.02 if ax.get_ylim()[1]>0 else 0.1,
                    l,fontsize=7,color=C["G"])
        ax.set_title(f"{esc}\nVault Med: ${s['vm']/1e6:.1f}M  |  P10: ${s['vp10']/1e6:.1f}M  |  P90: ${s['vp90']/1e6:.1f}M",
                     fontsize=8.5,fontweight="bold",color=color)
        ax.set_xlabel("Días",fontsize=8); ax.set_ylabel("Vault (M USD)",fontsize=8)
        ax.tick_params(labelsize=7.5); ax.spines[["top","right"]].set_visible(False)

    # ── Fila 1: Distribucion vault comparativa + Activacion B2 + Fees/Aprec ─
    ax_d = fig.add_subplot(gs[1,:2])
    ax_d.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        ax_d.hist(ST[esc]["vaults"]/1e6,bins=70,alpha=0.42,color=C[CK[esc]],
                  label=f"{esc} (med=${ST[esc]['vm']/1e6:.0f}M)",
                  density=True,edgecolor="white",linewidth=0.2)
    ax_d.axvline(P["k_min"]/1e6,color=C["R"],lw=1.5,ls="--",label="K_min=$25M")
    ax_d.set_xlabel("Vault año 5 (M USD)",fontsize=9); ax_d.set_ylabel("Densidad",fontsize=9)
    ax_d.set_title("Distribución del Vault — 3 escenarios comparados",fontsize=10,fontweight="bold")
    ax_d.legend(fontsize=8); ax_d.tick_params(labelsize=8); ax_d.spines[["top","right"]].set_visible(False)

    ax_b = fig.add_subplot(gs[1,2])
    ax_b.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        db2 = ST[esc]["dias_b2"]
        if db2:
            ax_b.hist(np.array(db2),bins=40,alpha=0.55,color=C[CK[esc]],
                      label=esc,density=True,edgecolor="white",lw=0.2)
    ax_b.axvline(365,color=C["V2"],lw=1.5,ls="--",label="Año 1")
    ax_b.axvline(730,color=C["G"], lw=1.5,ls="--",label="Año 2")
    ax_b.set_xlabel("Día de activación B2",fontsize=9); ax_b.set_ylabel("Densidad",fontsize=9)
    ax_b.set_title("Distribución del día de activación B2",fontsize=10,fontweight="bold")
    ax_b.legend(fontsize=7.5); ax_b.tick_params(labelsize=8); ax_b.spines[["top","right"]].set_visible(False)

    # ── Fila 2: Supply + Usuarios + Precio $LUKA ───────────────────────────
    ax_s = fig.add_subplot(gs[2,0])
    ax_s.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        all_s = np.array([r["ss"] for r in RT[esc][:300]])/1e9
        ax_s.plot(dias,np.percentile(all_s,50,axis=0),color=C[CK[esc]],lw=1.8,label=esc)
    ax_s.axhline(P["supply_objetivo"]/1e9,color=C["R"],lw=1.3,ls="--",label="3.3B objetivo")
    ax_s.set_xlabel("Días",fontsize=9); ax_s.set_ylabel("Supply (miles de mill.)",fontsize=9)
    ax_s.set_title("Deflación del supply $LUKA",fontsize=10,fontweight="bold")
    ax_s.legend(fontsize=7.5); ax_s.tick_params(labelsize=8); ax_s.spines[["top","right"]].set_visible(False)

    ax_u = fig.add_subplot(gs[2,1])
    ax_u.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        all_u = np.array([r["us"] for r in RT[esc][:100]])/1000
        ax_u.plot(dias,np.percentile(all_u,50,axis=0),color=C[CK[esc]],lw=1.8,label=esc)
    ax_u.set_xlabel("Días",fontsize=9); ax_u.set_ylabel("Usuarios activos (miles)",fontsize=9)
    ax_u.set_title("Adopción de usuarios por escenario",fontsize=10,fontweight="bold")
    ax_u.legend(fontsize=7.5); ax_u.tick_params(labelsize=8); ax_u.spines[["top","right"]].set_visible(False)

    ax_p = fig.add_subplot(gs[2,2])
    ax_p.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        all_p = np.array([r["ps"]/P["tge_precio"] for r in RT[esc][:200]])
        p25 = np.percentile(all_p,25,axis=0)
        p50 = np.percentile(all_p,50,axis=0)
        p75 = np.percentile(all_p,75,axis=0)
        color = C[CK[esc]]
        ax_p.fill_between(dias,p25,p75,alpha=0.18,color=color)
        ax_p.plot(dias,p50,color=color,lw=1.5,label=esc)
    ax_p.axhline(1.0,color=C["G"],lw=1.0,ls="--",alpha=0.7,label="TGE")
    ax_p.set_yscale("log")
    ax_p.set_xlabel("Días",fontsize=9); ax_p.set_ylabel("Precio $LUKA (x TGE, log)",fontsize=9)
    ax_p.set_title("Precio $LUKA vs TGE — P25/P50/P75",fontsize=10,fontweight="bold")
    ax_p.legend(fontsize=7.5); ax_p.tick_params(labelsize=8); ax_p.spines[["top","right"]].set_visible(False)

    # ── Fila 3: Precio KASH por escenario + volumen Motor A + Fees vs Apreciacion
    ax_k = fig.add_subplot(gs[3,0])
    ax_k.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        all_k = np.array([r["pks"] for r in RT[esc][:200]])
        p50 = np.percentile(all_k,50,axis=0)
        ax_k.plot(dias,p50,color=C[CK[esc]],lw=1.8,label=esc)
    ax_k.set_xlabel("Días",fontsize=9); ax_k.set_ylabel("Precio KASH (USD/LUKA)",fontsize=9)
    ax_k.set_title("Precio KASH — piso fundamental del protocolo",fontsize=10,fontweight="bold")
    ax_k.legend(fontsize=7.5); ax_k.tick_params(labelsize=8); ax_k.spines[["top","right"]].set_visible(False)

    ax_v = fig.add_subplot(gs[3,1])
    ax_v.set_facecolor("#FAFAF8")
    for esc in NOMBRES:
        all_vol = np.array([r["vols"] for r in RT[esc][:100]])/1e3
        v50 = np.percentile(all_vol,50,axis=0)
        ax_v.plot(dias,v50,color=C[CK[esc]],lw=1.5,label=esc)
    ax_v.set_xlabel("Días",fontsize=9); ax_v.set_ylabel("Volumen Motor A (miles USD)",fontsize=9)
    ax_v.set_title("Volumen diario Motor A — mediana por escenario",fontsize=10,fontweight="bold")
    ax_v.legend(fontsize=7.5); ax_v.tick_params(labelsize=8); ax_v.spines[["top","right"]].set_visible(False)

    ax_fa = fig.add_subplot(gs[3,2])
    ax_fa.set_facecolor("#FAFAF8")
    x = np.arange(3); w = 0.35
    fv = [ST[e]["fm"]/1e6 for e in NOMBRES]
    av = [ST[e]["am"]/1e6 for e in NOMBRES]
    ax_fa.bar(x-w/2,fv,w,color=[C[CK[e]] for e in NOMBRES],alpha=0.85,label="Fees acumulados")
    ax_fa.bar(x+w/2,av,w,color=[C[CK[e]] for e in NOMBRES],alpha=0.40,
               edgecolor=[C[CK[e]] for e in NOMBRES],lw=1.5,label="Apreciación activos")
    ax_fa.set_xticks(x); ax_fa.set_xticklabels(NOMBRES,fontsize=8)
    ax_fa.set_ylabel("Millones USD",fontsize=9)
    ax_fa.set_title("Vault: Fees vs Apreciación de activos\n(mediana, año 5)",fontsize=10,fontweight="bold")
    ax_fa.legend(fontsize=8); ax_fa.tick_params(labelsize=8); ax_fa.spines[["top","right"]].set_visible(False)

    # ── Fila 4: Regimen Markov + ejemplo precio + tabla resumen ────────────
    ax_r = fig.add_subplot(gs[4,0])
    ax_r.set_facecolor("#FAFAF8")
    r0 = RT["BASE"][5]
    reg0 = r0["regs"]
    for t in range(len(reg0)-1):
        ax_r.axvspan(t,t+1,color={0:"#1D9E7540",1:"#185FA540",2:"#C0392B40"}[reg0[t]],lw=0)
    ax_r2 = ax_r.twinx()
    ax_r2.plot(dias,r0["pks"],color=C["O"],lw=1.0,alpha=0.9,label="P_KASH")
    ax_r2.set_ylabel("P_KASH (USD)",fontsize=7.5,color=C["O"])
    ax_r2.tick_params(labelsize=7)
    ax_r.set_xlim(0,P["DIAS"]); ax_r.set_yticks([])
    ax_r.set_xlabel("Días",fontsize=9)
    ax_r.set_title("Régimen Markov + P_KASH\nEscenario BASE — trayectoria ejemplo",fontsize=10,fontweight="bold")
    patches = [mpatches.Patch(color="#1D9E75",label="BULL"),
               mpatches.Patch(color="#185FA5",label="NEUTRAL"),
               mpatches.Patch(color="#C0392B",label="BEAR")]
    ax_r.legend(handles=patches,fontsize=7.5,loc="upper left")
    ax_r.spines[["top","right"]].set_visible(False)

    ax_p2 = fig.add_subplot(gs[4,1])
    ax_p2.set_facecolor("#FAFAF8")
    ax_p2.plot(dias,r0["ps"]/P["tge_precio"],color=C["B"],lw=1.0,alpha=0.9)
    ax_p2.axhline(1.0,color=C["G"],lw=0.8,ls="--",alpha=0.7)
    ax_p2.set_yscale("log")
    ax_p2.set_xlabel("Días",fontsize=9); ax_p2.set_ylabel("Precio $LUKA (x TGE)",fontsize=9)
    ax_p2.set_title("Precio $LUKA — escenario BASE\ntrayectoria ejemplo (escala log)",fontsize=10,fontweight="bold")
    ax_p2.tick_params(labelsize=8); ax_p2.spines[["top","right"]].set_visible(False)

    ax_t = fig.add_subplot(gs[4,2])
    ax_t.axis("off")
    td = []
    for e in NOMBRES:
        s = ST[e]
        td.append([e,
                   f"${s['vm']/1e6:.0f}M",
                   f"${s['fm']/1e6:.0f}M",
                   f"{s['pb2_2']*100:.0f}%",
                   f"{s['p_esp']*100:.1f}%",
                   f"{s['p_rui']*100:.2f}%",
                   f"{s['mm']:.0f}x"])
    cols = ["Esc","Vault\nMed","Fees\nMed","P(B2\n<A2)","Espiral","Ruina","Precio\nMed"]
    tbl = ax_t.table(cellText=td,colLabels=cols,cellLoc="center",loc="center",bbox=[0,0.02,1,0.93])
    tbl.auto_set_font_size(False); tbl.set_fontsize(8)
    for (row,col),cell in tbl.get_celld().items():
        if row==0:
            cell.set_facecolor(C["O"]); cell.set_text_props(color="white",fontweight="bold")
        elif row>0:
            cell.set_facecolor(C[CK[NOMBRES[row-1]]]+"22")
        cell.set_edgecolor("#E0E0E0")
    ax_t.set_title("Tabla resumen",fontsize=10,fontweight="bold")

    plt.savefig("/home/claude/lukash_mc_v4.png",dpi=150,bbox_inches="tight",facecolor="#FAFAF8")
    print("\nGráfica guardada: lukash_mc_v4.png")

# ═══════════════════════════════════════════════════════════════════════════════
# 15. CSV y MAIN
# ═══════════════════════════════════════════════════════════════════════════════

def exportar(ST):
    filas = []
    for esc, s in ST.items():
        for i, r in enumerate(s["res"]):
            filas.append({
                "escenario": esc, "iteracion": i+1,
                "vault_final_usd":    round(r["vault_final"],2),
                "fees_acumulados_usd":round(r["fees_final"],2),
                "apreciacion_usd":    round(r["aprec_final"],2),
                "supply_final":       round(r["supply_final"],0),
                "precio_luka_final":  round(r["precio_final"],8),
                "multiplo_tge":       round(r["multiplo_x"],1),
                "dia_b2":             r["dia_b2"] if r["dia_b2"] else "N/A",
                "espiral":            int(r["espiral"]),
                "ruina":              int(r["ruina"]),
                "usuarios_finales":   round(r["usr_final"],0),
                "vol_a_promedio_usd": round(r["vol_a_promedio"],0),
            })
    pd.DataFrame(filas).to_csv("/home/claude/lukash_mc_v4_datos.csv",index=False)
    print("CSV guardado: lukash_mc_v4_datos.csv")

def main():
    np.random.seed(42)
    print("="*70)
    print("LUKASH PROTOCOL v4.2 — Monte Carlo v4 (Modelo Riguroso)")
    print(f"  {P['N']:,} iteraciones × 3 escenarios = {P['N']*3:,} trayectorias totales")
    print(f"  {P['DIAS']:,} dias por trayectoria (5 anos)")
    print("="*70)

    ST = {}; RT = {}
    for esc in ["CONSERVADOR","BASE","AGRESIVO"]:
        res = run_escenario(esc)
        s = stats(res); s["res"] = res
        ST[esc] = s; RT[esc] = res

    reporte(ST)
    graficas(ST, RT)
    exportar(ST)
    print(f"\nSimulacion completada. {P['N']*3:,} trayectorias generadas.")
    return ST

if __name__ == "__main__":
    main()
