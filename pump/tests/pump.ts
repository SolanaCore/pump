import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pump } from "../target/types/pump";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Keypair,
} from "@solana/web3.js";
import {
  ASSOCIATED_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert } from "chai";

describe("pump - create_token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.pump as Program<Pump>;

  let globalConfigPda: PublicKey;

  before(async () => {
    [globalConfigPda] = await PublicKey.findProgramAddress(
      [Buffer.from("global_config")],
      program.programId
    );
  });

  it("successfully creates a token and bonding curve", async () => {
    try {
      const mint = Keypair.generate();

      const [bondingCurvePda] = await PublicKey.findProgramAddressSync(
        [Buffer.from("BONDING_SEED"), mint.publicKey.toBuffer()],
        program.programId
      );

      const bondingCurveAta = await anchor.utils.token.associatedAddress({
        mint: mint.publicKey,
        owner: bondingCurvePda,
      });
          const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

      const [metadataAddress] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
      const name = "OPOS";
      const symbol = "OPOS";
      const uri = "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/Climate/metadata.json";
      const solReserve = new anchor.BN(1_000_000); // 0.001 SOL
      const tokenReserve = new anchor.BN(1_000_000_000); // 1 token

      const tx = await program.methods
        .createToken(solReserve, tokenReserve, name, symbol, uri)
        .accounts({
          signer: provider.wallet.publicKey,
          globalState: globalConfigPda,
          bondingCurve: bondingCurvePda, 
          mint: mint.publicKey,
          bondingCurveAta,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
          tokenMetadataProgram: new PublicKey(
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
          ), // METAPLEX_ID
          metadata: metadataAddress
        })
        .signers([provider.wallet.payer, mint])
        .rpc();

      console.log("✅ createToken transaction:", tx);

      const bondingCurve = await program.account.bondingCurve.fetch(bondingCurvePda);
      console.log("Bonding curve account state:", bondingCurve);

      // Optional assertions
      // assert.ok(bondingCurve.isActive);
      // assert.ok(bondingCurve.solReserve.eq(solReserve));
      // assert.ok(bondingCurve.tokenReserve.eq(tokenReserve));
    } catch (err: any) {
      console.error("❌ create_token test failed:", err);

      // If it's an Anchor error, try to print logs
      if (err.logs) {
        console.error("Transaction logs:");
        for (const log of err.logs) {
          console.error(log);
        }
      }

      throw err; // Rethrow to let Mocha mark the test as failed
    }
  });
});
