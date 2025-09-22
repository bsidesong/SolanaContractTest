// ppt.js - CommonJS version, no ES modules or dynamic import

const {
  Connection,
  PublicKey,
  clusterApiUrl,
  Keypair,
  Transaction,
  SystemProgram,
  sendAndConfirmTransaction,
  TransactionInstruction,
} = require("@solana/web3.js");
const borsh = require("@project-serum/borsh");
const fs = require("fs");

// === CONFIGURAÇÕES ===
const PROGRAM_ID = new PublicKey("JED8TspjbbDp9EKXPkJJw5Zsd8eySSWNygYvSGEakNev");
const NETWORK = "https://crimson-withered-aura.solana-devnet.quiknode.pro/d77410756a6a1e3b01afdb3a3d008812c6bba779/";

// === CLASSE Borsh e SCHEMA ===
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

// === FUNÇÃO QUE CRIA A INSTRUCTION ===
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

(async () => {
  const playerMove = parseInt(process.argv[2]);
  if (![0, 1, 2].includes(playerMove)) {
    console.error("Uso: node ppt.js [0|1|2]  (0=Pedra,1=Papel,2=Tesoura)");
    process.exit(1);
  }

  // Carregar keypair da carteira local (SOLANA_KEYPAIR)
  const secretKeyString = fs.readFileSync(process.env.SOLANA_KEYPAIR, "utf-8");
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const playerKeypair = Keypair.fromSecretKey(secretKey);

  const connection = new Connection(NETWORK, "confirmed");

  console.log("Carteira usada:", playerKeypair.publicKey.toBase58());
  console.log("Jogada:", playerMove);

  const instruction = createJogarInstruction(playerKeypair.publicKey, PROGRAM_ID, playerMove);
  const transaction = new Transaction().add(instruction);

  const signature = await sendAndConfirmTransaction(connection, transaction, [playerKeypair]);
  console.log("Transação confirmada:", signature);
})();
