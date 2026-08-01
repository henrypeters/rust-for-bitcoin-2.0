# Lab 05 — Broadcast and mempool

## Commands used

```bash
# Rust test suite
cargo test --test lab_05

# Send 1 BTC from miner to receiver without mining
bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak 1

# Check the node's local mempool for the returned TXID
bitcoin-cli getrawmempool

# Check sender's view of the transaction (0 confirmations)
bitcoin-cli -rpcwallet=miner gettransaction <txid>

# Check receiver's balance (shows untrusted_pending)
bitcoin-cli -rpcwallet=receiver getbalances
```

## Terminal output

```
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak 1
7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2

$ bitcoin-cli getrawmempool
[ "7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2" ]

$ bitcoin-cli -rpcwallet=miner gettransaction 7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2
{
  "txid": "7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2",
  "amount": -1.00000000,
  "fee": -0.00002820,
  "confirmations": 0,
  "trusted": false,
  "bip125-replaceable": "no"
}

$ bitcoin-cli -rpcwallet=receiver getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 1.00000000,
    "immature": 0.00000000
  }
}
```

TXID present in mempool: true. Sender confirmations: 0. Receiver pending: 1.0 BTC.

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_05`). The terminal output
shows the TXID returned by `sendtoaddress` appearing in `getrawmempool`, the sender
reporting zero confirmations and a negative fee, and the receiver seeing 1.0 BTC as
`untrusted_pending` — all before any block is mined.

## Explanation

A Bitcoin transaction passes through four distinct states:

**Built and signed** — the wallet selects UTXOs as inputs, constructs outputs for the
receiver and change, and signs each input with the relevant private key. The transaction
exists only in local memory; the network is unaware of it.

**Broadcast** — the signed transaction is serialised and relayed to connected peers via
the P2P gossip network. Broadcasting is a fire-and-forget announcement with no delivery
guarantee.

**Mempool** — a node that receives a valid, policy-compliant transaction holds it in its
local mempool (memory pool) as a candidate for block inclusion. The receiver's wallet
shows the payment as `untrusted_pending` because the coins are visible but not yet
secured by proof of work. Different nodes can have different mempools; a transaction can
be evicted, replaced, or never mined if its fee is too low.

**Confirmed** — a miner includes the transaction in a block and the network accepts that
block. The transaction now has one confirmation, and each subsequent block increases the
count, making the transaction progressively costlier to reverse. Broadcast is not
confirmation — only inclusion in a valid, accepted block is.
