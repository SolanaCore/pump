import {
  Connection,
  PublicKey,
  SystemProgram,
  clusterApiUrl,
  TransactionInstruction,
  Transaction,
} from "@solana/web3.js"
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction
} from "@solana/spl-token"
import {
  ACTIONS_CORS_HEADERS,
  type ActionGetResponse,
  type ActionPostRequest,
  type ActionPostResponse,
  createPostResponse,
} from "@solana/actions"
import { getBuyTokenInstructionAsync } from "@solanacore/bonding_curve"

const PROGRAM_ID = new PublicKey("FPf834XQpnVNgFTKtihkik9Bc9c57859SdXAMNrQ554Q")

// GET request handler
export async function GET(request: Request) {
  const url = new URL(request.url)

  const payload: ActionGetResponse = {
    icon: "https://ucarecdn.com/7aa46c85-08a4-4bc7-9376-88ec48bb1f43/-/preview/880x864/-/quality/smart/-/format/auto/",
    title: "Buy Pump Token",
    description: "Buy tokens from the bonding curve with SOL",
    label: "Buy Token",
    links: {
      actions: [
        {
          type: "message",
          label: "Buy Token",
          href: `${url.origin}/api/actions/buy-token?tokenMint={tokenMint}&maxSol={maxSol}`,
          parameters: [
            {
              name: "tokenMint",
              label: "Token Mint Address",
              required: true,
            },
            {
              name: "maxSol",
              label: "Maximum SOL to spend (in lamports)",
              required: true,
            },
          ],
        },
      ],
    },
  }

  return new Response(JSON.stringify(payload), {
    headers: {
      ...ACTIONS_CORS_HEADERS,
      "X-Action-Version": "2.2.0",
      "X-Blockchain-Ids": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
    },
  })
}

// OPTIONS request handler for CORS
export async function OPTIONS(request: Request) {
  return new Response(null, {
    status: 200,
    headers: {
      ...ACTIONS_CORS_HEADERS,
      "X-Action-Version": "2.2.0",
      "X-Blockchain-Ids": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
    },
  })
}

// POST request handler
export async function POST(request: Request) {
  try {
    const body: ActionPostRequest = await request.json()
    const url = new URL(request.url)

    const tokenMintParam = url.searchParams.get("tokenMint")
    const maxSolParamRaw = url.searchParams.get("maxSol")

    if (!tokenMintParam || !maxSolParamRaw) {
      return new Response(JSON.stringify({ error: { message: "Missing required parameters." } }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    // Sanitize and validate maxSol input
    const maxSolParam = maxSolParamRaw.replace(/[^0-9.]/g, "") // remove underscores or junk
    const maxSol = Number(maxSolParam)

    if (isNaN(maxSol) || maxSol <= 0) {
      return new Response(JSON.stringify({ error: { message: "Invalid SOL amount." } }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    let sender: PublicKey
    let tokenMint: PublicKey

    try {
      sender = new PublicKey(body.account)
      tokenMint = new PublicKey(tokenMintParam)
    } catch {
      return new Response(JSON.stringify({ error: { message: "Invalid account or token mint" } }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    const connection = new Connection(process.env.SOLANA_RPC_URL || clusterApiUrl("devnet"), "confirmed")

    // Derive PDAs
    const [bondingCurve] = PublicKey.findProgramAddressSync(
      [Buffer.from("bonding_curve"), tokenMint.toBuffer()],
      PROGRAM_ID
    )
    
    const [globalConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("global_config")],
      PROGRAM_ID
    )

    // Get user's token ATA using SPL Token utility
    const tokenAta = await getAssociatedTokenAddress(
      tokenMint,
      sender,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
    
    // Get bonding curve's token ATA (token escrow)
    const bondingCurveAta = await getAssociatedTokenAddress(
      tokenMint,
      bondingCurve,
      true, // allowOwnerOffCurve = true for PDA owners
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    // Verify the mint account is owned by the SPL Token program
    const mintAccountInfo = await connection.getAccountInfo(tokenMint)
    if (!mintAccountInfo || !mintAccountInfo.owner.equals(TOKEN_PROGRAM_ID)) {
      return new Response(JSON.stringify({ 
        error: { message: `Token mint ${tokenMint.toBase58()} is not owned by SPL Token program` } 
      }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    // Check if user's token ATA exists and create if needed
    const tokenAtaInfo = await connection.getAccountInfo(tokenAta)
    const instructions = []

    if (!tokenAtaInfo) {
      console.log("Creating ATA for user:", tokenAta.toBase58())
      // Create ATA instruction using the modern function
      const createAtaInstruction = createAssociatedTokenAccountInstruction(
        sender,           // payer
        tokenAta,         // associatedToken
        sender,           // owner
        tokenMint,        // mint
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID
      )
      instructions.push(createAtaInstruction)
    }

    // Debug logging
    console.log("Account addresses:", {
      sender: sender.toBase58(),
      tokenMint: tokenMint.toBase58(),
      tokenAta: tokenAta.toBase58(),
      bondingCurve: bondingCurve.toBase58(),
      bondingCurveAta: bondingCurveAta.toBase58(),
      globalConfigPda: globalConfigPda.toBase58(),
      maxSol: maxSol
    })
    /*
    signer: payerPublicKey,
          tokenAta,
          tokenEscrow: bondingCurveAta,
          bondingCurve,
          tokenMint: mint.publicKey,
          systemProgram: SYSTEM_PROGRAM,
          tokenProgram: TOKEN_PROGRAM,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
    */ 
   const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
    const [sol_escrow, sol_escrow_bump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("BONDING_CURVE"), // ✅ no utils needed
        tokenMint.toBuffer(),
        bondingCurve.toBuffer()
      ],
      PROGRAM_ID
    );

   /**
    * signer: payerPublicKey,
          solEscrow: sol_escrow,
          tokenAta,
          tokenEscrow: bondingCurveAta,
          bondingCurve,
          tokenMint: mint.publicKey,
          systemProgram: SYSTEM_PROGRAM,
          tokenProgram: TOKEN_PROGRAM,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
    */
    const instruction = await getBuyTokenInstructionAsync(
      {
        signer: sender.toBase58(),
        solEscrow: sol_escrow,
        tokenAta: tokenAta.toBase58(),
        tokenEscrow: bondingCurveAta.toBase58(),
        bondingCurve: bondingCurve.toBase58(),
        tokenMint: tokenMint.toBase58(),
        systemProgram: SystemProgram.programId.toBase58(),
        tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID.toBase58(),
        maxSol, // make sure it's a valid primitive number here
      },
      { programAddress: PROGRAM_ID.toBase58() },
    )
    
    console.log("Generated instruction:", instruction)

    const solanaInstruction = new TransactionInstruction({
      programId: new PublicKey(instruction.programAddress),
      keys: instruction.accounts.map((account:any) => ({
        pubkey: new PublicKey(account.address),
        isSigner: account.address === sender.toBase58(),
        isWritable: [
          sender.toBase58(),
          tokenAta.toBase58(),
          bondingCurveAta.toBase58(),
          bondingCurve.toBase58(),
        ].includes(account.address),
      })),
      data: Buffer.from(instruction.data),
    })

    instructions.push(solanaInstruction)

    const transaction = new Transaction().add(...instructions)
    transaction.feePayer = sender

    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash()
    transaction.recentBlockhash = blockhash
    transaction.lastValidBlockHeight = lastValidBlockHeight

    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,
        message: `Buying tokens from ${tokenMint.toBase58()} with max ${maxSol} lamports SOL`,
      },
    })

    return new Response(JSON.stringify(payload), {
      headers: ACTIONS_CORS_HEADERS,
    })
  } catch (error) {
    console.error("Error creating buy token transaction:", error)
    return new Response(JSON.stringify({ error: { message: "Failed to create transaction" } }), {
      status: 500,
      headers: ACTIONS_CORS_HEADERS,
    })
  }
}