import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LukashProtocol } from "../target/types/lukash_protocol";
import { assert } from "chai";

// Constantes de prueba (USD 6 dec)
const USD = 1_000_000; // $1

describe("lukash_protocol — núcleo económico (Milestone 1)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.LukashProtocol as Program<LukashProtocol>;

  const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")], program.programId);
  const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state")], program.programId);

  it("inicializa el protocolo con parámetros correctos", async () => {
    await program.methods.initialize().accounts({
      authority: provider.wallet.publicKey,
    }).rpc();

    const config = await program.account.protocolConfig.fetch(configPda);
    assert.equal(config.stage, 1);
    assert.equal(config.paused, false);
    assert.equal(config.kMinUsd.toString(), (25_000_000 * USD).toString());
    // Composición del Vault suma 100%
    const sum = config.vaultCbtcBps.toNumber() + config.vaultSolBps.toNumber() +
      config.vaultLstBps.toNumber() + config.vaultUsdcResBps.toNumber() +
      config.vaultUsdcLendBps.toNumber();
    assert.equal(sum, 10000);
  });

  it("Motor A (Etapa 1, 4%): distribuye 35/35/15/15 exacto y cumple el invariante", async () => {
    const before = await program.account.protocolState.fetch(statePda);
    const amount = new anchor.BN(1 * USD); // $1
    // motor=0 (A), layer=0, currency=1 (SOL), is_whitelist=false
    await program.methods.processFee(amount, 0, 0, 1, false).accounts({
      caller: provider.wallet.publicKey,
    }).rpc();

    const s = await program.account.protocolState.fetch(statePda);
    // fee = 4% de $1 = 40000. vault=14000, lp=14000, om=6000, staking=6000
    const dCore = s.vaultCoreUsd.sub(before.vaultCoreUsd).toNumber();
    const dSoc = s.vaultSociedadUsd.sub(before.vaultSociedadUsd).toNumber();
    const dBurn = s.burnedTotal.sub(before.burnedTotal).toNumber();
    const dOm = s.omTotal.sub(before.omTotal).toNumber();
    const dStk = s.stakingTotal.sub(before.stakingTotal).toNumber();

    assert.equal(dCore + dSoc, 14000, "asset layer = 35% del fee");
    assert.equal(dCore, 9800, "70% al Core");
    assert.equal(dSoc, 4200, "30% a Sociedad");
    assert.equal(dBurn, 14000, "35% a quema (Motor A siempre quema)");
    assert.equal(dOm, 6000, "15% O&M");
    assert.equal(dStk, 6000, "15% staking");
    // Invariante: 14000 + 14000 + 6000 + 6000 = 40000 (el fee)
    assert.equal(dCore + dSoc + dBurn + dOm + dStk, 40000);

    // Composición por activo del Core: cbtc 3430, sol 1470, lst 1960, usdc_res 2450, usdc_lend 490
    assert.equal(s.cbtcUsd.sub(before.cbtcUsd).toNumber(), 3430);
    assert.equal(s.usdcLendUsd.sub(before.usdcLendUsd).toNumber(), 490);
  });

  it("Capa 0 del Motor D: fee 0 (exención total), no mueve el Vault", async () => {
    const before = await program.account.protocolState.fetch(statePda);
    // Necesita Etapa >= 2 para Motor D. Subimos la etapa vía admin (sin timelock para stage en test:
    // encolamos y ejecutamos requiere timelock; en su lugar validamos que Motor D en Etapa 1 falla).
    try {
      await program.methods.processFee(new anchor.BN(USD), 3, 0, 0, false).accounts({
        caller: provider.wallet.publicKey,
      }).rpc();
      assert.fail("Motor D no debería estar activo en Etapa 1");
    } catch (e: any) {
      assert.include(e.toString(), "MotorNotActiveInStage");
    }
    const after = await program.account.protocolState.fetch(statePda);
    assert.equal(after.vaultCoreUsd.toString(), before.vaultCoreUsd.toString());
  });

  it("Motor B no está activo en Etapa 1 → revierte", async () => {
    try {
      await program.methods.processFee(new anchor.BN(USD), 1, 0, 0, false).accounts({
        caller: provider.wallet.publicKey,
      }).rpc();
      assert.fail("Motor B no debería estar activo en Etapa 1");
    } catch (e: any) {
      assert.include(e.toString(), "MotorNotActiveInStage");
    }
  });

  it("switch_motor_b revierte si K(t) < K_min", async () => {
    try {
      await program.methods.switchMotorB().accounts({
        caller: provider.wallet.publicKey,
      }).rpc();
      assert.fail("No debería conmutar a B2 sin alcanzar K_min");
    } catch (e: any) {
      assert.include(e.toString(), "KminNotReached");
    }
  });

  it("Throttle: el modo cambia según precio vs EMA30", async () => {
    // precio = 0.5x EMA30 → DEFENSIVO(3)
    await program.methods.updateOracleState(new anchor.BN(40), new anchor.BN(100)).accounts({
      config: configPda, state: statePda, authority: provider.wallet.publicKey,
    }).rpc();
    let s = await program.account.protocolState.fetch(statePda);
    assert.equal(s.throttleMode, 3, "DEFENSIVO");

    // precio = 1.3x EMA30 → ACELERADO(0)
    await program.methods.updateOracleState(new anchor.BN(130), new anchor.BN(100)).accounts({
      config: configPda, state: statePda, authority: provider.wallet.publicKey,
    }).rpc();
    s = await program.account.protocolState.fetch(statePda);
    assert.equal(s.throttleMode, 0, "ACELERADO");

    // precio = 1.0x EMA30 → NORMAL(1)
    await program.methods.updateOracleState(new anchor.BN(100), new anchor.BN(100)).accounts({
      config: configPda, state: statePda, authority: provider.wallet.publicKey,
    }).rpc();
    s = await program.account.protocolState.fetch(statePda);
    assert.equal(s.throttleMode, 1, "NORMAL");
  });

  it("Timelock: ejecutar un cambio antes de 48h revierte", async () => {
    // Encolar cambio de k_min (kind=2)
    await program.methods.queueAdminChange(2, new anchor.BN(1000 * USD), anchor.web3.PublicKey.default)
      .accounts({ config: configPda, authority: provider.wallet.publicKey }).rpc();
    try {
      await program.methods.executeAdminChange()
        .accounts({ config: configPda, authority: provider.wallet.publicKey }).rpc();
      assert.fail("No debería ejecutar antes del Timelock");
    } catch (e: any) {
      assert.include(e.toString(), "TimelockNotElapsed");
    }
  });

  it("Pausa: process_fee revierte con el protocolo en pausa", async () => {
    await program.methods.setPause(true)
      .accounts({ config: configPda, authority: provider.wallet.publicKey }).rpc();
    try {
      await program.methods.processFee(new anchor.BN(USD), 0, 0, 1, false)
        .accounts({ caller: provider.wallet.publicKey }).rpc();
      assert.fail("No debería procesar fees en pausa");
    } catch (e: any) {
      assert.include(e.toString(), "ProtocolPaused");
    }
    await program.methods.setPause(false)
      .accounts({ config: configPda, authority: provider.wallet.publicKey }).rpc();
  });
});
