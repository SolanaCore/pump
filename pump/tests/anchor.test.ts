import * as anchor from "@coral-xyz/anchor";
import {
  provider,
  program,
  payerPublicKey,
  payer,
  mint,
  solReserve,
  tokenReserve,
  name,
  symbol,
  uri,
  findGlobalConfigPda,
  findBondingCurvePda,
  getMetadataPda,
  TOKEN_METADATA_PROGRAM_ID,
  TOKEN_PROGRAM,
  SYSTEM_PROGRAM,
  RENT_SYSVAR,
  ASSOCIATED_TOKEN_PROGRAM,
} from "./constants.ts";
import {
  createInitializeMintInstruction,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress
} from "@solana/spl-token";
import { SystemProgram, Transaction } from "@solana/web3.js";

describe("pump - create_token", () => {
  let bondingCurve: anchor.web3.PublicKey;
  let bondingCurveAta: anchor.web3.PublicKey;
  let globalConfigPda: anchor.web3.PublicKey;
  let sol_escrow: anchor.web3.PublicKey;
  let sol_escrow_bump: number;
  let connection = provider.connection;

  before(async () => {
    globalConfigPda = await findGlobalConfigPda();
    bondingCurve = await findBondingCurvePda();
    
    console.log("bonding curve PublicKey", bondingCurve);
    bondingCurveAta = await anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: bondingCurve,
    });
    console.log(globalConfigPda, bondingCurve);

    // 🔍 Check if globalConfig account exists
    const globalStateInfo = await provider.connection.getAccountInfo(globalConfigPda);
    [sol_escrow, sol_escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode("BONDING_CURVE")), mint.publicKey.toBuffer(), bondingCurve.toBuffer()],
      program.programId
    );

    if (!globalStateInfo) {
      console.log("🛠️ Initializing GlobalConfig...");
      await program.methods
        .initGlobalConfig()
        .accounts({
          globalConfig: globalConfigPda, // ✅ Fixed: use globalConfig not globalState
          signer: provider.wallet.publicKey,
          systemProgram: SYSTEM_PROGRAM,
        })
        .signers([payer])
        .rpc();
    } else {
      console.log("✅ GlobalConfig already initialized. Skipping init.");
    }
  });

  it("successfully creates a token and bonding curve", async () => {
    const [metadataAddress] = getMetadataPda(mint.publicKey);

    const tx = await program.methods
      .createToken(solReserve, tokenReserve, name, symbol, uri)
      .accounts({
        signer: payerPublicKey,
        globalState: globalConfigPda,
        solEscrow: sol_escrow,
        bondingCurve,
        mint: mint.publicKey,
        tokenEscrow: bondingCurveAta, // ✅ Fixed: renamed from bondingCurveAta to tokenEscrow
        tokenProgram: TOKEN_PROGRAM,
        systemProgram: SYSTEM_PROGRAM,
        rent: RENT_SYSVAR,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        metadata: metadataAddress,
      })
      .signers([payer, mint])
      .rpc();

    console.log("tx sig", tx);
    const state = await program.account.bondingCurve.fetch(bondingCurve);
    console.log("Bonding curve account state:", state);
  });

  it("buys tokens", async () => {
    console.log(mint.publicKey);
    const maxSol = new anchor.BN(1_000_000);
    const tokenAta = await getAssociatedTokenAddress(mint.publicKey, payerPublicKey);
    
    // ✅ Check if token ATA exists before creating
    const tokenAtaInfo = await connection.getAccountInfo(tokenAta);
    if (!tokenAtaInfo) {
      console.log("Creating token ATA...");
      const createAssociatedTokenAccountIx = createAssociatedTokenAccountInstruction(
        payer.publicKey,
        tokenAta,
        payer.publicKey, // owner
        mint.publicKey, // mint
        TOKEN_PROGRAM, // ✅ Use regular token program, not TOKEN_2022
        ASSOCIATED_TOKEN_PROGRAM_ID
      );
      
      const recentBlockhash = await connection.getLatestBlockhash();
      const transaction2 = new Transaction({
        feePayer: payer.publicKey,
        blockhash: recentBlockhash.blockhash,
        lastValidBlockHeight: recentBlockhash.lastValidBlockHeight
      }).add(createAssociatedTokenAccountIx);
      
      await provider.sendAndConfirm(transaction2, [payer]);
    }

    const tx = await program.methods
      .buyToken(maxSol)
      .accounts({
        signer: payerPublicKey,
        solEscrow: sol_escrow,
        tokenAta,
        tokenEscrow: bondingCurveAta,
        bondingCurve,
        tokenMint: mint.publicKey,
        systemProgram: SYSTEM_PROGRAM,
        tokenProgram: TOKEN_PROGRAM,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
      })
      .signers([payer])
      .rpc();

    console.log("buy tx sig", tx);
  });

  it("sells tokens", async () => {
    const maxToken = new anchor.BN(100_000_000);
    const tokenAta = await getAssociatedTokenAddress(mint.publicKey, payerPublicKey);
    console.log(mint.publicKey);
    
    const tx = await program.methods
      .sellToken(maxToken)
      .accounts({
        signer: payerPublicKey,
        tokenAta,
        solEscrow: sol_escrow,
        tokenEscrow: bondingCurveAta,
        bondingCurve: bondingCurve,
        tokenMint: mint.publicKey,
        systemProgram: SYSTEM_PROGRAM,
        tokenProgram: TOKEN_PROGRAM,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
      })
      .signers([payer])
      .rpc();

    console.log("sell tx signature", tx);
  });
});
