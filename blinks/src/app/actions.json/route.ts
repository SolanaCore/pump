import { ACTIONS_CORS_HEADERS, type ActionsJson } from "@solana/actions"

export const GET = async () => {
  const payload: ActionsJson = {
    rules: [
      // Map all action routes
      {
        pathPattern: "/api/actions/buy-token",
        apiPath: "/api/actions/buy-token",
      },
      {
        pathPattern: "/api/actions/sell-token",
        apiPath: "/api/actions/sell-token",
      },
      {
        pathPattern: "/api/actions/create-token",
        apiPath: "/api/actions/create-token",
      },
    ],
  }
  return new Response(JSON.stringify(payload), {
    headers: ACTIONS_CORS_HEADERS,
  })
}

// DO NOT FORGET TO INCLUDE THE `OPTIONS` HTTP METHOD
// THIS WILL ENSURE CORS WORKS FOR BLINKS
export const OPTIONS = GET
