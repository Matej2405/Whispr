#!/usr/bin/env node
import {
  Connection,
  Keypair,
  Transaction,
  TransactionInstruction,
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import { createHash } from 'crypto';
import fs from 'fs';
import path from 'path';

const DEVNET_URL = 'https://api.devnet.solana.com';
const MEMO_PROGRAM_ID = new PublicKey('MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr');
const KEYPAIR_PATH = './.whispr-keypair.json';

/**
 * Load or create a persistent keypair
 */
async function getOrCreateKeypair() {
  if (fs.existsSync(KEYPAIR_PATH)) {
    const secretKey = JSON.parse(fs.readFileSync(KEYPAIR_PATH, 'utf-8'));
    return Keypair.fromSecretKey(Uint8Array.from(secretKey));
  }

  const keypair = Keypair.generate();
  fs.writeFileSync(KEYPAIR_PATH, JSON.stringify(Array.from(keypair.secretKey)));
  console.error(`[INFO] Created new keypair: ${keypair.publicKey.toBase58()}`);
  return keypair;
}

/**
 * Request an airdrop if balance is low
 */
async function ensureBalance(connection, keypair, minBalance = 0.1) {
  const balance = await connection.getBalance(keypair.publicKey);
  const balanceSol = balance / LAMPORTS_PER_SOL;

  if (balanceSol < minBalance) {
    console.error(`[INFO] Balance low (${balanceSol.toFixed(4)} SOL), requesting airdrop...`);
    try {
      const signature = await connection.requestAirdrop(
        keypair.publicKey,
        1 * LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(signature, 'confirmed');
      console.error(`[INFO] Airdrop confirmed: ${signature}`);
    } catch (err) {
      console.error(`[WARN] Airdrop failed (rate limit?): ${err.message}`);
    }
  }
}

/**
 * Post a memo transaction with SHA-256 hash
 */
async function postMemo(memoText) {
  const connection = new Connection(DEVNET_URL, 'confirmed');
  const keypair = await getOrCreateKeypair();

  await ensureBalance(connection, keypair);

  // Create SHA-256 hash of the memo
  const hash = createHash('sha256').update(memoText).digest('hex');
  const memoWithHash = `whispr:${hash.substring(0, 16)}`;

  console.error(`[INFO] Posting memo: "${memoWithHash}"`);

  const instruction = new TransactionInstruction({
    keys: [{ pubkey: keypair.publicKey, isSigner: true, isWritable: true }],
    programId: MEMO_PROGRAM_ID,
    data: Buffer.from(memoWithHash, 'utf-8'),
  });

  const transaction = new Transaction().add(instruction);
  transaction.feePayer = keypair.publicKey;

  const signature = await sendAndConfirmTransaction(connection, transaction, [keypair], {
    commitment: 'confirmed',
  });

  const explorerUrl = `https://explorer.solana.com/tx/${signature}?cluster=devnet`;

  // Output JSON result to stdout for Rust to parse
  console.log(
    JSON.stringify({
      success: true,
      signature,
      explorerUrl,
      memo: memoWithHash,
      hash,
      pubkey: keypair.publicKey.toBase58(),
    })
  );
}

// CLI entry
const memoArg = process.argv[2];
if (!memoArg) {
  console.error('Usage: node postMemo.js <memo-text>');
  process.exit(1);
}

postMemo(memoArg).catch((err) => {
  console.error(`[ERROR] ${err.message}`);
  console.log(JSON.stringify({ success: false, error: err.message }));
  process.exit(1);
});

