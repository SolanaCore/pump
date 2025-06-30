// test/constants.ts
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { AnchorProvider, BN, Program, setProvider, utils, workspace } from "@coral-xyz/anchor";
import {
  ASSOCIATED_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@coral-xyz/anchor/dist/cjs/utils/token.js";
import type {Pump} from "../target/types/pump.js";

export const provider = AnchorProvider.env();
setProvider(provider);
export const program = workspace.pump as Program<Pump>;
export const payer = provider.wallet.payer;
export const payerPublicKey = provider.wallet.publicKey
export const mint = Keypair.generate();
export const solReserve = new BN(LAMPORTS_PER_SOL*30); // 30 SOL in LAMPORT
export const tokenReserve = new BN(800_000_000_000_000); // 1 billion token
export const name = "OPOS";
export const symbol = "OPOS";
export const uri =
  "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/Climate/metadata.json";

export const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")


export const SYSTEM_PROGRAM = SystemProgram.programId;
export const RENT_SYSVAR = SYSVAR_RENT_PUBKEY;
export const TOKEN_PROGRAM = TOKEN_PROGRAM_ID;
export const ASSOCIATED_TOKEN_PROGRAM = ASSOCIATED_PROGRAM_ID;

export async function findGlobalConfigPda() {
  const [globalConfigPda, _] = await PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode("global_config"))],
            program.programId
 
  );
  return globalConfigPda;
}

export async function findBondingCurvePda() {
  const [findBondingCurvePda, _] =  PublicKey.findProgramAddressSync(
    [Buffer.from(utils.bytes.utf8.encode("BONDING_CURVE")), mint.publicKey.toBuffer()],
    program.programId
 
  );
  return findBondingCurvePda;
}

export function getMetadataPda(mint: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
}

