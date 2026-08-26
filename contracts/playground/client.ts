// client.ts — LUKASH v9.1: test process_fee Motor A
// Precondición: v9.1 desplegado, initialize + initialize_burn_vault + burn_vault fondeado.

const LUKA_MINT = new anchor.web3.PublicKey("2DatjaKezpYkB3TitgwYGvpwTAWiFxN4JEwpYnk3Luvr");
const PYTH_BTC = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
const PYTH_SOL = new anchor.web3.PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
const TOKEN_PROG = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("config")], pg.PROGRAM_ID
);
const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("state")], pg.PROGRAM_ID
);
const [burnVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("burn_vault")], pg.PROGRAM_ID
);

// --- Oracle ---
console.log("=== Oracle feed ===");
await pg.program.methods.updateOracleState(
  new anchor.BN(100000),
  new anchor.BN(100000),
  new anchor.BN("10000000000000000")
).accounts({
  config: configPda,
  state: statePda,
  authority: pg.wallet.publicKey,
}).rpc();
console.log("OK: LUKA=$0.10, EMA=$0.10, supply=10B");

// --- Vault valuation (Pyth) ---
console.log("\n=== Vault valuation ===");
await pg.program.methods.refreshVaultValuation(
  new anchor.BN(160000000),
  new anchor.BN(100000),
  new anchor.BN(50000000),
  new anchor.BN(100000000000),
  new anchor.BN(80000000000),
  new anchor.BN(5000000000),
  new anchor.BN(1000000000),
).accounts({
  config: configPda,
  state: statePda,
  pythBtcFeed: PYTH_BTC,
  pythSolFeed: PYTH_SOL,
  caller: pg.wallet.publicKey,
}).rpc();
console.log("OK: vault valuation refreshed");

// --- ANTES ---
console.log("\n=== process_fee Motor A ($100) ===");
const antes = await pg.program.account.protocolState.fetch(statePda);
console.log("ANTES supply:", antes.currentSupply.toString());
console.log("ANTES burned:", antes.burnedTotal.toString());
console.log("ANTES vault_core:", antes.vaultCoreUsd.toString());
console.log("ANTES pend_cbtc:", antes.pendingSwapCbtcUsd.toString());
console.log("ANTES pend_sol:", antes.pendingSwapSolUsd.toString());
console.log("ANTES pend_lst:", antes.pendingSwapLstUsd.toString());
console.log("ANTES pend_usdc_r:", antes.pendingSwapUsdcResUsd.toString());
console.log("ANTES pend_usdc_l:", antes.pendingSwapUsdcLendUsd.toString());

// --- process_fee ---
const txFee = await pg.program.methods.processFee(
  new anchor.BN(100000000),
  0, 0, 0, false
).accounts({
  config: configPda,
  state: statePda,
  caller: pg.wallet.publicKey,
  burnVault: burnVaultPda,
  lukaMint: LUKA_MINT,
  tokenProgram: TOKEN_PROG,
}).rpc();
console.log("TX:", txFee);

// --- DESPUES ---
const despues = await pg.program.account.protocolState.fetch(statePda);
console.log("\nDESPUES supply:", despues.currentSupply.toString());
console.log("DESPUES burned:", despues.burnedTotal.toString());
console.log("DESPUES vault_core:", despues.vaultCoreUsd.toString());
console.log("DESPUES pend_cbtc:", despues.pendingSwapCbtcUsd.toString());
console.log("DESPUES pend_sol:", despues.pendingSwapSolUsd.toString());
console.log("DESPUES pend_lst:", despues.pendingSwapLstUsd.toString());
console.log("DESPUES pend_usdc_r:", despues.pendingSwapUsdcResUsd.toString());
console.log("DESPUES pend_usdc_l:", despues.pendingSwapUsdcLendUsd.toString());

// --- Checks ---
const sBefore = BigInt(antes.currentSupply.toString());
const sAfter = BigInt(despues.currentSupply.toString());
const burned = sBefore - sAfter;
console.log("\nQuema:", burned.toString(), "tokens");

const vBefore = BigInt(antes.vaultCoreUsd.toString());
const vAfter = BigInt(despues.vaultCoreUsd.toString());
const vGrew = vAfter - vBefore;
console.log("Vault Core crecio:", vGrew.toString());

const pBefore = BigInt(antes.pendingSwapCbtcUsd.toString())
  + BigInt(antes.pendingSwapSolUsd.toString())
  + BigInt(antes.pendingSwapLstUsd.toString())
  + BigInt(antes.pendingSwapUsdcResUsd.toString())
  + BigInt(antes.pendingSwapUsdcLendUsd.toString());
const pAfter = BigInt(despues.pendingSwapCbtcUsd.toString())
  + BigInt(despues.pendingSwapSolUsd.toString())
  + BigInt(despues.pendingSwapLstUsd.toString())
  + BigInt(despues.pendingSwapUsdcResUsd.toString())
  + BigInt(despues.pendingSwapUsdcLendUsd.toString());
const pendingNew = pAfter - pBefore;
console.log("Pending swaps nuevos:", pendingNew.toString());

console.log("\n=== RESULTADO ===");
console.log("Quema real:", burned > 0n ? "PASS" : "FAIL");
console.log("Vault Core crecio:", vGrew > 0n ? "PASS" : "FAIL");
console.log("Pending swaps:", pendingNew > 0n ? "PASS" : "FAIL");
