import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { readFileSync } from "node:fs";
import { join } from "node:path";

// === CONFIGURAÇÕES ===
const PROGRAM_ID = new PublicKey("JED8TspjbbDp9EKXPkJJw5Zsd8eySSWNygYvSGEakNev");
const NETWORK = "https://crimson-withered-aura.solana-devnet.quiknode.pro/d77410756a6a1e3b01afdb3a3d008812c6bba779/";

// === Borsh schema e classe ===
class JogarInstruction {
  constructor(fields) {
    this.tag = 0;
    this.player_move = 0;
    if (fields) {
      this.player_move = fields.player_move;
    }
  }
}

const schema = new Map([
  [JogarInstruction, { kind: "struct", fields: [["tag", "u8"], ["player_move", "u8"]] }],
]);

// === Função para ler a keypair local do Solana CLI (~/.config/solana/id.json) ===
function loadKeypair() {
  const homeDir = process.env.HOME || process.env.USERPROFILE;
  const keypairPath = join(homeDir, ".config", "solana", "id.json");
  const secretKeyString = readFileSync(keypairPath, "utf-8");
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

// === Função que cria instruction ===
function createJogarInstruction(playerPubkey, programId, playerMove) {
  const data = new JogarInstruction({ player_move: playerMove });
  const serializedData = Buffer.from(borsh.serialize(schema, data));

  const [scorePDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("score"), playerPubkey.toBuffer()],
    programId
  );

  const keys = [
    { pubkey: playerPubkey, isSigner: true, isWritable: true },
    { pubkey: scorePDA, isSigner: false, isWritable: true },
    { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
  ];

  return new TransactionInstruction({
    keys,
    programId,
    data: serializedData,
  });
}

// === MAIN ===
(async () => {
  const playerMove = parseInt(process.argv[2]);
  if (![0, 1, 2].includes(playerMove)) {
    console.error("Uso: node ppt.js [0|1|2]  (0=Pedra,1=Papel,2=Tesoura)");
    process.exit(1);
  }

  const connection = new Connection(NETWORK, "confirmed");
  const playerKeypair = loadKeypair();

  console.log("Carteira usada:", playerKeypair.publicKey.toBase58());
  console.log("Jogada:", playerMove);

  const instruction = createJogarInstruction(playerKeypair.publicKey, PROGRAM_ID, playerMove);
  const transaction = new Transaction().add(instruction);

  const signature = await sendAndConfirmTransaction(connection, transaction, [playerKeypair]);
  console.log("Transação confirmada:", signature);
})();
