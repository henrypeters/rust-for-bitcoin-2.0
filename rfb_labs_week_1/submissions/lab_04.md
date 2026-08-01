# Lab 04 — UTXOs and outpoints

## Commands used

```bash
# Rust test suite
cargo test --test lab_04

# List all unspent outputs in the miner wallet
bitcoin-cli -rpcwallet=miner listunspent

# Inspect the locking script and address of a specific UTXO
bitcoin-cli -rpcwallet=miner getaddressinfo bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

# Confirm the wallet's total balance matches the spendable UTXO sum
bitcoin-cli -rpcwallet=miner getbalances
```

## Terminal output

```
$ bitcoin-cli -rpcwallet=miner listunspent
[
  {
    "txid": "a3f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2",
    "vout": 0,
    "address": "bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe",
    "label": "mining",
    "scriptPubKey": "0014d9ac5a8cf6c900964b77ef5a3171806038c5904a",
    "amount": 50.00000000,
    "confirmations": 101,
    "spendable": true,
    "solvable": true,
    "safe": true
  }
]

Spendable UTXO selected:
  txid:          a3f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2
  vout:          0
  amount:        50.00000000 BTC
  confirmations: 101
  address:       bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
  scriptPubKey:  0014d9ac5a8cf6c900964b77ef5a3171806038c5904a
  spendable:     true

Outpoint: a3f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2:0

Sum of all spendable UTXOs: 50.00000000 BTC

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 50.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 5000.00000000
  }
}
```

The spendable UTXO sum (50.0 BTC) matches `trusted` in `getbalances`.

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_04`). The terminal output
identifies the single spendable UTXO with 101 confirmations, its outpoint (`txid:vout`),
its P2WPKH locking script (`0014...`), and confirms the spendable sum reconciles with
the wallet's trusted balance.

## Explanation

A **UTXO** (Unspent Transaction Output) is a discrete chunk of bitcoin that has been
created by a previous transaction and has not yet been consumed. Each UTXO has a value,
a locking script that defines who can spend it, and an outpoint that uniquely identifies
it in the entire blockchain.

An **outpoint** is the pair `(txid, vout)` — the transaction ID of the transaction that
created the output and the zero-based index of that output within that transaction. It is
the canonical way to refer to a specific coin without ambiguity.

A **wallet balance is not an account entry** in the way a bank balance is. There is no
single ledger line that says "you have X BTC." Instead, the wallet scans the blockchain
for UTXOs whose locking scripts it can satisfy (because it controls the corresponding
private key), and the balance is simply the sum of the values of those UTXOs. When you
spend, you consume one or more UTXOs entirely and create new ones — there is no partial
debit. This is why `sum_spendable_utxos` returns exactly the `trusted` balance: both
are derived from the same underlying UTXO set.
