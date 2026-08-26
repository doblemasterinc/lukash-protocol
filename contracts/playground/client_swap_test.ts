// client_swap_test.ts — LUKASH v9.1: test execute_vault_swaps
// Precondición: process_fee ya ejecutado (hay pending_swap_*_usd > 0).

const PYTH_BTC = new anchor.web3.PublicKey("HovQMDrbAgAYPCmHVSrezcSmkMtXSSUsLDFANExrZh2J");
const PYTH_SOL = new anchor.web3.PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");

const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("config")], pg.PROGRAM_ID
);
const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("state")], pg.PROGRAM_ID
);

// --- ANTES ---
console.log("=== execute_vault_swaps ===");
const antes = await pg.program.account.protocolState.fetch(statePda);
console.log("ANTES pending_swap_cbtc_usd:", antes.pendingSwapCbtcUsd.toString());
console.log("ANTES pending_swap_sol_usd:", antes.pendingSwapSolUsd.toString());
console.log("ANTES pending_swap_lst_usd:", antes.pendingSwapLstUsd.toString());
console.log("ANTES pending_swap_usdc_res_usd:", antes.pendingSwapUsdcResUsd.toString());
console.log("ANTES pending_swap_usdc_lend_usd:", antes.pendingSwapUsdcLendUsd.toString());
console.log("ANTES cbtc_amount:", antes.cbtcAmount.toString());
console.log("ANTES sol_amount:", antes.solAmount.toString());
console.log("ANTES lst_amount:", antes.lstAmount.toString());
console.log("ANTES usdc_res_amount:", antes.usdcResAmount.toString());
console.log("ANTES usdc_lend_amount:", antes.usdcLendAmount.toString());

const totalPending = BigInt(antes.pendingSwapCbtcUsd.toString())
  + BigInt(antes.pendingSwapSolUsd.toString())
  + BigInt(antes.pendingSwapLstUsd.toString())
  + BigInt(antes.pendingSwapUsdcResUsd.toString())
  + BigInt(antes.pendingSwapUsdcLendUsd.toString());
console.log("Total pending USD:", totalPending.toString());

if (totalPending === 0n) {
  console.log("No hay pending swaps. Corre primero client.ts (process_fee).");
} else {
  // --- EJECUTAR ---
  const txSwap = await pg.program.methods.executeVaultSwaps()
    .accounts({
      config: configPda,
      state: statePda,
      pythBtcFeed: PYTH_BTC,
      pythSolFeed: PYTH_SOL,
      caller: pg.wallet.publicKey,
    }).rpc();
  console.log("\nTX:", txSwap);

  // --- DESPUES ---
  const despues = await pg.program.account.protocolState.fetch(statePda);
  console.log("\nDESPUES pending_swap_cbtc_usd:", despues.pendingSwapCbtcUsd.toString());
  console.log("DESPUES pending_swap_sol_usd:", despues.pendingSwapSolUsd.toString());
  console.log("DESPUES pending_swap_lst_usd:", despues.pendingSwapLstUsd.toString());
  console.log("DESPUES pending_swap_usdc_res_usd:", despues.pendingSwapUsdcResUsd.toString());
  console.log("DESPUES pending_swap_usdc_lend_usd:", despues.pendingSwapUsdcLendUsd.toString());
  console.log("DESPUES cbtc_amount:", despues.cbtcAmount.toString());
  console.log("DESPUES sol_amount:", despues.solAmount.toString());
  console.log("DESPUES lst_amount:", despues.lstAmount.toString());
  console.log("DESPUES usdc_res_amount:", despues.usdcResAmount.toString());
  console.log("DESPUES usdc_lend_amount:", despues.usdcLendAmount.toString());

  // --- CHECKS ---
  const pendAfter = BigInt(despues.pendingSwapCbtcUsd.toString())
    + BigInt(despues.pendingSwapSolUsd.toString())
    + BigInt(despues.pendingSwapLstUsd.toString())
    + BigInt(despues.pendingSwapUsdcResUsd.toString())
    + BigInt(despues.pendingSwapUsdcLendUsd.toString());

  const cbtcGrew = BigInt(despues.cbtcAmount.toString()) > BigInt(antes.cbtcAmount.toString());
  const solGrew = BigInt(despues.solAmount.toString()) > BigInt(antes.solAmount.toString());
  const lstGrew = BigInt(despues.lstAmount.toString()) > BigInt(antes.lstAmount.toString());
  const usdcResGrew = BigInt(despues.usdcResAmount.toString()) > BigInt(antes.usdcResAmount.toString());
  const usdcLendGrew = BigInt(despues.usdcLendAmount.toString()) > BigInt(antes.usdcLendAmount.toString());

  console.log("\n=== RESULTADO ===");
  console.log("Pending limpiados (=0):", pendAfter === 0n ? "PASS" : "FAIL (quedan " + pendAfter.toString() + ")");
  console.log("cbtc_amount crecio:", cbtcGrew ? "PASS" : "FAIL");
  console.log("sol_amount crecio:", solGrew ? "PASS" : "FAIL");
  console.log("lst_amount crecio:", lstGrew ? "PASS" : "FAIL");
  console.log("usdc_res_amount crecio:", usdcResGrew ? "PASS" : "FAIL");
  console.log("usdc_lend_amount crecio:", usdcLendGrew ? "PASS" : "FAIL");

  if (pendAfter === 0n && cbtcGrew && solGrew && lstGrew && usdcResGrew && usdcLendGrew) {
    console.log("\nCapa 2 COMPLETA: process_fee + execute_vault_swaps verificados en devnet.");
    console.log("Pending USD -> native amounts a precio Pyth funciona.");
  }
}
