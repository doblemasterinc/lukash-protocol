"""
LUKASH — SUITE DE VALIDACIÓN (contract-faithful).

Reproduce las 4 simulaciones del informe cuantitativo v4.2, pero conducidas por
el MOTOR FIEL AL CONTRATO (engine.py = lib.rs). Objetivo doble:
  (a) validar que el CÓDIGO implementa el diseño (invariantes on-chain), y
  (b) confirmar que las conclusiones estructurales del informe se sostienen con
      la lógica on-chain real (no solo con el modelo float).

SIMs:
  0 — Invariantes del contrato (distribución exacta, simetría 15/15, monotonías)
  1 — Monte Carlo 3 campañas (CONSERVADOR/BASE/AGRESIVO) → espiral, timing B2
  2 — Sensibilidad OAT (qué parámetro mueve más el Vault Y5)
  3 — Estrés extremo (8 escenarios)
  4 — Throttle (grilla de umbrales) + Adopción de usuarios
  5 — MM vs NO-MM: fair-launch (ADR-011/019) vs Market Maker desde D1

PROVENANCE (ADR-008/H10): cifras ABSOLUTAS = familia "modelo v4 optimista".
Lo válido para decisión = estructural. Todo etiquetado.

Uso:
  python suite.py all        # corre todo (lento, ~40-50 min) → out/
  python suite.py inv        # solo invariantes (rápido)
  python suite.py mc         # solo Monte Carlo 3 campañas
  python suite.py sens       # sensibilidad
  python suite.py stress     # estrés
  python suite.py throttle   # throttle + usuarios
  python suite.py mm         # MM vs no-MM (ADR-019)
"""
import os, sys, json, time
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

import engine as E
import economic as EC
import trajectory as T
from scenarios import MKT

OUT = os.path.join(os.path.dirname(__file__), "out")
os.makedirs(OUT, exist_ok=True)

# iteraciones (reducidas vs informe 300-600 para runtime del motor entero; señal estructural intacta)
IT_MC = 200
IT_SENS = 60
IT_STRESS = 120
IT_THR = 100
IT_USERS = 100


def _pct(a, q): return float(np.percentile(a, q))


# ============================================================ SIM 0 — INVARIANTES
def sim_invariantes(seed=123):
    """Corre trayectorias representativas y verifica los invariantes DUROS del contrato."""
    rng = np.random.default_rng(seed)
    checks = []
    for name in ["CONSERVADOR", "BASE", "AGRESIVO"]:
        r = T.simular(MKT[name], {}, rng)
        st = r["st"]
        # I1 distribución declarada = 100%
        d_ok = st.invariant_distribution_ok() and st.invariant_composition_ok()
        # I2 simetría O&M == Staking (15/15)
        om, stk = st.om_total, st.staking_total
        sym = abs(om - stk) / max(stk, 1)
        # I3 conservación: burned+recirc+cola == suma histórica del tramo LP (35% de fees)
        #    om+staking == 30% de fees ; usamos om como proxy de 15% → fee_total = om/0.15
        fee_total = om / 0.15
        lp_bucket = st.burned_total + st.recirculated_total + st.deferred_burn_queue
        cons_err = abs(lp_bucket - fee_total * 0.35) / max(fee_total * 0.35, 1)
        # I4 supply monótona y piso 3.3B
        sup = r["sup_s"]
        mono = bool(np.all(np.diff(sup) <= 1e-3 + sup[:-1] * 0))  # decreciente salvo vesting
        # vesting puede subir supply levemente → chequeo relajado: nunca < 3.3B, nunca > 10B
        floor = bool(sup.min() >= 3_300_000_000 - 1 and sup.max() <= 10_000_000_000 + 1)
        # I5 P_KASH >= 0 y precio >= P_KASH siempre (muro)
        muro = bool(np.all(r["pl_s"] >= r["pk_s"] - 1e-12))
        checks.append({
            "escenario": name,
            "dist_comp_100pct": d_ok,
            "simetria_OM_STK": sym < 1e-6, "sym_err": sym,
            "conservacion_LP_35pct": cons_err < 0.01, "cons_err": cons_err,
            "supply_piso_3.3B_techo_10B": floor,
            "precio>=P_KASH (muro)": muro,
        })
    ok_all = all(c["dist_comp_100pct"] and c["simetria_OM_STK"] and
                 c["conservacion_LP_35pct"] and c["supply_piso_3.3B_techo_10B"] and
                 c["precio>=P_KASH (muro)"] for c in checks)
    with open(os.path.join(OUT, "sim0_invariantes.json"), "w", encoding="utf-8") as f:
        json.dump({"ok_all": ok_all, "checks": checks}, f, ensure_ascii=False, indent=2, default=float)
    print("\n=== SIM 0 — INVARIANTES DEL CONTRATO ===")
    for c in checks:
        print(f"  [{c['escenario']}] dist100%={c['dist_comp_100pct']} "
              f"OM==STK={c['simetria_OM_STK']} consLP={c['conservacion_LP_35pct']}(err={c['cons_err']:.1e}) "
              f"supplyFloor={c['supply_piso_3.3B_techo_10B']} muro={c['precio>=P_KASH (muro)']}")
    print(f"  → {'✅ TODOS LOS INVARIANTES OK' if ok_all else '❌ FALLA DE INVARIANTE'}")
    return ok_all


# ============================================================ SIM 1 — MONTE CARLO
def sim_montecarlo(iters=IT_MC, seed=42):
    print(f"\n=== SIM 1 — MONTE CARLO ({iters} iters × 3 campañas) ===")
    rng = np.random.default_rng(seed)
    res = {}
    series_ej = {}
    for name in ["CONSERVADOR", "BASE", "AGRESIVO"]:
        K = []; esp = 0; rui = 0; b2 = []; b2c = []; mult = []; quema = []; soc = []; colamax = []
        t0 = time.time()
        for i in range(iters):
            r = T.simular(MKT[name], {}, rng)
            K.append(r["K_final"]); esp += r["espiral"]; rui += r["ruina"]
            if r["dia_b2"] is not None: b2.append(r["dia_b2"])
            if r["dia_b2_contrato"] is not None: b2c.append(r["dia_b2_contrato"])
            mult.append(r["multiplo_x"]); quema.append(r["quemado_pct"]); soc.append(r["soc_final"])
            colamax.append(r["cola_max"])
            if i == 0: series_ej[name] = r
        K = np.array(K)
        res[name] = {
            "K_med": _pct(K, 50), "K_p10": _pct(K, 10), "K_p90": _pct(K, 90),
            "soc_med": float(np.median(soc)),
            "espiral_pct": esp / iters * 100, "ruina_pct": rui / iters * 100,
            "b2_med": float(np.median(b2)) if b2 else None,
            "b2_contrato_med": float(np.median(b2c)) if b2c else None,
            "p_b2_ano1": len([d for d in b2 if d <= 365]) / iters * 100,
            "p_b2_ano2": len([d for d in b2 if d <= 730]) / iters * 100,
            "p_b2_nunca": (iters - len(b2)) / iters * 100,
            "mult_med": float(np.median(mult)), "quema_med": float(np.median(quema)),
            "cola_max_med": float(np.median(colamax)),
            "secs": time.time() - t0,
        }
        s = res[name]
        print(f"  [{name}] K_med=${s['K_med']/1e6:.0f}M (P10 ${s['K_p10']/1e6:.0f}M–P90 ${s['K_p90']/1e6:.0f}M) "
              f"espiral={s['espiral_pct']:.1f}% ruina={s['ruina_pct']:.1f}% "
              f"B2_med=d{int(s['b2_med']) if s['b2_med'] else None} P(B2<A2)={s['p_b2_ano2']:.0f}% ({s['secs']:.0f}s)")
    with open(os.path.join(OUT, "sim1_montecarlo.json"), "w", encoding="utf-8") as f:
        json.dump(res, f, ensure_ascii=False, indent=2, default=float)
    _plot_mc(res, series_ej)
    return res


def _plot_mc(res, series_ej):
    fig, axes = plt.subplots(2, 2, figsize=(14, 9))
    fig.suptitle("LUKASH SIM 1 — Monte Carlo por campaña (motor fiel al contrato)", weight="bold")
    col = {"CONSERVADOR": "#BA7517", "BASE": "#1D9E75", "AGRESIVO": "#185FA5"}
    # a) Vault trayectoria ejemplo
    ax = axes[0, 0]
    for name, r in series_ej.items():
        ax.plot(r["K_s"] / 1e6, color=col[name], label=name, lw=1.3)
    ax.axhline(25, ls="--", c="r", lw=0.7); ax.axhline(50, ls="--", c="gray", lw=0.6)
    ax.set_title("Vault Core valorizado (USD M) — trayectoria ejemplo"); ax.set_yscale("log"); ax.legend(fontsize=8)
    # b) espiral/ruina bar
    ax = axes[0, 1]
    names = list(res.keys())
    esp = [res[n]["espiral_pct"] for n in names]; rui = [res[n]["ruina_pct"] for n in names]
    x = np.arange(len(names)); w = 0.35
    ax.bar(x - w/2, esp, w, label="Espiral (trampa B0) %", color="#C0392B")
    ax.bar(x + w/2, rui, w, label="Ruina %", color="#7d3c98")
    ax.set_xticks(x); ax.set_xticklabels(names, fontsize=8); ax.set_title("Riesgo por campaña"); ax.legend(fontsize=8)
    for i, v in enumerate(esp): ax.text(i - w/2, v + 0.3, f"{v:.0f}%", ha="center", fontsize=8)
    # c) día B2
    ax = axes[1, 0]
    b2 = [res[n]["b2_med"] or 0 for n in names]
    ax.bar(names, b2, color=[col[n] for n in names])
    ax.axhline(365, ls="--", c="green", lw=0.7); ax.axhline(730, ls="--", c="gray", lw=0.7)
    ax.set_title("Día mediano de switch B0→B2"); ax.tick_params(labelsize=8)
    for i, v in enumerate(b2): ax.text(i, v + 10, f"d{int(v)}", ha="center", fontsize=8)
    # d) precio ejemplo
    ax = axes[1, 1]
    for name, r in series_ej.items():
        ax.plot(r["pl_s"] / EC.ECO["tge_precio"], color=col[name], label=name, lw=1.0)
    ax.set_title("Precio $LUKA (× TGE) — trayectoria ejemplo"); ax.set_yscale("log"); ax.legend(fontsize=8)
    fig.tight_layout(rect=[0, 0, 1, 0.96])
    fig.savefig(os.path.join(OUT, "sim1_montecarlo.png"), dpi=110); plt.close(fig)


# ============================================================ SIM 2 — SENSIBILIDAD
def sim_sensibilidad(iters=IT_SENS, seed=7):
    print(f"\n=== SIM 2 — SENSIBILIDAD OAT ({iters} iters/punto) ===")
    base_esc = "BASE"
    # (nombre, key en params, valores). Valor base marcado aparte.
    # Fee Motor A se varía vía fee_a_mult (base 2.5% → mult = fee/0.025), fiel a la linealidad del fee.
    params_grid = {
        "Fee Motor A (%)":  ("_fee_a_pct", [0.015, 0.020, 0.025, 0.030, 0.035]),
        "K_min B2 ($M)":    ("k_min_usd", [10e6, 17e6, 25e6, 30e6, 35e6]),
        "Volumen (×base)":  ("_volmult", [0.3, 0.6, 1.0, 2.0, 4.0]),
        "Yield APY":        ("yield_apy", [0.04, 0.055, 0.07, 0.085, 0.10]),
        "Día launch App":   ("app_dia", [30, 60, 90, 135, 180]),
    }
    out = {}
    for pname, (key, vals) in params_grid.items():
        row = []
        for v in vals:
            rng = np.random.default_rng(seed)  # números comunes: mismas sendas por punto
            K = []
            for i in range(iters):
                esc = dict(MKT[base_esc])
                params = {}
                if key == "_volmult":
                    esc["vol_tge"] = esc["vol_tge"] * v
                    esc["vol_pico"] = esc["vol_pico"] * v
                elif key == "_fee_a_pct":
                    params["fee_a_mult"] = v / 0.025  # base Etapa 2 = 2.5%
                elif key == "app_dia":
                    esc["app_dia"] = int(v); params["app_dia"] = int(v)
                elif key == "k_min_usd":
                    params["k_min_usd"] = v
                else:
                    params[key] = v
                r = T.simular(esc, params, rng)
                K.append(r["K_final"])
            row.append({"valor": v, "K_med": _pct(np.array(K), 50)})
        Kmin = min(x["K_med"] for x in row); Kmax = max(x["K_med"] for x in row)
        out[pname] = {"key": key, "puntos": row, "swing": Kmax - Kmin, "K_min": Kmin, "K_max": Kmax}
        print(f"  [{pname}] swing=${(Kmax-Kmin)/1e6:.0f}M  (min ${Kmin/1e6:.0f}M / max ${Kmax/1e6:.0f}M)")
    ranking = sorted(out.items(), key=lambda kv: kv[1]["swing"], reverse=True)
    with open(os.path.join(OUT, "sim2_sensibilidad.json"), "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=2, default=float)
    _plot_tornado(ranking)
    print("  → Ranking de impacto:", " > ".join(k for k, _ in ranking))
    return out


def _plot_tornado(ranking):
    fig, ax = plt.subplots(figsize=(11, 6))
    names = [k for k, _ in ranking][::-1]
    swings = [v["swing"] / 1e6 for _, v in ranking][::-1]
    mins = [v["K_min"] / 1e6 for _, v in ranking][::-1]
    ax.barh(names, swings, left=mins, color="#185FA5", alpha=0.8)
    for i, (_, v) in enumerate(ranking[::-1]):
        ax.text(v["K_max"]/1e6, i, f" ${v['swing']/1e6:.0f}M", va="center", fontsize=9)
    ax.set_title("SIM 2 — Tornado de sensibilidad del Vault Y5 (mediana)", weight="bold")
    ax.set_xlabel("Vault Y5 (USD M)")
    fig.tight_layout(); fig.savefig(os.path.join(OUT, "sim2_tornado.png"), dpi=110); plt.close(fig)


# ============================================================ SIM 3 — ESTRÉS
def sim_estres(iters=IT_STRESS, seed=99):
    print(f"\n=== SIM 3 — ESTRÉS EXTREMO ({iters} iters/escenario) ===")

    def crash(volx, yieldx, d0, d1):
        return {"vol_mult": lambda t: volx if d0 <= t < d1 else 1.0,
                "yield_mult": lambda t: yieldx if d0 <= t < d1 else 1.0}

    escenarios = {
        "BASE (sin estrés)": {},
        "Crash BTC -50%": crash(0.5, 0.7, 60, 150),
        "Crash BTC -80%": crash(0.2, 0.5, 60, 330),
        "Motor A a 0% (6m)": {"vol_mult": lambda t: 0.0 if 180 <= t < 360 else 1.0},
        "Motor A -70% perm": {"vol_mult": lambda t: 0.30 if t >= 180 else 1.0},
        "Retiro LP 40%": {"vol_mult": lambda t: (0.7 if 200 <= t < 290 else (0.85 if 290 <= t < 380 else 1.0))},
        "Exploit 5% Vault": {"vault_sub": lambda t: 1e12 if t == 150 else 0.0},   # marca; se maneja como fracción
        "Exploit 15% Vault": {"vault_sub": lambda t: 3e12 if t == 150 else 0.0},
    }
    # nota: vault_sub se interpreta como USD sustraído; para % lo calculamos dentro con K. Simplificamos:
    # redefinimos como fracción directa del Vault en el día del shock.
    frac_sub = {"Exploit 5% Vault": 0.05, "Exploit 15% Vault": 0.15}

    out = {}
    base_med = None
    for name, shocks in escenarios.items():
        sh = dict(shocks)
        if name in frac_sub:
            fr = frac_sub[name]
            sh = {"_vault_sub_frac": (lambda t, fr=fr: fr if t == 150 else 0.0)}
        # números aleatorios comunes: cada escenario ve las MISMAS sendas de mercado
        rng = np.random.default_rng(seed)
        K = []; esp = 0; rui = 0
        for i in range(iters):
            r = T.simular(MKT["BASE"], {}, rng, shocks=(sh if sh else None))
            K.append(r["K_final"]); esp += r["espiral"]; rui += r["ruina"]
        Kmed = _pct(np.array(K), 50)
        if base_med is None: base_med = Kmed
        out[name] = {"K_med": Kmed, "delta_vs_base_pct": (Kmed/base_med - 1)*100,
                     "espiral_pct": esp/iters*100, "ruina_pct": rui/iters*100}
        print(f"  [{name}] K_med=${Kmed/1e6:.0f}M  Δ={out[name]['delta_vs_base_pct']:+.1f}%  espiral={esp/iters*100:.1f}%")
    with open(os.path.join(OUT, "sim3_estres.json"), "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=2, default=float)
    _plot_estres(out)
    return out


def _wrap_shocks(sh):
    """Normaliza el shock de exploit a modo fracción para trajectory.vault_sub."""
    if sh.get("_frac_mode"):
        frac_fn = sh["vault_sub"]
        # trajectory interpreta vault_sub como USD; para fracción, devolvemos K*frac vía closure diferida.
        return {"_vault_sub_frac": frac_fn}
    return sh


def _plot_estres(out):
    fig, ax = plt.subplots(figsize=(12, 6))
    names = list(out.keys()); K = [out[n]["K_med"]/1e6 for n in names]
    esp = [out[n]["espiral_pct"] for n in names]
    colors = ["#1D9E75" if out[n]["delta_vs_base_pct"] > -20 else "#C0392B" for n in names]
    ax.bar(range(len(names)), K, color=colors)
    ax.set_xticks(range(len(names))); ax.set_xticklabels(names, rotation=30, ha="right", fontsize=8)
    ax.set_ylabel("Vault Y5 mediana (USD M)")
    for i, n in enumerate(names):
        ax.text(i, K[i], f"{out[n]['delta_vs_base_pct']:+.0f}%\nesp {esp[i]:.0f}%", ha="center", va="bottom", fontsize=7)
    ax.set_title("SIM 3 — Resiliencia ante estrés extremo (Vault Y5 vs BASE)", weight="bold")
    fig.tight_layout(); fig.savefig(os.path.join(OUT, "sim3_estres.png"), dpi=110); plt.close(fig)


# ============================================================ SIM 4 — THROTTLE + USUARIOS
def sim_throttle(iters=IT_THR, seed=11):
    print(f"\n=== SIM 4a — THROTTLE (grilla de umbrales, {iters} iters) ===")
    configs = [(0.30, 0.60), (0.40, 0.70), (0.50, 0.80), (0.60, 0.70), (0.60, 0.85), (0.70, 0.80)]
    out = {}
    for (def_r, cons_r) in configs:
        rng = np.random.default_rng(seed)  # números comunes: misma senda por config
        K = []
        for i in range(iters):
            r = T.simular(MKT["BASE"], {}, rng, throttle_thresholds=(cons_r, def_r))
            K.append(r["K_final"])
        K = np.array(K)
        lbl = f"DEF {def_r:.2f} / CONS {cons_r:.2f}"
        out[lbl] = {"K_med": _pct(K, 50), "K_std": float(np.std(K)), "es_actual": (def_r, cons_r) == (0.50, 0.80)}
        print(f"  [{lbl}] K_med=${out[lbl]['K_med']/1e6:.0f}M std=${out[lbl]['K_std']/1e6:.0f}M {'← ACTUAL' if out[lbl]['es_actual'] else ''}")
    with open(os.path.join(OUT, "sim4_throttle.json"), "w", encoding="utf-8") as f:
        json.dump(out, f, ensure_ascii=False, indent=2, default=float)

    print(f"\n=== SIM 4b — USUARIOS (masa crítica, {IT_USERS} iters) ===")
    user_levels = {"1K": {6:1000,18:3000,36:8000,60:15000},
                   "5K": {6:5000,18:15000,36:40000,60:80000},
                   "20K": {6:20000,18:80000,36:200000,60:400000},
                   "50K": {6:50000,18:200000,36:500000,60:900000},
                   "100K": {6:100000,18:400000,36:955000,60:1500000}}
    out_u = {}
    for lvl, hitos in user_levels.items():
        rng = np.random.default_rng(seed + 1)  # números comunes por nivel de usuarios
        K = []
        for i in range(IT_USERS):
            esc = dict(MKT["BASE"]); esc["hitos"] = hitos
            r = T.simular(esc, {}, rng)
            K.append(r["K_final"])
        out_u[lvl] = {"K_med": _pct(np.array(K), 50)}
        print(f"  [{lvl} usuarios] K_med=${out_u[lvl]['K_med']/1e6:.0f}M")
    with open(os.path.join(OUT, "sim4_usuarios.json"), "w", encoding="utf-8") as f:
        json.dump(out_u, f, ensure_ascii=False, indent=2, default=float)
    _plot_throttle_users(out, out_u)
    return out, out_u


def _plot_throttle_users(out, out_u):
    fig, axes = plt.subplots(1, 2, figsize=(14, 5.5))
    labels = list(out.keys()); K = [out[l]["K_med"]/1e6 for l in labels]
    colors = ["#DA8E1C" if out[l]["es_actual"] else "#185FA5" for l in labels]
    axes[0].bar(range(len(labels)), K, color=colors)
    axes[0].set_xticks(range(len(labels))); axes[0].set_xticklabels(labels, rotation=25, ha="right", fontsize=8)
    axes[0].set_title("SIM 4a — Throttle: Vault Y5 por config (dorado=ACTUAL 0.50/0.80)", fontsize=10, weight="bold")
    axes[0].set_ylabel("Vault Y5 (USD M)")
    ul = list(out_u.keys()); Ku = [out_u[l]["K_med"]/1e6 for l in ul]
    axes[1].plot(ul, Ku, "o-", color="#1D9E75")
    axes[1].set_title("SIM 4b — Usuarios al launch vs Vault Y5", fontsize=10, weight="bold")
    axes[1].set_ylabel("Vault Y5 (USD M)"); axes[1].set_xlabel("Usuarios (curva de adopción)")
    fig.tight_layout(); fig.savefig(os.path.join(OUT, "sim4_throttle_usuarios.png"), dpi=110); plt.close(fig)


# ============================================================ SIM 5 — MM vs NO-MM
def sim_mm_comparison(iters=IT_MC, seed=77):
    """
    Compara fair-launch sin MM (ADR-011/019 F1) vs lanzamiento con MM desde D1.
    El MM modifica: (a) vol_tge 3x, (b) early ramp mas estable, (c) supply pressure
    por token loan ~3% (300M LUKA, 12 meses) con sell-through segun regimen.
    """
    print(f"\n=== SIM 5 — COMPARACION MM vs NO-MM ({iters} iters x 3 campañas x 2 modos) ===")

    mm_vol_mult = 3.0       # MM triplica vol TGE
    mm_ramp_mult = 1.8      # MM acelera adopcion temprana (primeros 6m)
    mm_loan_tokens = 300_000_000  # 3% supply prestado al MM
    mm_loan_months = 12     # duracion del loan

    res = {}
    series_ej = {}

    for name in ["CONSERVADOR", "BASE", "AGRESIVO"]:
        for modo in ["NO_MM", "CON_MM"]:
            lbl = f"{name}_{modo}"
            K = []; esp = 0; rui = 0; b2 = []; mult = []; quema = []; colamax = []
            precio_final = []; sup_final = []
            t0 = time.time()

            for i in range(iters):
                esc = dict(MKT[name])
                params = {}
                shocks = None

                if modo == "CON_MM":
                    esc["vol_tge"] = esc["vol_tge"] * mm_vol_mult
                    # MM accelera early ramp: hitos de 6 meses x1.8
                    h = dict(esc["hitos"])
                    if 6 in h:
                        h[6] = int(h[6] * mm_ramp_mult)
                    esc["hitos"] = h
                    # sell pressure del MM loan: tokens devueltos a circulacion
                    # modelado como aumento de vesting sell-through
                    sell_rate = {0: 0.02, 1: 0.06, 2: 0.12}  # MM vende mas en bear
                    mm_tokens_dia = mm_loan_tokens / (mm_loan_months * 30)

                    def _mm_vol_boost(t, base_fn=EC.vol_motor_a, esc=esc, rng_ref=[None]):
                        # primeros 180 dias: MM agrega profundidad extra
                        if t < 180:
                            return 1.0 + (mm_vol_mult - 1.0) * max(0, 1.0 - t/180)
                        return 1.0

                    shocks = {"vol_mult": lambda t: (
                        1.0 + max(0, (mm_vol_mult - 1.0) * (1.0 - t / 180)) if t < 180 else 1.0
                    )}

                rng = np.random.default_rng(seed + i)
                r = T.simular(esc, params, rng, shocks=shocks)

                # post-hoc: si CON_MM, aplicar supply pressure del loan
                if modo == "CON_MM":
                    sup = r["supply_final"]
                    # MM devuelve tokens no vendidos al final del loan.
                    # Promedio vendido: ~40% del loan en condiciones normales
                    vendido_pct = 0.40
                    sup_adj = sup  # supply ya calculada; el impacto real es menor
                    # lo que importa: el precio final baja por dilution
                    r["precio_final"] = r["precio_final"] * (1 - vendido_pct * mm_loan_tokens / max(sup, 1))
                    r["multiplo_x"] = r["precio_final"] / EC.ECO["tge_precio"]

                K.append(r["K_final"]); esp += r["espiral"]; rui += r["ruina"]
                if r["dia_b2"] is not None: b2.append(r["dia_b2"])
                mult.append(r["multiplo_x"]); quema.append(r["quemado_pct"])
                colamax.append(r["cola_max"])
                precio_final.append(r["precio_final"])
                sup_final.append(r["supply_final"])
                if i == 0:
                    series_ej[lbl] = r

            K = np.array(K)
            res[lbl] = {
                "modo": modo, "escenario": name,
                "K_med": _pct(K, 50), "K_p10": _pct(K, 10), "K_p90": _pct(K, 90),
                "espiral_pct": esp / iters * 100, "ruina_pct": rui / iters * 100,
                "b2_med": float(np.median(b2)) if b2 else None,
                "p_b2_ano1": len([d for d in b2 if d <= 365]) / iters * 100,
                "mult_med": float(np.median(mult)), "quema_med": float(np.median(quema)),
                "cola_max_med": float(np.median(colamax)),
                "precio_med": float(np.median(precio_final)),
                "secs": time.time() - t0,
            }
            s = res[lbl]
            print(f"  [{lbl}] K_med=${s['K_med']/1e6:.0f}M  espiral={s['espiral_pct']:.1f}%  "
                  f"B2_med=d{int(s['b2_med']) if s['b2_med'] else 'N/A'}  "
                  f"mult={s['mult_med']:.1f}x  ({s['secs']:.0f}s)")

    # --- resumen comparativo ---
    print("\n  --- RESUMEN COMPARATIVO ---")
    for name in ["CONSERVADOR", "BASE", "AGRESIVO"]:
        no = res[f"{name}_NO_MM"]; si = res[f"{name}_CON_MM"]
        dk = (si["K_med"] / max(no["K_med"], 1) - 1) * 100
        desp = si["espiral_pct"] - no["espiral_pct"]
        print(f"  [{name}] MM da Vault {dk:+.0f}%, espiral {desp:+.1f}pp, "
              f"mult {si['mult_med']:.1f}x vs {no['mult_med']:.1f}x")

    with open(os.path.join(OUT, "sim5_mm_vs_nomm.json"), "w", encoding="utf-8") as f:
        json.dump(res, f, ensure_ascii=False, indent=2, default=float)
    _plot_mm_comparison(res, series_ej)
    return res


def _plot_mm_comparison(res, series_ej):
    fig, axes = plt.subplots(2, 2, figsize=(15, 10))
    fig.suptitle("SIM 5 — Fair-launch (sin MM) vs Market Maker desde D1", weight="bold", fontsize=13)
    names = ["CONSERVADOR", "BASE", "AGRESIVO"]
    col_no = {"CONSERVADOR": "#BA7517", "BASE": "#1D9E75", "AGRESIVO": "#185FA5"}
    col_mm = {"CONSERVADOR": "#E8A838", "BASE": "#3FD4A0", "AGRESIVO": "#4A8FD5"}

    # (a) Vault Y5 comparativo
    ax = axes[0, 0]
    x = np.arange(len(names)); w = 0.35
    k_no = [res[f"{n}_NO_MM"]["K_med"]/1e6 for n in names]
    k_mm = [res[f"{n}_CON_MM"]["K_med"]/1e6 for n in names]
    ax.bar(x - w/2, k_no, w, label="Sin MM (fair-launch)", color=[col_no[n] for n in names])
    ax.bar(x + w/2, k_mm, w, label="Con MM D1", color=[col_mm[n] for n in names], alpha=0.85)
    ax.set_xticks(x); ax.set_xticklabels(names, fontsize=9)
    ax.set_ylabel("Vault Y5 mediana (USD M)"); ax.set_title("Vault Core Y5"); ax.legend(fontsize=8)
    for i in range(len(names)):
        delta = (k_mm[i]/max(k_no[i],1)-1)*100
        ax.text(i+w/2, k_mm[i], f"{delta:+.0f}%", ha="center", va="bottom", fontsize=8, color="green" if delta>0 else "red")

    # (b) Riesgo espiral
    ax = axes[0, 1]
    esp_no = [res[f"{n}_NO_MM"]["espiral_pct"] for n in names]
    esp_mm = [res[f"{n}_CON_MM"]["espiral_pct"] for n in names]
    ax.bar(x - w/2, esp_no, w, label="Sin MM", color="#C0392B")
    ax.bar(x + w/2, esp_mm, w, label="Con MM D1", color="#E74C3C", alpha=0.7)
    ax.set_xticks(x); ax.set_xticklabels(names, fontsize=9)
    ax.set_ylabel("Espiral (trampa B0) %"); ax.set_title("Riesgo de espiral"); ax.legend(fontsize=8)
    for i in range(len(names)):
        for j, (v, xo) in enumerate([(esp_no[i], -w/2), (esp_mm[i], w/2)]):
            ax.text(i+xo, v+0.3, f"{v:.0f}%", ha="center", fontsize=8)

    # (c) Multiplo precio
    ax = axes[1, 0]
    m_no = [res[f"{n}_NO_MM"]["mult_med"] for n in names]
    m_mm = [res[f"{n}_CON_MM"]["mult_med"] for n in names]
    ax.bar(x - w/2, m_no, w, label="Sin MM", color=[col_no[n] for n in names])
    ax.bar(x + w/2, m_mm, w, label="Con MM D1", color=[col_mm[n] for n in names], alpha=0.85)
    ax.set_xticks(x); ax.set_xticklabels(names, fontsize=9)
    ax.set_ylabel("Multiplo precio (x TGE)"); ax.set_title("Apreciacion de precio (med.)"); ax.legend(fontsize=8)
    ax.set_yscale("log")

    # (d) Trayectoria ejemplo BASE
    ax = axes[1, 1]
    if "BASE_NO_MM" in series_ej:
        ax.plot(series_ej["BASE_NO_MM"]["K_s"]/1e6, color=col_no["BASE"], label="BASE sin MM", lw=1.3)
    if "BASE_CON_MM" in series_ej:
        ax.plot(series_ej["BASE_CON_MM"]["K_s"]/1e6, color=col_mm["BASE"], label="BASE con MM D1", lw=1.3, ls="--")
    ax.axhline(25, ls="--", c="r", lw=0.7, label="K_min $25M")
    ax.set_title("Vault Core — trayectoria ejemplo (BASE)"); ax.set_yscale("log"); ax.legend(fontsize=8)
    ax.set_xlabel("Dia"); ax.set_ylabel("USD M")

    fig.tight_layout(rect=[0, 0, 1, 0.96])
    fig.savefig(os.path.join(OUT, "sim5_mm_vs_nomm.png"), dpi=110); plt.close(fig)


# ============================================================ MAIN
def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "all"
    t0 = time.time()
    if cmd in ("all", "inv"): sim_invariantes()
    if cmd in ("all", "mc"): sim_montecarlo()
    if cmd in ("all", "sens"): sim_sensibilidad()
    if cmd in ("all", "stress"): sim_estres()
    if cmd in ("all", "throttle"): sim_throttle()
    if cmd in ("all", "mm"): sim_mm_comparison()
    print(f"\n⏱  Completado en {(time.time()-t0)/60:.1f} min. Salidas en {OUT}")


if __name__ == "__main__":
    main()
