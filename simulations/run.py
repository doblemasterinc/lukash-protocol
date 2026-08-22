"""
LUKASH — Runner de escenarios + chequeo de invariantes + hallazgos.

Corre el motor (fiel al contrato) bajo varios escenarios de mercado/adopción y
responde: ¿el protocolo alcanza sus hitos? ¿se sostiene el precio? ¿la cola de
quema explota? ¿la distribución 35/35/15/15 cierra siempre exacta?

Uso:  python run.py            # corre todos los escenarios, escribe CSV + PNG + resumen
"""
import os
import json
import math
import pandas as pd
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

import engine as E
from market import ScenarioParams, Simulation

OUT = os.path.join(os.path.dirname(__file__), "out")
os.makedirs(OUT, exist_ok=True)


# ======================= definición de escenarios =======================
def scenarios():
    S = []

    # 1. BASE — neutral, adopción base, liquidez media
    S.append(ScenarioParams(name="01_base_neutral"))

    # 2. BULL sostenido — más volumen, adopción rápida
    S.append(ScenarioParams(
        name="02_bull",
        volA_base_usd=8_000_000, users_midpoint_day=360, users_cap=600_000,
        regimes=((0, 1825, 1.6, 0.0),),
    ))

    # 3. BEAR persistente — poco volumen, adopción lenta, más ventas
    S.append(ScenarioParams(
        name="03_bear",
        volA_base_usd=3_000_000, users_midpoint_day=720, users_cap=300_000,
        base_sell_frac=0.006, regimes=((0, 1825, 0.55, 0.0),),
    ))

    # 4. LIQUIDEZ SECA — pool inicial muy bajo (riesgo real del fair-launch)
    S.append(ScenarioParams(
        name="04_liquidez_baja",
        lp_depth_usd0=60_000, volA_base_usd=2_500_000,
    ))

    # 5. SHOCK DE PÁNICO — bull inicial y crash brutal en el año 2 (test del Throttle/Exit)
    S.append(ScenarioParams(
        name="05_shock_panico",
        reflex_sell_mult=6.0,
        regimes=((0, 365, 1.4, 0.0), (365, 500, 0.3, -0.00002), (500, 1825, 0.8, 0.0)),
    ))

    # 6. SIN CAP DE QUEMA — misma base, pero desactivando el cap 1%/día (hipótesis del diseño)
    S.append(ScenarioParams(name="06_sin_cap_quema", apply_daily_burn_cap=False))

    # 7. ADOPCIÓN AGRESIVA — usuarios masivos, prueba de escala de Motores B/C/D
    S.append(ScenarioParams(
        name="07_adopcion_agresiva",
        users_cap=1_200_000, users_midpoint_day=300, users_k=0.014,
        volA_base_usd=7_000_000,
    ))

    return S


# ======================= invariantes =======================
def check_invariants(df, sim):
    """Chequeos duros sobre el run completo. Devuelve lista de (nombre, ok, detalle)."""
    checks = []
    st = sim.st

    # I1: composición y distribución declaradas suman 100%
    checks.append(("Composición Vault = 100%", st.invariant_composition_ok(), ""))
    checks.append(("Distribución 35/35/15/15 = 100%", st.invariant_distribution_ok(), ""))

    # I2: conservación de fees — las 4 cubetas suman (aprox) el fee total recaudado.
    #     vault(core+soc) + (burned+recirc+cola) + om + staking == fee_total (sin yield).
    fee_total = df["fee_total_usd"].sum()
    vault_contrib = (st.vault_core_usd + st.vault_sociedad_usd) / E.USD
    # descontar el yield acumulado (no viene de fees): reconstruimos el aporte de fees al vault
    # como 35% del fee_total (por definición de la distribución).
    expected_vault = fee_total * 0.35
    lp_bucket = (st.burned_total + st.recirculated_total + st.deferred_burn_queue) / E.USD
    om_bucket = st.om_total / E.USD
    stk_bucket = st.staking_total / E.USD
    # el LP + om + staking deben sumar ~65% del fee_total
    non_vault = lp_bucket + om_bucket + stk_bucket
    expected_non_vault = fee_total * 0.65
    rel_err = abs(non_vault - expected_non_vault) / max(expected_non_vault, 1)
    checks.append(("Conservación LP+O&M+Staking ≈ 65% de fees",
                   rel_err < 0.005, f"err={rel_err*100:.3f}%"))

    # I3: O&M ≈ Staking (ambos 15%, simétricos)
    sym_err = abs(om_bucket - stk_bucket) / max(stk_bucket, 1)
    checks.append(("O&M ≈ Staking (simetría 15/15)", sym_err < 0.01, f"err={sym_err*100:.3f}%"))

    # I4: supply nunca sube y nunca baja de 3.3B
    supply = df["supply"].values
    monotonic = all(supply[i] >= supply[i + 1] - 1 for i in range(len(supply) - 1))
    floor_ok = supply.min() >= 3_300_000_000 - 1
    checks.append(("Supply monótona decreciente", monotonic, ""))
    checks.append(("Supply nunca < 3.3B (ENZ floor)", floor_ok, f"min={supply.min()/1e9:.3f}B"))

    # I5: P_KASH nunca decrece (el Vault Core solo crece → piso creciente)
    pk = df["p_kash"].values
    pk_monotonic = all(pk[i] <= pk[i + 1] + 1e-12 for i in range(len(pk) - 1))
    checks.append(("P_KASH monótono creciente (piso)", pk_monotonic, ""))

    # I6: precio de mercado >= P_KASH siempre (el muro se respeta)
    above_floor = (df["price"] >= df["p_kash"] - 1e-12).all()
    checks.append(("Precio ≥ P_KASH siempre (muro)", bool(above_floor), ""))

    return checks


# ======================= métricas de hito =======================
def milestones(df):
    def first_day(cond):
        sub = df[cond]
        return int(sub["day"].iloc[0]) if len(sub) else None

    m = {}
    m["K_min $25M (B2)"] = first_day(df["vault_core_usd"] >= 25_000_000)
    m["Jaguar Lock $30M"] = first_day(df["vault_core_usd"] >= 30_000_000)
    m["Etapa 3 $50M"] = first_day(df["vault_core_usd"] >= 50_000_000)
    m["Escala $100M"] = first_day(df["vault_core_usd"] >= 100_000_000)
    m["ENZ (supply 3.3B)"] = first_day(df["supply"] <= 3_300_000_001)
    m["Switch a B2"] = first_day(df["motor_b"] == 1)
    return m


def summarize(name, df, sim, checks):
    end = df.iloc[-1]
    ms = milestones(df)
    invariants_ok = all(ok for _, ok, _ in checks)
    days_defensive = int((df["throttle"] == E.THROTTLE_DEFENSIVE).sum())
    days_cap_hit = int(df["burn_cap_hit"].sum()) if "burn_cap_hit" in df else 0
    max_queue = df["deferred_queue_usd"].max()
    end_queue = end["deferred_queue_usd"]
    return {
        "escenario": name,
        "vault_core_final_M": round(end["vault_core_usd"] / 1e6, 2),
        "vault_soc_final_M": round(end["vault_sociedad_usd"] / 1e6, 2),
        "p_kash_final": round(end["p_kash"], 8),
        "price_final": round(end["price"], 8),
        "price/pkash_final": round(end["price_over_pkash"], 2) if not math.isnan(end["price_over_pkash"]) else None,
        "supply_final_B": round(end["supply"] / 1e9, 3),
        "quemado_pct": round((1 - end["supply"] / 10e9) * 100, 1),
        "dia_K_min": ms["K_min $25M (B2)"],
        "dia_switch_B2": ms["Switch a B2"],
        "dia_jaguar_lock": ms["Jaguar Lock $30M"],
        "dia_etapa3": ms["Etapa 3 $50M"],
        "dia_ENZ": ms["ENZ (supply 3.3B)"],
        "dias_defensivo": days_defensive,
        "dias_cap_quema": days_cap_hit,
        "cola_max_usd": round(max_queue, 0),
        "cola_final_usd": round(end_queue, 0),
        "invariantes_ok": invariants_ok,
    }


# ======================= plots =======================
def plot_scenario(name, df):
    fig, axes = plt.subplots(2, 2, figsize=(13, 8))
    fig.suptitle(f"LUKASH — {name}", fontsize=13, weight="bold")

    ax = axes[0, 0]
    ax.plot(df["day"], df["vault_core_usd"] / 1e6, label="Vault Core", color="#1470CC")
    ax.plot(df["day"], df["vault_sociedad_usd"] / 1e6, label="Vault Sociedad", color="#CC7818")
    for y, lb in [(25, "K_min $25M"), (30, "Jaguar $30M"), (50, "Etapa3 $50M"), (100, "Escala $100M")]:
        ax.axhline(y, ls="--", lw=0.6, color="gray")
    ax.set_title("Vault (USD M)"); ax.set_xlabel("día"); ax.legend(fontsize=7)

    ax = axes[0, 1]
    ax.plot(df["day"], df["price"], label="Precio mercado", color="#20C486")
    ax.plot(df["day"], df["p_kash"], label="P_KASH (piso)", color="#D02818", lw=1)
    ax.plot(df["day"], df["ema30"], label="EMA30", color="#9E48D8", lw=0.7, ls=":")
    ax.set_title("Precio vs P_KASH (USD)"); ax.set_xlabel("día"); ax.set_yscale("log"); ax.legend(fontsize=7)

    ax = axes[1, 0]
    ax.plot(df["day"], df["supply"] / 1e9, color="#E44035")
    ax.axhline(3.3, ls="--", lw=0.6, color="gray")
    ax.set_title("Supply circulante (B tokens)"); ax.set_xlabel("día")

    ax = axes[1, 1]
    ax.plot(df["day"], df["deferred_queue_usd"], color="#DA8E1C", label="Cola diferida USD")
    ax2 = ax.twinx()
    ax2.plot(df["day"], df["throttle"], color="#12B4D4", lw=0.5, alpha=0.5, label="Throttle mode")
    ax2.set_ylabel("Throttle (0=ACEL..3=DEF)", fontsize=7)
    ax.set_title("Cola de quema diferida + Throttle"); ax.set_xlabel("día"); ax.legend(fontsize=7)

    fig.tight_layout(rect=[0, 0, 1, 0.96])
    path = os.path.join(OUT, f"{name}.png")
    fig.savefig(path, dpi=110)
    plt.close(fig)
    return path


# ======================= main =======================
def main():
    rows = []
    all_checks = {}
    for p in scenarios():
        sim = Simulation(p)
        sim.run()
        df = pd.DataFrame(sim.history)
        df.to_csv(os.path.join(OUT, f"{p.name}.csv"), index=False)
        checks = check_invariants(df, sim)
        all_checks[p.name] = checks
        plot_scenario(p.name, df)
        rows.append(summarize(p.name, df, sim, checks))

    summary = pd.DataFrame(rows)
    summary.to_csv(os.path.join(OUT, "_resumen.csv"), index=False)

    # salida legible
    print("=" * 100)
    print("RESUMEN DE ESCENARIOS")
    print("=" * 100)
    with pd.option_context("display.max_columns", None, "display.width", 200):
        print(summary.to_string(index=False))

    print("\n" + "=" * 100)
    print("INVARIANTES POR ESCENARIO")
    print("=" * 100)
    for name, checks in all_checks.items():
        allok = all(ok for _, ok, _ in checks)
        print(f"\n[{name}]  {'✅ TODOS OK' if allok else '❌ FALLA'}")
        for cname, ok, detail in checks:
            mark = "  ✅" if ok else "  ❌"
            print(f"{mark} {cname} {('· ' + detail) if detail else ''}")

    # persistir invariantes a JSON
    inv_json = {name: [{"check": c, "ok": ok, "detail": d} for c, ok, d in checks]
                for name, checks in all_checks.items()}
    with open(os.path.join(OUT, "_invariantes.json"), "w", encoding="utf-8") as f:
        json.dump(inv_json, f, ensure_ascii=False, indent=2)

    print(f"\nSalidas en: {OUT}")


if __name__ == "__main__":
    main()
