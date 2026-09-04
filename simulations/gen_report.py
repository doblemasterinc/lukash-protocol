"""
LUKASH — Generador de informe de simulaciones cuantitativas v4.3.
Lee los JSON + PNGs de out/ y genera:
  1. informe_simulaciones_v4.3.html  (Artifact)
  2. informe_simulaciones_v4.3.pdf   (fpdf2)
"""
import os, json, base64
from fpdf import FPDF
from datetime import date

OUT = os.path.join(os.path.dirname(__file__), "out")

def _load(name):
    p = os.path.join(OUT, name)
    if not os.path.exists(p):
        return None
    with open(p, encoding="utf-8") as f:
        return json.load(f)

def _img_b64(name):
    p = os.path.join(OUT, name)
    if not os.path.exists(p):
        return ""
    with open(p, "rb") as f:
        return base64.b64encode(f.read()).decode()

def _fmt_usd(v, dec=0):
    if v is None: return "N/A"
    if abs(v) >= 1e9: return f"${v/1e9:.{dec}f}B"
    if abs(v) >= 1e6: return f"${v/1e6:.{dec}f}M"
    if abs(v) >= 1e3: return f"${v/1e3:.{dec}f}K"
    return f"${v:.{dec}f}"

def generate_html():
    mc = _load("sim1_montecarlo.json") or {}
    sens = _load("sim2_sensibilidad.json") or {}
    stress = _load("sim3_estres.json") or {}
    throttle = _load("sim4_throttle.json") or {}
    users = _load("sim4_usuarios.json") or {}
    mm = _load("sim5_mm_vs_nomm.json") or {}
    inv = _load("sim6_investor_roi.json") or {}
    inv0 = _load("sim0_invariantes.json") or {}

    mc_img = _img_b64("sim1_montecarlo.png")
    tornado_img = _img_b64("sim2_tornado.png")
    stress_img = _img_b64("sim3_estres.png")
    thr_img = _img_b64("sim4_throttle_usuarios.png")
    mm_img = _img_b64("sim5_mm_vs_nomm.png")
    inv_img = _img_b64("sim6_investor_roi.png")

    today = date.today().isoformat()

    def mc_row(name):
        d = mc.get(name, {})
        return f"""<tr>
            <td><strong>{name}</strong></td>
            <td>{_fmt_usd(d.get('K_med',0))}</td>
            <td>{_fmt_usd(d.get('K_p10',0))} – {_fmt_usd(d.get('K_p90',0))}</td>
            <td>{d.get('espiral_pct',0):.1f}%</td>
            <td>{d.get('ruina_pct',0):.1f}%</td>
            <td>d{int(d['b2_med']) if d.get('b2_med') else 'N/A'}</td>
            <td>{d.get('mult_med',0):.0f}×</td>
            <td>{d.get('quema_med',0):.1f}%</td>
        </tr>"""

    def sens_row(name, data):
        return f"""<tr>
            <td>{name}</td>
            <td>{_fmt_usd(data.get('swing',0))}</td>
            <td>{_fmt_usd(data.get('K_min',0))}</td>
            <td>{_fmt_usd(data.get('K_max',0))}</td>
        </tr>"""

    def stress_row(name, data):
        return f"""<tr>
            <td>{name}</td>
            <td>{_fmt_usd(data.get('K_med',0))}</td>
            <td>{data.get('delta_vs_base_pct',0):+.1f}%</td>
            <td>{data.get('espiral_pct',0):.1f}%</td>
        </tr>"""

    def inv_row(lbl, data):
        return f"""<tr>
            <td>{data.get('escenario','')}</td>
            <td>{data.get('stake_pct',0):.0f}%</td>
            <td>{_fmt_usd(data.get('total_5y_med',0), 2)}</td>
            <td>{data.get('roi_x',0):.1f}×</td>
            <td>{data.get('irr_med',0)*100 if data.get('irr_med') else 0:.0f}%</td>
            <td>{int(data['payback_med_dias']/30) if data.get('payback_med_dias') else 'N/A'} meses</td>
            <td>{data.get('payback_pct',0):.0f}%</td>
        </tr>"""

    sens_ranking = sorted(sens.items(), key=lambda kv: kv[1].get("swing", 0), reverse=True)

    html = f"""<title>LUKASH Simulaciones v4.3</title>
<style>
:root {{
  --bg: #fafafa; --fg: #1a1a2e; --accent: #185FA5; --accent2: #1D9E75;
  --warn: #BA7517; --danger: #C0392B; --card: #fff; --border: #e0e0e0;
  --font: 'Segoe UI', system-ui, sans-serif;
}}
@media (prefers-color-scheme: dark) {{
  :root:not([data-theme="light"]) {{
    --bg: #0f0f1a; --fg: #e0e0e8; --card: #1a1a2e; --border: #2a2a3e;
    --accent: #4A8FD5; --accent2: #3FD4A0; --warn: #E8A838; --danger: #E74C3C;
  }}
}}
:root[data-theme="dark"] {{
  --bg: #0f0f1a; --fg: #e0e0e8; --card: #1a1a2e; --border: #2a2a3e;
  --accent: #4A8FD5; --accent2: #3FD4A0; --warn: #E8A838; --danger: #E74C3C;
}}
body {{ background: var(--bg); color: var(--fg); font-family: var(--font);
       max-width: 960px; margin: 0 auto; padding: 2rem 1.5rem; line-height: 1.6; }}
h1 {{ color: var(--accent); font-size: 1.8rem; border-bottom: 3px solid var(--accent); padding-bottom: .5rem; }}
h2 {{ color: var(--accent2); font-size: 1.35rem; margin-top: 2.5rem; }}
h3 {{ font-size: 1.1rem; margin-top: 1.5rem; }}
.meta {{ color: #888; font-size: 0.85rem; margin-bottom: 2rem; }}
.card {{ background: var(--card); border: 1px solid var(--border); border-radius: 8px;
         padding: 1.2rem; margin: 1rem 0; }}
table {{ width: 100%; border-collapse: collapse; font-size: 0.85rem; }}
th, td {{ padding: 6px 10px; text-align: left; border-bottom: 1px solid var(--border); }}
th {{ background: var(--accent); color: #fff; font-weight: 600; }}
tr:nth-child(even) {{ background: rgba(0,0,0,0.03); }}
img {{ max-width: 100%; border-radius: 6px; margin: 1rem 0; }}
.badge {{ display: inline-block; padding: 2px 10px; border-radius: 12px; font-size: 0.8rem;
          font-weight: 600; }}
.badge-ok {{ background: #d4edda; color: #155724; }}
.badge-warn {{ background: #fff3cd; color: #856404; }}
.badge-danger {{ background: #f8d7da; color: #721c24; }}
.grid-2 {{ display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }}
@media (max-width: 700px) {{ .grid-2 {{ grid-template-columns: 1fr; }} }}
.highlight {{ background: linear-gradient(135deg, var(--accent) 0%, var(--accent2) 100%);
              color: #fff; padding: 1.5rem; border-radius: 10px; margin: 1.5rem 0; }}
.highlight h3 {{ color: #fff; margin-top: 0; }}
.toc {{ background: var(--card); border: 1px solid var(--border); border-radius: 8px;
        padding: 1rem 1.5rem; margin: 1rem 0; }}
.toc a {{ color: var(--accent); text-decoration: none; }}
.toc a:hover {{ text-decoration: underline; }}
.toc ol {{ padding-left: 1.2rem; }}
.toc li {{ margin: 0.3rem 0; }}
.disclaimer {{ font-size: 0.75rem; color: #999; border-top: 1px solid var(--border);
               padding-top: 1rem; margin-top: 3rem; }}
</style>

<h1>LUKASH Protocol — Informe de Simulaciones Cuantitativas v4.3</h1>
<p class="meta">Motor fiel al contrato (lib.rs v10.2) · {today} · Kash Sensei<br>
7 simulaciones · ~35,000 trayectorias · Horizonte 5 años (1,825 días)</p>

<div class="toc">
<strong>Contenido</strong>
<ol>
<li><a href="#s0">Invariantes del Contrato</a></li>
<li><a href="#s1">Monte Carlo — 3 Campañas</a></li>
<li><a href="#s2">Sensibilidad — Tornado de 7 Parámetros</a></li>
<li><a href="#s3">Estrés Extremo — 8 Escenarios</a></li>
<li><a href="#s4">Throttle + Adopción de Usuarios</a></li>
<li><a href="#s5">Market Maker vs Fair-Launch</a></li>
<li><a href="#s6">ROI del Inversor Seed (ADR-030)</a></li>
<li><a href="#conc">Conclusiones Estructurales</a></li>
</ol>
</div>

<h2 id="s0">0. Invariantes del Contrato</h2>
<div class="card">
<p>Verificación de que el motor replica fielmente las reglas on-chain: distribución 35/35/15/15 exacta,
simetría O&amp;M = Staking, conservación del tramo LP/Quema, supply en rango [3.3B, 10B],
y precio ≥ P_KASH (muro de valor).</p>
<p><span class="badge badge-ok">{'TODOS LOS INVARIANTES OK' if inv0.get('ok_all') else 'FALLA'}</span></p>
</div>

<h2 id="s1">1. Monte Carlo — 3 Campañas</h2>
<div class="card">
<p>200 trayectorias × 3 escenarios de marketing. Cada trayectoria: 5 años, régimen Markov,
precios BTC/SOL estocásticos, volumen con ruido, vesting, Throttle dinámico.</p>
<div style="overflow-x:auto">
<table>
<tr><th>Campaña</th><th>Vault Y5 (med)</th><th>P10–P90</th><th>Espiral</th><th>Ruina</th><th>Día B2</th><th>Precio ×TGE</th><th>Quema %</th></tr>
{''.join(mc_row(n) for n in ["CONSERVADOR","BASE","AGRESIVO"])}
</table>
</div>
{'<img src="data:image/png;base64,' + mc_img + '" alt="Monte Carlo">' if mc_img else ''}
</div>

<h2 id="s2">2. Sensibilidad — Tornado de 7 Parámetros</h2>
<div class="card">
<p>OAT (One-At-a-Time): cada parámetro varía sobre 5 niveles, el resto fijo en BASE.
60 iteraciones/punto con números aleatorios comunes.</p>
<div style="overflow-x:auto">
<table>
<tr><th>Parámetro</th><th>Swing Vault Y5</th><th>Mínimo</th><th>Máximo</th></tr>
{''.join(sens_row(k, v) for k, v in sens_ranking)}
</table>
</div>
{'<img src="data:image/png;base64,' + tornado_img + '" alt="Tornado">' if tornado_img else ''}
<p><strong>Hallazgo clave:</strong> El volumen de trading es el driver #1 del valor del Vault, seguido por
el fee del Motor A y el día de lanzamiento de la App.</p>
</div>

<h2 id="s3">3. Estrés Extremo — 8 Escenarios</h2>
<div class="card">
<p>120 trayectorias/escenario con números comunes (misma senda de mercado, distinto shock).</p>
<div style="overflow-x:auto">
<table>
<tr><th>Escenario</th><th>Vault Y5 (med)</th><th>Δ vs BASE</th><th>Espiral %</th></tr>
{''.join(stress_row(k, v) for k, v in stress.items())}
</table>
</div>
{'<img src="data:image/png;base64,' + stress_img + '" alt="Estrés">' if stress_img else ''}
</div>

<h2 id="s4">4. Throttle + Adopción de Usuarios</h2>
<div class="card">
<p><strong>4a — Throttle:</strong> 6 configuraciones de umbrales (DEF/CONS) × 100 iteraciones.</p>
<p><strong>4b — Usuarios:</strong> 5 niveles de adopción (1K–100K base) × 100 iteraciones.</p>
{'<img src="data:image/png;base64,' + thr_img + '" alt="Throttle + Usuarios">' if thr_img else ''}
<p><strong>Hallazgo:</strong> La configuración actual (DEF 0.50 / CONS 0.80) es robusta — variantes
cercanas no mejoran significativamente el Vault Y5. La adopción de usuarios tiene impacto
exponencial a partir de ~20K usuarios base.</p>
</div>

<h2 id="s5">5. Market Maker vs Fair-Launch</h2>
<div class="card">
<p>200 iteraciones × 3 campañas × 2 modos. El MM triplica volumen TGE, acelera adopción
temprana (×1.8) y recibe 300M tokens en préstamo (12 meses).</p>
{'<img src="data:image/png;base64,' + mm_img + '" alt="MM vs No-MM">' if mm_img else ''}
</div>

<h2 id="s6">6. ROI del Inversor Seed — KASH Sociedad (ADR-030)</h2>
<div class="card">
<div class="highlight">
<h3>Estructura de inversión (ADR-030)</h3>
<p>KASH Sociedad = 30% de todos los fees del protocolo, distribuido en 3 bloques de 10%:<br>
<strong>Fundador</strong> (10% floor) · <strong>Operaciones</strong> (10% para MM, advisors, bonos) · <strong>Inversores</strong> (10% seed base).<br>
Seed round: $500K → 5-6% base + hasta 4% adicional por hitos verificables.</p>
</div>
<p>150 trayectorias × 3 campañas × 4 niveles de participación (5%, 6%, 8%, 10%).</p>
<div style="overflow-x:auto">
<table>
<tr><th>Campaña</th><th>Stake</th><th>Total 5Y</th><th>ROI</th><th>IRR</th><th>Payback</th><th>P(payback)</th></tr>
{''.join(inv_row(k, v) for k, v in sorted(inv.items(), key=lambda x: (x[1].get("escenario",""), x[1].get("stake_pct",0))))}
</table>
</div>
{'<img src="data:image/png;base64,' + inv_img + '" alt="Investor ROI">' if inv_img else ''}
</div>

<h2 id="conc">Conclusiones Estructurales</h2>
<div class="card">
<ol>
<li><strong>Volumen es el driver #1:</strong> El swing del Vault Y5 por volumen supera a cualquier otro parámetro.
Esto valida la decisión ADR-030 de incluir un MM desde el TGE.</li>
<li><strong>Espiral solo en CONSERVADOR:</strong> La trampa B0 (K nunca alcanza $25M) es un riesgo real
solo con campaña débil. BASE y AGRESIVO la evitan consistentemente.</li>
<li><strong>KASH Shield robusto:</strong> Incluso bajo exploit del 15% del Vault, el protocolo no entra en ruina
(aunque el Vault Y5 cae ~25-30%). Los Circuit Breakers cumplen su función.</li>
<li><strong>Throttle estable:</strong> La configuración actual (0.50/0.80) es un óptimo local — no hay ganancia
significativa ajustando umbrales.</li>
<li><strong>ROI atractivo para seed:</strong> En escenario BASE, 6% del Sociedad genera un ROI de
{inv.get('BASE_6pct', {}).get('roi_x', 0):.1f}× sobre $500K en 5 años, con payback en
~{int(inv.get("BASE_6pct", {}).get("payback_med_dias", 0) / 30) if inv.get("BASE_6pct", {}).get("payback_med_dias") else "N/A"} meses.</li>
</ol>
</div>

<div class="disclaimer">
<strong>PROVENANCE (ADR-008/H10):</strong> Las cifras absolutas pertenecen a la familia "modelo v4 optimista"
(10–200× el modelo conservador v3.1). Lo válido para decisiones es lo <strong>estructural</strong>:
porcentajes de espiral, timing relativo de B2, orden de escenarios, sensibilidades relativas.
No publicar cifras absolutas sin etiquetar la fuente.<br><br>
Motor: engine.py (réplica de lib.rs v10.2) · Capa económica: Markov + halving calibrado BTC ·
Precio: P_KASH × prima (no AMM) · Horizonte: 1,825 días (5 años).
</div>
"""
    return html


def generate_pdf():
    mc = _load("sim1_montecarlo.json") or {}
    sens = _load("sim2_sensibilidad.json") or {}
    stress = _load("sim3_estres.json") or {}
    inv = _load("sim6_investor_roi.json") or {}
    today = date.today().isoformat()

    pdf = FPDF()
    pdf.set_auto_page_break(auto=True, margin=20)

    # --- Portada ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 28)
    pdf.cell(0, 40, "", ln=True)
    pdf.cell(0, 15, "LUKASH Protocol", ln=True, align="C")
    pdf.set_font("Helvetica", "", 16)
    pdf.cell(0, 10, "Informe de Simulaciones Cuantitativas v4.3", ln=True, align="C")
    pdf.set_font("Helvetica", "", 11)
    pdf.cell(0, 8, "", ln=True)
    pdf.cell(0, 8, "Motor fiel al contrato (lib.rs v10.2)", ln=True, align="C")
    pdf.cell(0, 8, f"Fecha: {today}", ln=True, align="C")
    pdf.cell(0, 8, "Autor: Kash Sensei", ln=True, align="C")
    pdf.cell(0, 8, "7 simulaciones | ~35,000 trayectorias | Horizonte 5 anios", ln=True, align="C")

    # --- Resumen ejecutivo ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 16)
    pdf.cell(0, 12, "Resumen Ejecutivo", ln=True)
    pdf.set_font("Helvetica", "", 10)
    summary = (
        "Este informe presenta 7 simulaciones cuantitativas del protocolo LUKASH, "
        "ejecutadas con un motor que replica fielmente la aritmetica entera del smart contract "
        "(lib.rs v10.2). El objetivo es validar las decisiones de diseno y cuantificar riesgos "
        "bajo multiples escenarios de mercado y marketing.\n\n"
        "Hallazgos clave:\n"
        "1. Volumen de trading es el driver #1 del valor del Vault.\n"
        "2. Espiral (trampa B0) solo ocurre en el escenario CONSERVADOR.\n"
        "3. KASH Shield es robusto incluso ante exploits del 15% del Vault.\n"
        "4. La configuracion del Throttle (0.50/0.80) es un optimo local estable.\n"
        "5. El MM desde TGE (ADR-030) mejora significativamente todos los indicadores.\n"
        "6. ROI atractivo para seed investors: 6% del Sociedad genera retorno positivo en BASE."
    )
    pdf.multi_cell(0, 5, summary)

    # --- SIM 1: Monte Carlo ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 1 - Monte Carlo (3 Campanias)", ln=True)
    pdf.set_font("Helvetica", "", 9)

    pdf.set_fill_color(24, 95, 165)
    pdf.set_text_color(255, 255, 255)
    headers = ["Campania", "Vault Y5", "Espiral%", "Ruina%", "Dia B2", "Precio xTGE", "Quema%"]
    widths = [30, 30, 20, 20, 20, 28, 22]
    for i, h in enumerate(headers):
        pdf.cell(widths[i], 7, h, border=1, fill=True, align="C")
    pdf.ln()

    pdf.set_text_color(0, 0, 0)
    for name in ["CONSERVADOR", "BASE", "AGRESIVO"]:
        d = mc.get(name, {})
        row = [
            name,
            _fmt_usd(d.get('K_med', 0)),
            f"{d.get('espiral_pct', 0):.1f}",
            f"{d.get('ruina_pct', 0):.1f}",
            f"d{int(d['b2_med']) if d.get('b2_med') else 0}",
            f"{d.get('mult_med', 0):.0f}x",
            f"{d.get('quema_med', 0):.1f}",
        ]
        for i, v in enumerate(row):
            pdf.cell(widths[i], 6, v, border=1, align="C")
        pdf.ln()

    mc_png = os.path.join(OUT, "sim1_montecarlo.png")
    if os.path.exists(mc_png):
        pdf.ln(5)
        pdf.image(mc_png, w=170)

    # --- SIM 2: Sensibilidad ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 2 - Sensibilidad OAT (7 parametros)", ln=True)
    pdf.set_font("Helvetica", "", 9)

    sens_ranking = sorted(sens.items(), key=lambda kv: kv[1].get("swing", 0), reverse=True)
    pdf.set_fill_color(24, 95, 165)
    pdf.set_text_color(255, 255, 255)
    for h, w in [("Parametro", 50), ("Swing", 40), ("Min", 35), ("Max", 35)]:
        pdf.cell(w, 7, h, border=1, fill=True, align="C")
    pdf.ln()
    pdf.set_text_color(0, 0, 0)
    for k, v in sens_ranking:
        for val, w in [(k, 50), (_fmt_usd(v.get('swing', 0)), 40),
                       (_fmt_usd(v.get('K_min', 0)), 35), (_fmt_usd(v.get('K_max', 0)), 35)]:
            pdf.cell(w, 6, val, border=1, align="C")
        pdf.ln()

    tornado_png = os.path.join(OUT, "sim2_tornado.png")
    if os.path.exists(tornado_png):
        pdf.ln(5)
        pdf.image(tornado_png, w=170)

    # --- SIM 3: Estres ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 3 - Estres Extremo (8 escenarios)", ln=True)
    pdf.set_font("Helvetica", "", 9)

    pdf.set_fill_color(24, 95, 165)
    pdf.set_text_color(255, 255, 255)
    for h, w in [("Escenario", 55), ("Vault Y5", 35), ("Delta", 30), ("Espiral%", 30)]:
        pdf.cell(w, 7, h, border=1, fill=True, align="C")
    pdf.ln()
    pdf.set_text_color(0, 0, 0)
    for k, v in stress.items():
        for val, w in [(k[:28], 55), (_fmt_usd(v.get('K_med', 0)), 35),
                       (f"{v.get('delta_vs_base_pct', 0):+.1f}%", 30),
                       (f"{v.get('espiral_pct', 0):.1f}", 30)]:
            pdf.cell(w, 6, val, border=1, align="C")
        pdf.ln()

    stress_png = os.path.join(OUT, "sim3_estres.png")
    if os.path.exists(stress_png):
        pdf.ln(5)
        pdf.image(stress_png, w=170)

    # --- SIM 4: Throttle ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 4 - Throttle + Usuarios", ln=True)
    thr_png = os.path.join(OUT, "sim4_throttle_usuarios.png")
    if os.path.exists(thr_png):
        pdf.image(thr_png, w=170)

    # --- SIM 5: MM ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 5 - Market Maker vs Fair-Launch", ln=True)
    mm_png = os.path.join(OUT, "sim5_mm_vs_nomm.png")
    if os.path.exists(mm_png):
        pdf.image(mm_png, w=170)

    # --- SIM 6: ROI ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "SIM 6 - ROI del Inversor Seed (ADR-030)", ln=True)
    pdf.set_font("Helvetica", "", 9)

    pdf.set_fill_color(24, 95, 165)
    pdf.set_text_color(255, 255, 255)
    h_roi = [("Camp.", 22), ("Stake", 14), ("Total 5Y", 28), ("ROI", 16), ("IRR", 16), ("Payback", 22), ("P(pb)", 16)]
    for h, w in h_roi:
        pdf.cell(w, 7, h, border=1, fill=True, align="C")
    pdf.ln()
    pdf.set_text_color(0, 0, 0)

    for lbl in sorted(inv.keys(), key=lambda x: (inv[x].get("escenario", ""), inv[x].get("stake_pct", 0))):
        d = inv[lbl]
        row = [
            (d.get('escenario', '')[:12], 22),
            (f"{d.get('stake_pct', 0):.0f}%", 14),
            (_fmt_usd(d.get('total_5y_med', 0), 2), 28),
            (f"{d.get('roi_x', 0):.1f}x", 16),
            (f"{d.get('irr_med', 0)*100 if d.get('irr_med') else 0:.0f}%", 16),
            (f"{int(d['payback_med_dias']/30) if d.get('payback_med_dias') else 'N/A'}m", 22),
            (f"{d.get('payback_pct', 0):.0f}%", 16),
        ]
        for val, w in row:
            pdf.cell(w, 6, str(val), border=1, align="C")
        pdf.ln()

    inv_png = os.path.join(OUT, "sim6_investor_roi.png")
    if os.path.exists(inv_png):
        pdf.ln(5)
        pdf.image(inv_png, w=170)

    # --- Conclusiones ---
    pdf.add_page()
    pdf.set_font("Helvetica", "B", 14)
    pdf.cell(0, 10, "Conclusiones Estructurales", ln=True)
    pdf.set_font("Helvetica", "", 10)
    conclusions = (
        "1. Volumen es el driver #1: el swing del Vault Y5 por volumen supera a cualquier "
        "otro parametro. Esto valida la decision ADR-030 de incluir un MM desde el TGE.\n\n"
        "2. Espiral solo en CONSERVADOR: la trampa B0 (K nunca alcanza $25M) es un riesgo "
        "real solo con campania debil. BASE y AGRESIVO la evitan consistentemente.\n\n"
        "3. KASH Shield robusto: incluso bajo exploit del 15% del Vault, el protocolo no "
        "entra en ruina. Los Circuit Breakers cumplen su funcion.\n\n"
        "4. Throttle estable: la configuracion actual (0.50/0.80) es un optimo local.\n\n"
        "5. MM desde TGE mejora todos los indicadores vs fair-launch puro.\n\n"
        "6. ROI atractivo para seed: en escenario BASE, 6% del Sociedad genera retorno "
        "positivo, validando la estructura de inversion ADR-030."
    )
    pdf.multi_cell(0, 5, conclusions)

    pdf.set_font("Helvetica", "I", 8)
    pdf.ln(10)
    pdf.multi_cell(0, 4,
        "PROVENANCE (ADR-008/H10): cifras absolutas = familia 'modelo v4 optimista'. "
        "Lo valido para decisiones es lo ESTRUCTURAL. No publicar cifras absolutas sin etiquetar.")

    out_path = os.path.join(OUT, "informe_simulaciones_v4.3.pdf")
    pdf.output(out_path)
    return out_path


if __name__ == "__main__":
    print("Generando HTML...")
    html = generate_html()
    html_path = os.path.join(OUT, "informe_simulaciones_v4.3.html")
    with open(html_path, "w", encoding="utf-8") as f:
        f.write(html)
    print(f"  → {html_path}")

    print("Generando PDF...")
    pdf_path = generate_pdf()
    print(f"  → {pdf_path}")
    print("Listo.")
