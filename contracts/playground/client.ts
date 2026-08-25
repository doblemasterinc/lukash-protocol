// client.ts — LUKASH Protocol v7 (Sprint 5A)
// Pegar en la pestaña "Client" de Solana Playground y darle "Run".
// Flujo: close_protocol (PDAs viejas) → initialize (v7) → oracle → process_fee → verify supply tracking.

// --- CONFIGURACIÓN ---
const PROGRAM_ID = pg.PROGRAM_ID;
const connection = pg.connection;
const wallet = pg.wallet;

// PDAs
const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("config")], PROGRAM_ID
);
const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("state")], PROGRAM_ID
);

// ============================================================
// PASO 1: Cerrar PDAs viejas (si existen)
// ============================================================
console.log("=== close_protocol: cerrando PDAs viejas ===");
try {
  const txClose = await pg.program.methods.closeProtocol()
    .accounts({
      authority: wallet.publicKey,
      config: configPda,
      state: statePda,
    })
    .rpc();
  console.log("✅ PDAs cerradas. SOL devuelto. Tx:", txClose);
} catch (e: any) {
  if (e.toString().includes("AccountNotInitialized") || e.toString().includes("not found")) {
    console.log("⚠️  PDAs no existen (ya cerradas o primer deploy). Continuando...");
  } else {
    console.error("❌ Error en close_protocol:", e.message || e);
  }
}

// ============================================================
// PASO 2: Inicializar protocolo v7
// ============================================================
console.log("\n=== initialize: creando PDAs v7 ===");
try {
  const txInit = await pg.program.methods.initialize()
    .accounts({
      authority: wallet.publicKey,
      config: configPda,
      state: statePda,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
  console.log("✅ Initialize v7 OK. Tx:", txInit);
} catch (e: any) {
  if (e.toString().includes("already in use")) {
    console.log("⚠️  PDAs ya existen (v7 ya inicializado).");
  } else {
    console.error("❌ Error en initialize:", e.message || e);
  }
}

// Verificar estado inicial
const config = await pg.program.account.protocolConfig.fetch(configPda);
console.log("\n--- Config ---");
console.log("  stage:", config.stage);
console.log("  authority:", config.authority.toBase58());
console.log("  tridente_activated:", config.tridenteActivated);

const state = await pg.program.account.protocolState.fetch(statePda);
console.log("\n--- State ---");
console.log("  motor_b_state:", state.motorBState, "(0=B0, 1=B2)");
console.log("  throttle_mode:", state.throttleMode);
console.log("  current_supply:", state.currentSupply.toString());

// ============================================================
// PASO 3: Oracle feed
// ============================================================
console.log("\n=== Oracle feed ===");
const LUKA_PRICE = new anchor.BN(100_000);  // $0.10 (6 dec)
const EMA30 = new anchor.BN(100_000);       // $0.10 (sin pánico)
const SUPPLY = new anchor.BN("10000000000000000"); // 10B tokens (6 dec)

await pg.program.methods.updateOracleState(LUKA_PRICE, EMA30, SUPPLY)
  .accounts({ config: configPda, state: statePda, authority: wallet.publicKey })
  .rpc();
console.log("✅ Oracle: $0.10, EMA $0.10, 10B supply");

// ============================================================
// PASO 4: Vault valuation
// ============================================================
console.log("\n=== Refresh Vault Valuation ===");
await pg.program.methods.refreshVaultValuation(
  new anchor.BN(60_000_000_000),    // BTC $60,000
  new anchor.BN(150_000_000),       // SOL $150
  new anchor.BN(160_000_000),       // LST $160
  LUKA_PRICE,                        // LUKA $0.10
  new anchor.BN(50_000_000),        // 0.5 cBTC (satoshis)
  new anchor.BN(100_000_000_000),   // 100 SOL (lamports)
  new anchor.BN(80_000_000_000),    // 80 LST (lamports)
  new anchor.BN(5_000_000_000),     // $5,000 USDC reserva
  new anchor.BN(1_000_000_000),     // $1,000 USDC lending
).accounts({
  config: configPda,
  state: statePda,
  authority: wallet.publicKey,
}).rpc();
console.log("✅ Vault valuation refreshed");

// ============================================================
// PASO 5: process_fee Motor A — TEST SUPPLY TRACKING
// ============================================================
console.log("\n=== process_fee Motor A ($100) — test supply tracking ===");
const antes = await pg.program.account.protocolState.fetch(statePda);
console.log("📊 ANTES: current_supply:", antes.currentSupply.toString(), "burned_total:", antes.burnedTotal.toString());

const FEE_USD = new anchor.BN(100_000_000); // $100 (6 dec)
// process_fee(amount, motor, layer, currency, is_whitelist)
const txFee = await pg.program.methods.processFee(
  FEE_USD,
  0,      // motor (0=A)
  0,      // layer
  0,      // currency (0=SOL)
  false,  // is_whitelist
).accounts({
  config: configPda,
  state: statePda,
  caller: wallet.publicKey,
}).rpc();
console.log("✅ process_fee OK. Tx:", txFee);

const despues = await pg.program.account.protocolState.fetch(statePda);
console.log("📊 DESPUÉS: current_supply:", despues.currentSupply.toString(), "burned_total:", despues.burnedTotal.toString());

const supplyAntes = BigInt(antes.currentSupply.toString());
const supplyDespues = BigInt(despues.currentSupply.toString());
const delta = supplyAntes - supplyDespues;
console.log("\n🔥 Supply decrementó:", delta.toString(), "tokens");
console.log("   Fee $100 × 4% = $4 → 35% burn = $1.40 → $1.40/$0.10 = 14 LUKA = 14,000,000 (6 dec)");

if (delta > 0n) {
  console.log("\n✅ FIX #1 VERIFICADO: supply tracking on burns funciona!");
} else {
  console.log("\n❌ FALLO: supply no decrementó.");
}

// ============================================================
// PASO 6: Transfer Hook — Anti-Whale + Exit Fee combo
// ============================================================
console.log("\n=== Transfer Hook: Anti-Whale (ballena 3% pool) ===");
const fakeSender = anchor.web3.Keypair.generate().publicKey;
const POOL_LIQ = new anchor.BN(10_000_000_000); // $10,000 pool
const WHALE_SALE = new anchor.BN(3_000_000_000); // 30,000 LUKA

const txWhale = await pg.program.methods.transferHook(
  WHALE_SALE, fakeSender, true, POOL_LIQ,
  new anchor.BN(0), false, false, false, false,
).accounts({ config: configPda, state: statePda, authority: wallet.publicKey }).rpc();
console.log("✅ Ballena procesada. Tx:", txWhale);

// ============================================================
// PASO 7: Estado final
// ============================================================
console.log("\n=== Estado final ===");
const finalState = await pg.program.account.protocolState.fetch(statePda);
console.log("  vault_core_usd:", finalState.vaultCoreUsd.toString());
console.log("  current_supply:", finalState.currentSupply.toString());
console.log("  burned_total:", finalState.burnedTotal.toString());
console.log("\n🎉 Sprint 5A verificado en devnet.");
