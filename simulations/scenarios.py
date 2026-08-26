"""
LUKASH — Escenarios de marketing (campaña de lanzamiento) alineados con el
informe cuantitativo v4.2 y lukash_mc_v4.py. La AGRESIVIDAD de la campaña es
la palanca central: define el volumen temprano que alimenta el Vault y ejecuta
la quema antes de caer en la trampa B0 (espiral).
"""

MKT = {
    "CONSERVADOR": {
        "desc": "Sin MMs. 2-3 KOLs pequeños. CAC $9. Crecimiento orgánico. Lanzamiento DÉBIL.",
        "app_dia": 180, "vol_tge": 200_000, "vol_pico": 3_000_000, "dias_pico": 1080,
        "hitos": {6: 4_000, 18: 30_000, 36: 120_000, 60: 300_000},
    },
    "BASE": {
        "desc": "3 KOLs con KASH Lock. 2 MMs desde D1. CAC $6. Flywheel moderado.",
        "app_dia": 120, "vol_tge": 800_000, "vol_pico": 12_000_000, "dias_pico": 900,
        "hitos": {6: 15_000, 18: 120_000, 36: 450_000, 60: 1_100_000},
    },
    "AGRESIVO": {
        "desc": "MMs desde D1. KOLs virales. Manadas activas. CAC $4. Flywheel pleno. Lanzamiento FUERTE.",
        "app_dia": 60, "vol_tge": 3_000_000, "vol_pico": 60_000_000, "dias_pico": 720,
        "hitos": {6: 60_000, 18: 400_000, 36: 1_200_000, 60: 3_500_000},
    },
}
