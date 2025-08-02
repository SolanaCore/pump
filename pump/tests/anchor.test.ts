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
  import { getAssociatedTokenAddress } from "@solana/spl-token";

  describe("pump - create_token", () => {
    let bondingCurve: anchor.web3.PublicKey;
    let bondingCurveAta: anchor.web3.PublicKey;
    let globalConfigPda: anchor.web3.PublicKey;
  before(async () => {
    globalConfigPda = await findGlobalConfigPda();
    bondingCurve = await findBondingCurvePda();
    
    console.log("bonding curve", bondingCurve);
    bondingCurveAta = await anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: bondingCurve,
    });
    console.log(globalConfigPda, bondingCurve);

    // 🔍 Check if globalConfig account exists
    const globalStateInfo = await provider.connection.getAccountInfo(globalConfigPda);

    if (!globalStateInfo) {
      console.log("🛠️ Initializing GlobalConfig...");
      await program.methods
        .initGlobalConfig()
        .accounts({
          globalState: globalConfigPda,
          signer: provider.wallet.publicKey,
          systemProgram: SYSTEM_PROGRAM,
        })
        .signers([payer])
        .rpc();
    } else {
      console.log("✅ GlobalConfig already initialized. Skipping init.");
    }
  });


    it.only("successfully creates a token and bonding curve", async () => {
      const [metadataAddress] = getMetadataPda(mint.publicKey);

      const  tx = await program.methods
        .createToken(solReserve, tokenReserve, name, symbol, uri)
        .accounts({
          signer: payerPublicKey,
          globalState: globalConfigPda,
          bondingCurve,
          mint: mint.publicKey,
          bondingCurveAta,
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
      const maxSol = new anchor.BN(1_000_000_000);
      const tokenAta = await getAssociatedTokenAddress(mint.publicKey, payerPublicKey);
      const  tx = await program.methods
        .buyToken(maxSol)
        .accounts({
          signer: payerPublicKey,
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

        console.log("tx sig", tx);
    });

    it("sells tokens", async () => {
      const maxToken = new anchor.BN(25_000_000_000);
      const tokenAta = await getAssociatedTokenAddress(mint.publicKey, payerPublicKey);
      console.log(mint.publicKey);
      const tx =  await program.methods
        .sellToken(maxToken)
        .accounts({
          signer: payerPublicKey,
          tokenAta,
          tokenEscrow: bondingCurveAta,
          bondingCurve: bondingCurve,
          tokenMint: mint.publicKey,
          systemProgram: SYSTEM_PROGRAM,
          tokenProgram: TOKEN_PROGRAM,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
        })
        .signers([payer])
        .rpc();

        console.log("tx signature", tx);
    });
  });
