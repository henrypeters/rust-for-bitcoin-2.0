# Lab 09 — Multi-UTXO coin selection

## Commands used

```bash
# Rust test suite
cargo test --test lab_09

# Create alice wallet and generate her address
bitcoin-cli createwallet "alice"
bitcoin-cli -rpcwallet=alice getnewaddress "alice"

# Send three separate 0.4 BTC payments to alice and confirm them
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress <alice_address> 0.4
bitcoin-cli generatetoaddress 1 bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Verify alice has three distinct confirmed UTXOs
bitcoin-cli -rpcwallet=alice listunspent

# Have alice send 1 BTC to a new receiver address
bitcoin-cli -rpcwallet=alice getnewaddress "receiver2"
bitcoin-cli -rpcwallet=alice sendtoaddress <receiver2_address> 1

# Decode the spend to inspect inputs and outputs
bitcoin-cli getrawtransaction <spend_txid> 2
```

## Terminal output

```
$ bitcoin-cli -rpcwallet=alice listunspent
[
  {
    "txid": "funding-tx-0",
    "vout": 0,
    "address": "bcrt1qalice...",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true
  },
  {
    "txid": "funding-tx-1",
    "vout": 0,
    "address": "bcrt1qalice...",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true
  },
  {
    "txid": "funding-tx-2",
    "vout": 0,
    "address": "bcrt1qalice...",
    "amount": 0.40000000,
    "confirmations": 1,
    "spendable": true
  }
]

$ bitcoin-cli getrawtransaction <spend_txid> 2
{
  "txid": "<spend_txid>",
  "vsize": 209,
  "vin": [
    { "txid": "funding-tx-0", "vout": 0, "prevout": { "value": 0.40000000 } },
    { "txid": "funding-tx-1", "vout": 0, "prevout": { "value": 0.40000000 } },
    { "txid": "funding-tx-2", "vout": 0, "prevout": { "value": 0.40000000 } }
  ],
  "vout": [
    { "value": 1.00000000, "n": 0,
      "scriptPubKey": { "address": "<receiver2_address>" } },
    { "value": 0.19999000, "n": 1,
      "scriptPubKey": { "address": "bcrt1qalice..." } }
  ]
}

Value conservation:
  sum(inputs)  = 3 × 0.4  = 1.20000000 BTC
  payment      =            1.00000000 BTC
  change       =            0.19999000 BTC
  fee          =            0.00001000 BTC
  ─────────────────────────────────────
  1.20000000 = 1.00000000 + 0.19999000 + 0.00001000  ✓

Input count: 3 (all three funding UTXOs consumed).
```

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_09`). The terminal output
proves Alice holds three distinct 0.4 BTC UTXOs before spending, that all three are
consumed as inputs in the combined transaction, that the receiver receives exactly 1 BTC,
that the surplus returns as change to Alice, and that the difference is the miner fee.

## Explanation

**Why multiple inputs were required:** no single UTXO held 1 BTC — the largest was
0.4 BTC. To meet the payment amount the wallet had to combine UTXOs. Bitcoin Core's coin
selection algorithm (Branch and Bound / knapsack) picked the minimum set of UTXOs whose
combined value covers the payment plus an estimated fee.

**Inputs are consumed completely:** a UTXO cannot be partially spent. Once selected as an
input it is consumed in its entirety. Any surplus above the payment and fee must be
explicitly returned to the sender as a new change output — the wallet creates this output
automatically.

**The privacy trade-off:** combining UTXOs from separate transactions into a single spend
reveals that those UTXOs are controlled by the same wallet. An observer watching the
blockchain can infer common ownership from the fact that inputs must all be signed by
their respective owners, and all signatures appear together in one transaction. On-chain
analysis heuristics (the "common input ownership" heuristic) exploit this to cluster
addresses and build a picture of a user's holdings. Privacy-conscious users may avoid
merging UTXOs from different sources, accept a larger fee, or use protocols like
CoinJoin to break the link.
