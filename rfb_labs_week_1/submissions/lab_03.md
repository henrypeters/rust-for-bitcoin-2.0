# Lab 03 — Coinbase maturity

## Commands used

```bash
# Rust test suite
cargo test --test lab_03

# Mine exactly one block to the miner address
bitcoin-cli generatetoaddress 1 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Inspect height and balances immediately after the first block
bitcoin-cli getblockcount
bitcoin-cli -rpcwallet=miner getbalances

# Attempt a premature spend (expected to fail)
bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak 1

# Mine 100 more blocks to mature the first coinbase
bitcoin-cli generatetoaddress 100 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Confirm height and final balances
bitcoin-cli getblockcount
bitcoin-cli -rpcwallet=miner getbalances
```

## Terminal output

```
$ bitcoin-cli generatetoaddress 1 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
[ "65a7d8f3b2e1c4a09f5b2d3e1a7c6f8b9d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6" ]

$ bitcoin-cli getblockcount
1

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 50.00000000
  }
}

$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak 1
error code: -6
error message:
Insufficient funds

$ bitcoin-cli generatetoaddress 100 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
[ "...100 block hashes..." ]

$ bitcoin-cli getblockcount
101

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 50.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 5000.00000000
  }
}
```

At height 1 the reward is entirely `immature` and the spend is refused with
"Insufficient funds". At height 101 the first reward moves to `trusted` (50 BTC)
while the 100 subsequent coinbases remain `immature` (100 × 50 = 5000 BTC).

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_03`). The terminal output
shows the progression from height 1 (reward immature, spend refused) to height 101
(first reward trusted, later rewards still immature). The `"Insufficient funds"` error
text is preserved in the `premature_spend_error` field of `CoinbaseMaturityReport`.

## Explanation

The `COINBASE_MATURITY = 100` rule requires that a coinbase output cannot be spent
until the block containing it has at least 100 more blocks built on top of it.

The rationale is reorganization safety. If a miner spent a fresh coinbase reward
immediately and a chain reorganization then removed that block, the spending transaction
would reference a UTXO that ceased to exist, invalidating every downstream transaction
with it. Requiring 100 confirmations makes this risk negligible on mainnet.

The convention of mining **101 blocks** on a fresh chain derives from the rule itself:
block 1 creates the first coinbase. That coinbase reaches maturity once 100 more blocks
exist — i.e., when the chain is at height 101. Mining one block then 100 more is the
minimum sequence that makes exactly one coinbase spendable (`trusted: 50.0`), while all
100 subsequent coinbases remain `immature` because they each have fewer than 100
confirmations.
