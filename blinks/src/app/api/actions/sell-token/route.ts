import {
  Connection,
  PublicKey,
  SystemProgram,
  clusterApiUrl,
  TransactionInstruction,
  Transaction,
} from "@solana/web3.js"
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token"
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
    console.log(tokenMintParam);
    const maxSolParam = url.searchParams.get("maxSol")

    if (!tokenMintParam || !maxSolParam) {
      return new Response(JSON.stringify({ error: { message: "Missing required parameters." } }), {
        status: 400,
        headers: {
          ...ACTIONS_CORS_HEADERS,
          "X-Action-Version": "2.2.0",
          "X-Blockchain-Ids": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
        },
      })
    }

const maxSol = Number(maxSolParam.replace(/_/g, ""));
    if (maxSol <=  0) {
      return new Response(JSON.stringify({ error: { message: "Invalid SOL amount" } }), {
        status: 400,
        headers: {
          ...ACTIONS_CORS_HEADERS,
          "X-Action-Version": "2.2.0",
          "X-Blockchain-Ids": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
        },
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
        headers: {
          ...ACTIONS_CORS_HEADERS,
          "X-Action-Version": "2.2.0",
          "X-Blockchain-Ids": "solana:5eykt4UsFv8P8NJdTREpY1vzqKqZKvdp",
        },
      })
    }

    const connection = new Connection(process.env.SOLANA_RPC_URL || clusterApiUrl("devnet"), "confirmed")

    // Derive PDAs
    const [bondingCurve] = PublicKey.findProgramAddressSync(
      [Buffer.from("BONDING_CURVE"), tokenMint.toBuffer()],
      PROGRAM_ID,
    )
    
    const tokenAta = await getAssociatedTokenAddress(tokenMint, sender)
    
    const tokenEscrow = await getAssociatedTokenAddress(tokenMint, bondingCurve, true)

    const instruction = await getBuyTokenInstructionAsync(
      {
        signer: sender.toBase58(),
        tokenAta: tokenAta.toBase58(),
        tokenEscrow: tokenEscrow.toBase58(),
        bondingCurve: bondingCurve.toBase58(),
        tokenMint: tokenMint.toBase58(),
        systemProgram: SystemProgram.programId.toBase58(),
        tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID.toBase58(),
        maxSol: maxSol,
      },
      { programAddress: PROGRAM_ID.toBase58() },
    )

    const solanaInstruction = new TransactionInstruction({
      programId: new PublicKey(instruction.programAddress),
      keys: instruction.accounts.map((account) => ({
        pubkey: new PublicKey(account.address),
        isSigner: account.address === sender.toBase58(),
        isWritable: account.address === sender.toBase58() || 
                   account.address === tokenAta.toBase58() ||
                   account.address === tokenEscrow.toBase58() ||
                   account.address === bondingCurve.toBase58(),
      })),
      data: Buffer.from(instruction.data),
    })

    const transaction = new Transaction().add(solanaInstruction)
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