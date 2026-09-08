// LUKASH — Anchor document hashes on Solana devnet
// Run in Solana Playground (client.ts) — uses pg.wallet + pg.connection
//
// Purpose: Proof of existence — SHA-256 hashes of key documents
// anchored on-chain via Memo Program v2. Immutable timestamp.
//
// Date: 2026-09-04
// Documents hashed:
//   - docs/etapa-0/LITEPAPER_v1.md
//   - docs/etapa-0/ONE_PAGER_es.md
//   - docs/etapa-0/ONE_PAGER_en.md
//   - docs/etapa-0/ONE_PAGER_pt.md

const MEMO_PROGRAM_ID = new web3.PublicKey(
  "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
);

const memo = [
  "LUKASH Proof-of-Existence 2026-09-05",
  "Litepaper_v1: D2DA01C4F5BD148A31BCB36F67F351F9AED256B2A15C74AC1281552FE5416DB1",
  "OnePager_ES:  3FF4BC2E82A90255BD937221ADB52497D8146EACFD755502DACE089A86032E40",
  "OnePager_EN:  9BCEA39980438EEB9A78749FA513A1344A68D0D926E6EB0B493333F738A93898",
  "OnePager_PT:  A07FC0FADF9EE95E98E1B7FDDB521EC02F87F4574709E87B916A6AE7E2EDA21D",
].join(" | ");

const ix = new web3.TransactionInstruction({
  keys: [
    { pubkey: pg.wallet.publicKey, isSigner: true, isWritable: false },
  ],
  programId: MEMO_PROGRAM_ID,
  data: Buffer.from(memo, "utf-8"),
});

const tx = new web3.Transaction().add(ix);

console.log("Sending memo transaction...");
console.log("Memo length:", memo.length, "bytes");

const sig = await web3.sendAndConfirmTransaction(pg.connection, tx, [
  pg.wallet.keypair,
]);

console.log("✅ Document hashes anchored on Solana devnet!");
console.log("Tx signature:", sig);
console.log("Explorer: https://explorer.solana.com/tx/" + sig + "?cluster=devnet");
console.log("\nMemo content:");
console.log(memo);
