"""
LUKASH — Capa económica (Markov + halving, precios de activos, prima de mercado,
vesting, adopción). Portada y adaptada del modelo riguroso `lukash_mc_v4.py`
(Ing. Sebastián Botero, Marzo 2026), para alimentar el MOTOR FIEL AL CONTRATO.

Diferencia clave con market.py (descartado): aquí el precio de $LUKA NO es un
modelo de impacto AMM (que colapsaba al piso), sino:

    precio_$LUKA = P_KASH × prima_de_mercado

donde P_KASH = Vault Core valorizado / supply, y la prima refleja régimen,
deflación acumulada y momentum. Es el mismo enfoque validado en el informe
cuantitativo v4.2. Los activos del Vault (cBTC/SOL/LST) se APRECIAN con el
mercado — el Vault crece por fees + apreciación + yield, no solo por fees.

NOTA DE PROVENANCE (ADR-008 / H10): los valores ABSOLUTOS de esta familia de
modelos son el "modelo v4 optimista" (10-200× el modelo conservador v3.1 de
$37.2M). Lo válido para decisiones es lo ESTRUCTURAL: % espiral, timing B2,
orden de escenarios, sensibilidades relativas. No publicar cifras absolutas
sin etiquetar la fuente.
"""
import numpy as np

# ======================= parámetros económicos =======================
ECO = {
    "tge_precio": 0.0001,
    "supply_total": 10_000_000_000,
    "supply_objetivo": 3_300_000_000,
    "circulante_ini": 9_000_000_000,   # 1B en vesting/reserva fuera de circulación al inicio

    "vault_ini": 100_000,              # $100K pool liquidez (ADR-018: 20% de $500K)
    "yield_apy": 0.07,                 # blended sobre LST+lending

    # Prima de mercado (precio = P_KASH × prima)
    "prima_bull": 5.0, "prima_neu": 1.8, "prima_bear": 0.45,
    "prima_min": 0.25, "prima_max": 8.0,
    "deflacion_boost": 3.0,            # 1 + deflacion*3 → escasez sube la prima

    # Vesting: 55% del supply, cliff 180d + 1080d lineal; % que vende por régimen
    "vest_supply": 0.55, "vest_cliff": 180, "vest_dur": 1080,
    "vest_vende": {0: 0.12, 1: 0.22, 2: 0.40},

    # Detección de espiral (trampa B0) y ruina
    "espiral_dia_min": 540, "espiral_vault_pct": 0.50,
    "espiral_precio_pct": 0.50, "espiral_dias_consec": 60,
    "ruina_dia_min": 365, "ruina_pct": 0.10,   # K < 10% de K_min tras día 365

    "dias": 1825,
}

# Fases del ciclo cripto (calibradas con históricos BTC de 4 años / halving)
FASES_CICLO = [
    (0, 120, +0.10), (120, 360, +0.08), (360, 600, -0.08), (600, 900, -0.18),
    (900, 1200, +0.04), (1200, 1460, +0.18), (1460, 1825, +0.12),
]
TRANS_BASE = np.array([
    [0.68, 0.26, 0.06],   # BULL → [bull,neu,bear]
    [0.28, 0.52, 0.20],   # NEUTRAL
    [0.10, 0.30, 0.60],   # BEAR
])


def trans_dia(dia):
    delta = 0.0
    for d0, d1, db in FASES_CICLO:
        if d0 <= dia < d1:
            delta = db
            break
    T = TRANS_BASE.copy()
    T[1, 0] = np.clip(T[1, 0] + delta, 0.05, 0.80)
    T[1, 2] = np.clip(T[1, 2] - delta * 0.5, 0.05, 0.60)
    T[2, 0] = np.clip(T[2, 0] + delta * 0.3, 0.03, 0.50)
    for i in range(3):
        T[i] = np.maximum(T[i], 0.03)
        T[i] /= T[i].sum()
    return T


def markov(n, rng, s0=1, trans_mult=None):
    """Genera la senda de regímenes (0=bull,1=neu,2=bear). trans_mult sesga el régimen (para estrés)."""
    reg = np.zeros(n, dtype=int)
    reg[0] = s0
    for t in range(1, n):
        p = trans_dia(t)[reg[t - 1]].copy()
        if trans_mult is not None:
            p = p * trans_mult
            p /= p.sum()
        reg[t] = rng.choice(3, p=p)
    return reg


def precios_btc(n, reg, rng):
    params = {0: (+0.45 / 365, 0.50 / np.sqrt(365)),
              1: (+0.08 / 365, 0.40 / np.sqrt(365)),
              2: (-0.28 / 365, 0.55 / np.sqrt(365))}
    r = np.array([rng.normal(params[reg[t]][0] - 0.5 * params[reg[t]][1] ** 2,
                             params[reg[t]][1]) for t in range(n)])
    return np.exp(np.cumsum(r))


def precios_sol(n, reg, pbtc, rng):
    r_btc = np.diff(np.log(np.concatenate([[1.0], pbtc])))
    params_ind = {0: (+0.08 / 365, 0.70 / np.sqrt(365)),
                  1: (+0.02 / 365, 0.55 / np.sqrt(365)),
                  2: (-0.10 / 365, 0.75 / np.sqrt(365))}
    eps = np.array([rng.normal(params_ind[reg[t]][0], params_ind[reg[t]][1]) for t in range(n)])
    r = 0.80 * r_btc + np.sqrt(1 - 0.80 ** 2) * eps
    return np.exp(np.cumsum(np.clip(r, -0.35, 0.35)))


def precio_luka(p_kash, supply_ini, supply_act, reg, p_prev, tge):
    """Precio $LUKA = P_KASH × prima. P_KASH es el piso absoluto."""
    deflacion = max(0, (supply_ini - supply_act) / supply_ini)
    prima_base = {0: ECO["prima_bull"], 1: ECO["prima_neu"], 2: ECO["prima_bear"]}[reg]
    factor_def = 1.0 + deflacion * ECO["deflacion_boost"]
    if p_prev > 0:
        mom = np.clip((p_prev / tge) ** 0.15, 0.10, 20.0)
    else:
        mom = 1.0
    prima = np.clip(prima_base * factor_def * mom, ECO["prima_min"], ECO["prima_max"] * factor_def)
    return max(p_kash, p_kash * prima)


def presion_vesting_tokens(t, reg):
    """Tokens/día que la presión de vesting devuelve a circulación (antes de impacto)."""
    if t < ECO["vest_cliff"]:
        return 0.0
    tokens_dia = ECO["supply_total"] * ECO["vest_supply"] / ECO["vest_dur"]
    return tokens_dia * ECO["vest_vende"][reg]


# ======================= adopción de usuarios =======================
def curva_usuarios(t, app_dia, hitos):
    """hitos = dict {mes: usuarios} p.ej. {6:15000,18:120000,36:450000,60:1100000}."""
    if t < app_dia:
        return 0.0
    meses = sorted(hitos.keys())
    users = [hitos[m] for m in meses]
    dias_h = [0] + [m * 30 for m in meses]
    users = [0] + users
    td = t - app_dia
    if td >= dias_h[-1]:
        return users[-1]
    for i in range(len(dias_h) - 1):
        if dias_h[i] <= td < dias_h[i + 1]:
            frac = (td - dias_h[i]) / (dias_h[i + 1] - dias_h[i])
            fs = 1 / (1 + np.exp(-8 * (frac - 0.5)))
            return users[i] + (users[i + 1] - users[i]) * fs
    return users[-1]


def vol_motor_a(t, reg, p_luka, esc, rng):
    """Volumen diario Motor A: curva de adopción × feedback de precio × régimen × ruido."""
    dias_h = [0, 180, 540, esc["dias_pico"], 1825]
    vols_h = [esc["vol_tge"], esc["vol_tge"] * 2.5, esc["vol_pico"] * 0.35,
              esc["vol_pico"], esc["vol_pico"] * 0.85]
    vol_base = float(np.interp(t, dias_h, vols_h))
    ratio = p_luka / ECO["tge_precio"]
    feedback = np.clip(ratio ** 0.30, 0.15, 2.5)
    reg_f = {0: 2.5, 1: 1.0, 2: 0.30}[reg]
    s = 0.75
    ruido = np.exp(rng.normal(-0.5 * s ** 2, s))
    return max(vol_base * feedback * reg_f * ruido, 1_000.0)
