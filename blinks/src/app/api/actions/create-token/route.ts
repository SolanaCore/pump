import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
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
import {getCreateTokenInstructionAsync } from "../../../../../../client/js/src/instructions/createToken"
const PROGRAM_ID = new PublicKey("FPf834XQpnVNgFTKtihkik9Bc9c57859SdXAMNrQ554Q")
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
const RENT_PROGRAM_ID = new PublicKey("SysvarRent111111111111111111111111111111111")


// GET request handler
export async function GET(request: Request) {
  const url = new URL(request.url)

  const payload: ActionGetResponse = {
    icon: "https://ucarecdn.com/7aa46c85-08a4-4bc7-9376-88ec48bb1f43/-/preview/880x864/-/quality/smart/-/format/auto/",
    title: "Create Pump Token",
    description: "Launch a new token with bonding curve mechanics",
    label: "Create Token",
    links: {
      actions: [
        {
          label: "Create Token",
          href: `${url.origin}/api/actions/create-token?name={name}&ticker={ticker}&uri={uri}&solReserve={solReserve}&tokenReserve={tokenReserve}`,
          parameters: [
            {
              name: "name",
              label: "Token Name",
              required: true,
            },
            {
              name: "ticker",
              label: "Token Symbol",
              required: true,
            },
            {
              name: "uri",
              label: "Metadata URI",
              required: true,
            },
            {
              name: "solReserve",
              label: "Initial SOL Reserve",
              required: false,
            },
            {
              name: "tokenReserve",
              label: "Initial Token Reserve",
              required: false,
            },
          ],
        },
      ],
    },
  }

  return new Response(JSON.stringify(payload), {
    headers: ACTIONS_CORS_HEADERS,
  })
}

// OPTIONS request handler for CORS
export async function OPTIONS(request: Request) {
  return new Response(null, {
    status: 200,
    headers: ACTIONS_CORS_HEADERS,
  })
}

// POST request handler
export async function POST(request: Request) {
  try {
    console.log(request)
    const body: ActionPostRequest = await request.json()
    const url = new URL(request.url)

    const name = url.searchParams.get("name")
    const ticker = url.searchParams.get("ticker")
    const uri = url.searchParams.get("uri")
    const solReserve = Number(url.searchParams.get("solReserve")) || 1
    const tokenReserve = Number(url.searchParams.get("tokenReserve")) || 1000000

    console.log(name, ticker, uri, solReserve, tokenReserve)

    if (!name || !ticker || !uri) {
      return new Response(JSON.stringify({ error: { message: "Missing required parameters." } }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    let sender: PublicKey
    try {
      sender = new PublicKey(body.account)
    } catch {
      return new Response(JSON.stringify({ error: { message: "Invalid account" } }), {
        status: 400,
        headers: ACTIONS_CORS_HEADERS,
      })
    }

    const connection = new Connection(process.env.SOLANA_RPC_URL || clusterApiUrl("devnet"), "confirmed")

    const mintKeypair = Keypair.generate()
    const mintPublicKey = mintKeypair.publicKey
    console.log(mintPublicKey);
    const [globalState] = PublicKey.findProgramAddressSync([Buffer.from("global_config")], PROGRAM_ID)

    const [bondingCurve] = PublicKey.findProgramAddressSync(
      [Buffer.from("BONDING_SEED"), mintPublicKey.toBuffer()],
      PROGRAM_ID,
    )

    const bondingCurveAta = await getAssociatedTokenAddress(mintPublicKey, bondingCurve, true)

    const [metadata] = PublicKey.findProgramAddressSync(
      [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mintPublicKey.toBuffer()],
      TOKEN_METADATA_PROGRAM_ID,
    )

    const instruction = await getCreateTokenInstructionAsync(
      {
        signer: sender.toBase58(),
        globalState: globalState.toBase58(),
        bondingCurve: bondingCurve.toBase58(),
        mint: mintPublicKey.toBase58(),
        bondingCurveAta: bondingCurveAta.toBase58(),
        tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
        systemProgram: SystemProgram.programId.toBase58(),
        rent: RENT_PROGRAM_ID.toBase58(),
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID.toBase58(),
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID.toBase58(),
        metadata: metadata.toBase58(),
        name,
        ticker,
        uri,
        solReserve: solReserve,
        tokenReserve: tokenReserve,
      },
      { programAddress: PROGRAM_ID.toBase58() },
    )
    

    const solanaInstruction = new TransactionInstruction({
      programId: new PublicKey(instruction.programAddress),
      keys: instruction.accounts.map((account) => ({
        pubkey: new PublicKey(account.address),
        isSigner: account.address === sender.toBase58() || account.address === mintPublicKey.toBase58(),
        isWritable: account.address === sender.toBase58() ||  account.address === metadata.toBase58() || account.address === bondingCurve.toBase58() || account.address === mintKeypair.publicKey.toBase58() || account.address === bondingCurveAta.toBase58(),
      })),
      data: Buffer.from(instruction.data),
    })
    console.log("bba",bondingCurveAta.toBase58())
    console.log("token_ata", )
    const transaction = new Transaction().add(solanaInstruction)
    transaction.feePayer = sender
    console.log(metadata.toBase58());
    const { blockhash, lastValidBlockHeight } = await connection.getLatestBlockhash()
    transaction.recentBlockhash = blockhash
    transaction.lastValidBlockHeight = lastValidBlockHeight
    console.log(bondingCurve.toBase58());

    // Add debugging
    console.log("Transaction before signing:", {
      feePayer: transaction.feePayer?.toBase58(),
      recentBlockhash: transaction.recentBlockhash,
      instructions: transaction.instructions.length,
      mintKeypair: mintKeypair.publicKey.toBase58(),
    })

    // Verify the transaction is properly constructed
    if (!transaction.feePayer || !transaction.recentBlockhash) {
      throw new Error("Transaction not properly constructed")
    }

    transaction.partialSign(mintKeypair)

    const payload: ActionPostResponse = await createPostResponse({
      fields: {
        transaction,
        message: `Creating token: ${name} (${ticker}), mint address: ${mintKeypair.publicKey.toBase58()}, bonding curve: ${bondingCurve.toBase58()}`,
      },
    })

    return new Response(JSON.stringify(payload), {
      headers: ACTIONS_CORS_HEADERS,
    })
  } catch (error) {
    console.error("Error creating token transaction:", error)
    return new Response(JSON.stringify({ error: { message: "Failed to create transaction" } }), {
      status: 500,
      headers: ACTIONS_CORS_HEADERS,
    })
  }
}
