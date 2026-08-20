# LUKASH Contracts — Milestone 1 (devnet)

Núcleo económico del protocolo LUKASH en Anchor/Solana: **Universal Fee Extractor + Vault + Throttle +
conmutación B0/B2**. Un solo programa (`lukash_protocol`) modular. Ver la spec: `../specs/02-smart-contracts-milestone-1.md`.

> **Estado:** scaffold completo, **listo para compilar y probar** cuando esté instalado el toolchain.
> El modelo trata los montos como notional USD (6 dec); la integración real de tokens SPL/Jupiter/burn es del
> siguiente milestone. Aquí se prueba la LÓGICA económica de forma aislada (distribución 35/35/15/15, fees por
> etapa/capa, Throttle, B0/B2, Timelock, pausa).

## Qué hay
```
programs/lukash_protocol/src/
  lib.rs         # programa: initialize, process_fee, update_oracle_state, switch_motor_b, admin/timelock, pause
  state.rs       # cuentas: ProtocolConfig, ProtocolState
  constants.rs   # parámetros del Blueprint v4.3 §13
  errors.rs      # errores del protocolo
tests/lukash_protocol.ts   # pruebas (distribución, fees, throttle, timelock, pausa)
```

## Requisitos (toolchain) — NO instalado en esta máquina
Anchor requiere un entorno tipo Linux. En Windows, la ruta recomendada es **WSL2**.

### Opción A — WSL2 (recomendada para desarrollo real)
```bash
# 1. Instalar WSL2 (PowerShell como admin, en Windows):  wsl --install
# 2. Dentro de Ubuntu (WSL):
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"   # Solana CLI
cargo install --git https://github.com/coral-xyz/anchor avm --force  # AVM
avm install 0.30.1 && avm use 0.30.1                                  # Anchor
# Rust: https://rustup.rs   |   Node + yarn: para los tests
```

### Opción B — Solana Playground (cero instalación, en el navegador)
`https://beta.solpg.io` — pega el programa, compila y despliega a devnet desde el navegador. Ideal para
iterar rápido sin instalar nada. (Los tests TS locales no aplican allí; se prueba desde la UI.)

## Compilar y probar (una vez con toolchain)
```bash
cd contracts
yarn install
anchor keys sync     # genera el program id real y lo sincroniza en lib.rs y Anchor.toml
anchor build
anchor test          # corre los tests contra un validador local
```

## Desplegar a devnet
```bash
solana config set --url devnet
solana airdrop 2                     # SOL de prueba para el pagador
anchor deploy --provider.cluster devnet
```

## Checklist de seguridad (cubierto en el scaffold)
- [x] Aritmética `checked_*` en toda operación (u128 intermedio en `mul_bps`).
- [x] Invariante de distribución (suma == fee) verificado en runtime.
- [x] Composición del Vault validada a 100% en `initialize`.
- [x] Access control: `has_one = authority` en instrucciones admin; Signer requerido.
- [x] Timelock 48h para parámetros críticos (stage, k_min, authority).
- [x] Circuit Breaker (`set_pause`) bloquea `process_fee`.
- [ ] **Pendiente milestone 2:** integración SPL/Jupiter, quema real a null, staking distributor, cNFT/Aura,
      fallback de oráculo (Switchboard), y **auditoría Halborn/OtterSec antes de mainnet**.

## Nota sobre pruebas que requieren avance de tiempo
`switch_motor_b` real (alcanzar $25M) y el hito Jaguar Lock por tiempo (12 meses) requieren manipular el reloj
del validador (p. ej. con `solana-bankrun` / `Clock` sysvar override). Los tests actuales cubren las
transiciones y validaciones que no dependen del tiempo; añadir esos casos con bankrun en el milestone 2.
