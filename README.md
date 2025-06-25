# PUMP
Open source alternative of pump.fun

# Problem 1: Liquidity Barrier to Entry

>In an AMM, liquidity for both tokens in the pair is required to enable trading.

For example, Alice creates a token named $ALICE and wants to enable its trading on Raydium by creating the ALICE/SOL pair. The problem is that Alice needs SOL to create the pool. Some might question whether she can just put in a small amount of both tokens in the desired ratio and expect everything to work fine.

>In an AMM, the lesser the amount of liquidity, the higher the price impact of trades.

>Therefore, the token creator needs a good amount of SOL to create a healthy pool.


## Security Tip

A common issue I’ve encountered while auditing bonding curve launchpads is that anyone can create the liquidity pool on a DEX, which the launchpad will later migrate to. For example, with Raydium, only one pool is allowed per token pair. If the liquidity migration is automatic through Anchor code, it can result in a denial-of-service (DoS) situation, as the pool is already created.

>In this case, instead of attempting to create a new pool (which will fail), you should call the add_liquidity function on the DEX. This function will add liquidity based on the current price that’s already running on the DEX.