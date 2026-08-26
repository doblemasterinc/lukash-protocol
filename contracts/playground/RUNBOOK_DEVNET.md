# RUNBOOK — Disparar deploy + token $LUKA en devnet (Solana Playground)

> Todo listo para "copiar-pegar" el día que haya SOL de prueba. El ÚNICO bloqueo actual es el
> faucet de devnet (rate-limit/caído). Nada de esto requiere instalar nada — todo en https://beta.solpg.io
>
> Estado 2026-08-20: contrato COMPILA ("Build successful"). Falta SOL para desplegar. Ver §0.

---

## §0 — Conseguir SOL de prueba (el bloqueo actual)
Necesitas **~1.5 SOL** en la wallet de Playground. Orden de intento:
1. **CLI, poco a poco:** `solana airdrop 1` (reintentar cada ~30s; probar `0.5`).
2. **Cambiar IP** (hotspot del celular / VPN) y reintentar el CLI. El límite es por IP.
3. **RPC propio (Helius):** cuenta gratis en https://www.helius.dev → copiar RPC devnet
   (`https://devnet.helius-rpc.com/?api-key=xxxx`) → en Playground, clic en el endpoint (junto a la
   wallet) → **Custom** → pegar URL → reintentar `solana airdrop 1`. Esto además evita el bug
   "body stream already read" del RPC por defecto.
4. **Faucets web:** https://faucet.solana.com (pide GitHub con antigüedad) · https://solfaucet.com
5. **Último recurso:** esperar unas horas — los límites de devnet resetean solos.

Verificar en cualquier momento: `solana balance`  (objetivo: > 1.5 SOL)

---

## §1 — Desplegar el programa (Milestone 1)
1. Abrir el proyecto `lukash` en https://beta.solpg.io (si no está, recrear: *Create new project* →
   Anchor → pegar `lib.rs` → **Build** hasta "Build successful").
2. Wallet conectada (abajo-izq) y en **devnet**. Saldo > 1.5 SOL (ver §0).
3. Barra izquierda → ícono **Deploy** (cohete) → **Deploy**. Espera ~30-60s.
4. Copiar el **Program Id** que devuelve. Anotarlo abajo en §4.

> El `declare_id!` del código es un placeholder; Playground sincroniza el Program Id real al desplegar.

---

## §2 — Crear el token $LUKA (SPL) para VERLO en el explorador
> Se usa **Token-2022 con metadata on-chain** (nombre y símbolo quedan on-chain, sin hosting).
> El **logo** es opcional y va en §3 (necesita una URL pública). Sin logo igual se ve "LUKASH / LUKA".
> Todo se corre en la **terminal de Playground**.

```bash
# 1) Crear el mint con extensión de metadata (Token-2022), 6 decimales
spl-token create-token --program-2022 --enable-metadata --decimals 6
#    -> anota el MINT ADDRESS que imprime (empieza con letras/números)

# 2) Inicializar la metadata on-chain: NOMBRE, SÍMBOLO, URI (deja la URI de §3 o "" por ahora)
spl-token initialize-metadata <MINT> "LUKASH" "LUKA" "https://raw.githubusercontent.com/doblemasterinc/lukash-brand/main/luka.json"

# 3) Crear tu cuenta asociada para el token
spl-token create-account <MINT>

# 4) Acuñar el supply total: 10,000,000,000 (10B)
spl-token mint <MINT> 10000000000

# 5) (Opcional pero recomendado) Verificar
spl-token supply <MINT>
spl-token accounts
```

**Verlo en el explorador:** abrir `https://explorer.solana.com/address/<MINT>?cluster=devnet`.
Debe mostrar **LUKASH (LUKA)**, 6 decimales, supply 10B. (El logo aparece solo si §3 está hecho.)

> Si `--enable-metadata` no está soportado por la versión de `spl-token` de Playground, avísame:
> caemos al camino clásico SPL + Metaplex Token Metadata (unos comandos más).

---

## §3 — Logo del token (opcional, cuando quieras que salga el isotipo)
El explorador lee el logo desde la **URI** de la metadata → un JSON público → campo `image`.
La URI apunta a `luka.json` (ya preparado en `brand/token-metadata/luka.json`). Falta **hostearlo público**
porque `lukash-protocol` es privado. Opciones (elige una):
- **Repo público mínimo** `lukash-brand`: subir `luka.json` + `isotipo.png` → usar las URLs `raw.githubusercontent.com`.
- **Vercel** (ya lo usas para OILO): subir los 2 archivos a un proyecto estático → usar esas URLs.
- **IPFS gratis** (Pinata/web3.storage): subir imagen y JSON → usar el gateway `https://ipfs.io/ipfs/...`.

Pasos una vez hosteado:
1. Editar `brand/token-metadata/luka.json` → poner la URL pública real del isotipo en `"image"`.
2. Subir `luka.json` al host → obtener su URL pública.
3. Actualizar la URI on-chain:
```bash
spl-token update-metadata <MINT> uri "https://<tu-host>/luka.json"
```

---

## §4 — Registro
- **Token $LUKA creado en devnet: 2026-08-20** ✅
  - Mint: `2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr` (Token clásico SPL, 6 dec, supply 10B)
  - Mint Authority: `5PMsqNBZ5cQWRLLU9XihEe8cp2kxka9xdK9pRNsg9xBf` (wallet Playground)
  - Explorer: https://explorer.solana.com/address/2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr?cluster=devnet
  - Estado: "Unknown Token" — falta metadata (nombre/logo) vía Metaplex (ver §3 + nota abajo).
- **Programa deployado en devnet: 2026-08-22** ✅
  - Program Id: `AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy`
  - Explorer: https://explorer.solana.com/address/AmRWTQtJHiuRdFcTwZdVDUkWvv5w3rxCFebsgWqmiCuy?cluster=devnet
  - Versión deployada: lib.rs v7 (Sprint 5A — hardening). Pendiente: v9 (Sprint 5B Fase A+B+C).

### Nota migración v8→v9 (Sprint 5B Fase B+C: Capa 2 oráculos Pyth + swaps)

> v9 cambia `refresh_vault_valuation` y agrega `execute_vault_swaps`. ProtocolState crece (+40 bytes: 5 pending swap fields). PDAs de v8 incompatibles → cerrar y re-inicializar en devnet.

**Secuencia de deploy completa para v9:**
1. En Playground, pegar `lib.rs` v9 completo en `src/lib.rs`. Cargo.toml ya tiene `anchor-spl`.
2. Build (puede tardar ~2 min si es primer build con anchor-spl).
3. Si hay PDAs existentes de v8: ejecutar `close_protocol` (cierra config + state, devuelve rent).
4. Deploy (upgrade in-place al mismo Program Id).
5. Llamar `initialize` para crear nuevas PDAs con los campos actualizados.
6. Llamar `initialize_burn_vault` para crear el burn_vault PDA.
7. Fondear burn_vault con tokens $LUKA: `spl-token transfer <MINT> <AMOUNT> <BURN_VAULT_PDA> --fund-recipient`.
8. Verificar con `update_oracle_state` (para luka_price, ema30, current_supply).
9. Probar `refresh_vault_valuation` con feeds Pyth devnet reales.
10. Probar `process_fee` Motor A → verificar `pending_swap_*_usd` acumulados.
11. Probar `execute_vault_swaps` → verificar `*_amount` actualizados a precio Pyth.

**Flujo operativo completo post-deploy:**
```
update_oracle_state(luka_price, ema30, supply)     ← keeper, Capa 1
refresh_vault_valuation(lst, luka, amounts...)      ← permissionless, Pyth BTC/SOL
process_fee(amount, motor)                          ← cada transacción
execute_vault_swaps()                               ← keeper, periódico (convierte pending → native)
```

### Detalle cambios en v9

#### `refresh_vault_valuation` (Capa 2 oráculos Pyth)
`lib.rs` v9 cambia `refresh_vault_valuation` de authority-gated a **permissionless** con feeds Pyth.

**Cambios en la interfaz:**
1. `refresh_vault_valuation` pierde 2 parámetros (`btc_price_usd`, `sol_price_usd`) — se leen de Pyth.
2. Los 7 parámetros restantes se mantienen: `lst_price_usd`, `luka_price_usd`, `cbtc_amount`, `sol_amount`, `lst_amount`, `usdc_res_amount`, `usdc_lend_amount`.
3. El contexto `RefreshVaultValuation` pierde `authority: Signer` y gana:
   - `pyth_btc_feed: AccountInfo` — feed Pyth BTC/USD devnet: `HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J`
   - `pyth_sol_feed: AccountInfo` — feed Pyth SOL/USD devnet: `J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix`
   - `authority: Signer` — restringido a authority (v10: `has_one = authority`)

**Actualizar `client.ts`:**
```typescript
const PYTH_BTC_USD_DEVNET = new PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
const PYTH_SOL_USD_DEVNET = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");

await program.methods
  .refreshVaultValuation(
    lstPriceUsd,    // u64, USD 6-dec
    lukaPriceUsd,   // u64, USD 6-dec
    cbtcAmount, solAmount, lstAmount, usdcResAmount, usdcLendAmount
  )
  .accounts({
    config: configPda,
    state: statePda,
    pythBtcFeed: PYTH_BTC_USD_DEVNET,
    pythSolFeed: PYTH_SOL_USD_DEVNET,
    authority: wallet.publicKey,
  })
  .rpc();
```

**Nuevo evento `VaultValuationRefreshed`:** ahora incluye `btc_price_usd` y `sol_price_usd` (precios leídos de Pyth).

#### `execute_vault_swaps` (Capa 2 swaps a precio de oráculo)

Nueva instrucción permissionless. Convierte los USD pendientes de swap (acumulados por `process_fee`) a balances nativos de activos a precio de oráculo Pyth.

**Campos nuevos en `ProtocolState`:**
- `pending_swap_cbtc_usd`, `pending_swap_sol_usd`, `pending_swap_lst_usd`, `pending_swap_usdc_res_usd`, `pending_swap_usdc_lend_usd` — USD 6-dec acumulados por `process_fee`, consumidos por `execute_vault_swaps`.

**Contexto `ExecuteVaultSwaps`:**
```typescript
await program.methods
  .executeVaultSwaps()
  .accounts({
    config: configPda,
    state: statePda,
    pythBtcFeed: PYTH_BTC_USD_DEVNET,
    pythSolFeed: PYTH_SOL_USD_DEVNET,
    authority: wallet.publicKey,
  })
  .rpc();
```

**Comportamiento:**
- Lee `pending_swap_*_usd` del state.
- Si total_pending == 0 → error `NoPendingSwaps`.
- Convierte USD a nativos: `native = usd * scale / price` (helper `usd_to_native`).
- cBTC usa BTC/USD de Pyth, SOL usa SOL/USD, LST usa SOL/USD como proxy, USDC es 1:1.
- Actualiza `*_amount` en state.
- Pone a cero los `pending_swap_*_usd`.
- Emite `VaultSwapsExecuted` con montos nativos, precios, total swapped, timestamp.

**Nota devnet:** estos "swaps" son accounting puro — no mueven tokens reales. En mainnet, esta instrucción sería reemplazada por CPIs a Jupiter V6. La interfaz de cuentas (Pyth feeds + authority) es compatible.

### Nota migración v7→v8 (Sprint 5B Fase A: Capa 2 quema real)
`lib.rs` v8 agrega `anchor-spl` como dependencia. Antes de compilar en Playground:
1. En la pestaña **Cargo.toml** del proyecto, agregar `anchor-spl = "0.30.1"` bajo `[dependencies]`.
2. Pegar el `lib.rs` v8 completo en `src/lib.rs`.
3. Build. El primer build con anchor-spl tarda más (~2 min) porque descarga las crates.
4. Después del deploy, llamar `initialize_burn_vault` para crear el burn_vault PDA:
   - Requiere que el mint de $LUKA ya exista (`2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr`).
   - El burn_vault se crea con authority = state PDA.
5. Fondear el burn_vault con tokens $LUKA para que las quemas reales funcionen:
   ```bash
   spl-token transfer <LUKA_MINT> <AMOUNT> <BURN_VAULT_PDA> --fund-recipient
   ```
   La PDA burn_vault se deriva: `findProgramAddress([b"burn_vault"], programId)`.
6. `process_fee` y `execute_deferred_burn` ahora requieren 3 cuentas adicionales:
   `burn_vault`, `luka_mint`, `token_program`. Actualizar el `client.ts` correspondientemente.

### Nota migración v3→v4 (Sprint 2)
`ProtocolState` creció en 64 bytes (8 campos nuevos para token accounting). Si el programa v3
ya fue inicializado (existen las PDAs `config` y `state`), Anchor no podrá deserializarlas con
la v4. **Solución devnet:** cerrar las PDAs existentes y re-inicializar. Dos opciones:
1. **Re-deploy a nueva dirección** (Playground: Create new project, Build, Deploy → nuevas PDAs).
2. **Mismo programa:** agregar instrucción temporal `close_accounts` que cierra ambas PDAs con
   `close = authority`, re-deploy v4 sobre el mismo Program Id, y luego `initialize` de nuevo.
Si **nunca** se llamó `initialize` en v3 (no existen PDAs), simplemente re-deploy y `initialize`.

### Nota metadata (por qué no se hizo con spl-token)
La versión de `spl-token` de Playground NO soporta `--enable-metadata` ni `--program-2022` (es vieja). El token
se creó clásico SPL. Para nombre/símbolo/logo hay que usar **Metaplex Token Metadata** (no el CLI):
- La **mint authority** es la wallet de Playground → cualquier herramienta que ponga metadata debe firmar con esa wallet.
- Opción recomendada: **metaboss** local con el keypair exportado de Playground, O un script TS de Metaplex en el
  cliente de Playground. Requiere primero **hostear** `luka.json` + isotipo público (ver §3).
