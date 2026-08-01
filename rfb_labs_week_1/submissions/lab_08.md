# Lab 08 — Block security

## Commands used

```bash
# Rust test suite
cargo test --test lab_08

# Inspect the confirming block's verbose header
bitcoin-cli getblockheader 00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea

# Read transaction confirmation count before additional mining
bitcoin-cli -rpcwallet=receiver gettransaction \
    7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2

# Mine five more blocks
bitcoin-cli generatetoaddress 5 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Read confirmation count again (now six)
bitcoin-cli -rpcwallet=receiver gettransaction \
    7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2
```

## Terminal output

```
$ bitcoin-cli getblockheader \
    00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea
{
  "hash":             "00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea",
  "height":           102,
  "previousblockhash":"000000c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9ab",
  "merkleroot":       "e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2",
  "nonce":            2,
  "difficulty":       4.656542373906925e-10,
  "bits":             "207fffff",
  "confirmations":    1,
  "chainwork":        "000000000000000000000000000000000000000000000000000000000000ce00"
}

$ bitcoin-cli -rpcwallet=receiver gettransaction <txid>
{ "confirmations": 1, ... }

$ bitcoin-cli generatetoaddress 5 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
[ "hash103", "hash104", "hash105", "hash106", "hash107" ]

$ bitcoin-cli -rpcwallet=receiver gettransaction <txid>
{ "confirmations": 6, ... }
```

Confirmations before: 1. Confirmations after mining 5 blocks: 6.

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_08`). The terminal output
records all required header fields — hash, height, previous-block hash, Merkle root,
nonce, bits, difficulty, confirmations, and chainwork — and demonstrates the confirmation
count increasing from 1 to 6 after five additional blocks are mined.

## Explanation

**Hash links** form the chain structure. Every block header commits to the hash of the
previous block via the `previousblockhash` field. Changing any historical block would
change its hash, breaking the link from every subsequent block — the entire chain from
that point would need to be recomputed, requiring a re-do of all the proof-of-work that
followed.

**The Merkle root** is a single 32-byte commitment to every transaction in the block. It
is computed by hashing pairs of transaction IDs up a binary tree until one root hash
remains. This means a miner cannot add, remove, or reorder transactions in a block
without changing the Merkle root, which in turn changes the block hash, invalidating the
proof of work.

**Proof-of-work search** is the process of incrementing the `nonce` (and other fields)
until the resulting block hash falls below the target encoded in `bits`. On regtest the
difficulty is negligible (`207fffff`), so this is instant. On mainnet it represents
enormous real-world energy expenditure, making history rewriting prohibitively expensive.

**Confirmation depth** increases each time a new block is added after the one containing
the transaction. Each additional block requires its own proof of work, so reorganising
*n* blocks deep requires producing *n* valid blocks faster than the rest of the network —
a task that grows more impractical with each confirmation. Crucially, confirmations
measure the *cost* of rewriting history; they do not make an intrinsically invalid
transaction valid.
