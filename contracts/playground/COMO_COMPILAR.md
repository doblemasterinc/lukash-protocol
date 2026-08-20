# Cómo compilar los contratos ($LUKA) — 4 pasos, sin instalar nada

> El archivo `lib.rs` de esta carpeta es el programa completo del Milestone 1 en **un solo archivo**
> (listo para Solana Playground). Ya pasó una revisión de código; el único error real (comentarios `//!`)
> quedó corregido. Compilar es literalmente 2 clics.

## Solana Playground (navegador, cero instalación)

1. Abre **https://beta.solpg.io**
2. Clic en **"Create a new project"** → nómbralo `lukash` → framework **Anchor** → Create.
3. En el explorador de la izquierda, abre **`src/lib.rs`**, **borra todo** su contenido y **pega** el contenido completo de `lib.rs` de esta carpeta.
4. Clic en **Build** (ícono de martillo/llave en la barra izquierda, o `Ctrl+Shift+B`).

Verás la salida abajo. Si dice **"Build successful"**, compiló. Si hay errores, cópialos y me los pasas — los resuelvo.

## Notas
- El **program id** en el código es un placeholder (`Fg6Pa...`). Playground genera el suyo al compilar; para desplegar, deja que Playground lo sincronice.
- Para **desplegar a devnet** desde Playground: conecta una wallet de prueba (arriba a la derecha "Not connected"), pide SOL de devnet con `solana airdrop 2` en la terminal de Playground, y clic en **Deploy**.
- Este milestone prueba la **lógica económica** (distribución 35/35/15/15, Vault, Throttle, B0/B2, Timelock, pausa) de forma aislada. La integración real de tokens (SPL/Jupiter/quema) y la auditoría externa son pasos posteriores (pre-mainnet).

## Alternativa local (si algún día quieres tu propio entorno)
WSL2 + Rust + Solana CLI + Anchor (`avm install 0.30.1`). Más setup; Playground es más simple para empezar.
