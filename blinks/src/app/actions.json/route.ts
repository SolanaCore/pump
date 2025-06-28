import { ACTIONS_CORS_HEADERS, type ActionsJson } from "@solana/actions"

export const GET = async () => {
  const payload: ActionsJson = {
    rules: [
      // Map all action routes
    //   {
    //     pathPattern: "/api/actions/buy",
    //     apiPath: "/api/actions/buy",
    //   },
    //   {
    //     pathPattern: "/api/actions/sell",
    //     apiPath: "/api/actions/sell",
    //   },
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
