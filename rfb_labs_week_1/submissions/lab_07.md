# Lab 07 — Confirmation and block membership

## Commands used

```bash
# Rust test suite
cargo test --test lab_07

# Mine exactly one block to confirm the pending transaction
bitcoin-cli generatetoaddress 1 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Verify the mempool is now empty
bitcoin-cli getrawmempool

# Read the transaction's confirmation count and containing block hash
bitcoin-cli -rpcwallet=receiver gettransaction \
    7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2

# Inspect the block and confirm the TXID appears in its tx list
bitcoin-cli getblock <blockhash> 1
```

## Terminal output

```
$ bitcoin-cli generatetoaddress 1 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
[ "00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea" ]

$ bitcoin-cli getrawmempool
[]

$ bitcoin-cli -rpcwallet=receiver gettransaction \
    7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2
{
  "txid": "7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2",
  "amount": 1.00000000,
  "confirmations": 1,
  "blockhash": "00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea",
  "trusted": true
}

$ bitcoin-cli getblock 00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea 1
{
  "hash": "00000041b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9ea",
  "height": 102,
  "tx": [
    "d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2",
    "7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2"
  ]
}
```

Mempool empty: true. Confirmations: 1. TXID in block tx list: true.
Receiver balance is now `trusted: 1.0 BTC`.

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_07`). The terminal output
shows the mempool clearing after the block is mined, the transaction gaining one
confirmation and a `blockhash`, and the TXID appearing as the second entry in the
block's `tx` array (after the coinbase).

## Explanation

When a block is mined and accepted by the network, the only thing that changes for the
transaction is its **position in the agreed history** — it moves from the unordered,
unconfirmed mempool into a specific slot within a specific block at a specific height.
The serialised bytes of the transaction itself are identical before and after; no field
is rewritten.

What changes from the node's perspective is how the transaction is indexed. Before
confirmation: it is looked up by TXID in the mempool. After confirmation: it is stored
as part of a block and looked up via `blockhash` + position in the `tx` array. The
wallet reflects this by promoting the receiver's balance from `untrusted_pending` to
`trusted` and setting the `blockhash` field in `gettransaction`.

Mining did not alter the transaction; it altered the transaction's *context* — it is now
part of a chain of proof-of-work commitments that would be expensive to rewrite.
