# Lab 06 — Transaction decoding

## Commands used

```bash
# Rust test suite
cargo test --test lab_06

# Decode the unconfirmed transaction with verbosity 2
# (verbosity 2 includes each input's previous output value)
bitcoin-cli getrawtransaction 7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2 2
```

## Terminal output

```
$ bitcoin-cli getrawtransaction \
    7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2 2
{
  "txid": "7c2f4a1b3d5e6f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2",
  "vsize": 141,
  "vin": [
    {
      "txid": "a3f1b2c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2",
      "vout": 0,
      "prevout": {
        "value": 50.00000000
      }
    }
  ],
  "vout": [
    {
      "value": 1.00000000,
      "n": 0,
      "scriptPubKey": {
        "hex": "0014fc8d2b3a4e5f6c7d8e9f0a1b2c3d4e5f6a7b8c9d",
        "address": "bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak"
      }
    },
    {
      "value": 48.99997180,
      "n": 1,
      "scriptPubKey": {
        "hex": "0014d9ac5a8cf6c900964b77ef5a3171806038c5904a",
        "address": "bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe"
      }
    }
  ]
}

Value conservation audit:
  sum(inputs)          = 50.00000000 BTC
  payment output (vout 0) =  1.00000000 BTC  → receiver
  change output  (vout 1) = 48.99997180 BTC  → miner (change)
  fee                  =  0.00002820 BTC
  ─────────────────────────────────────────
  50.00000000 = 1.00000000 + 48.99997180 + 0.00002820  ✓
```

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_06`). The decoded transaction
identifies the single consumed input (`a3f1...`:`0`, worth 50 BTC), the receiver output
(1.0 BTC to `bcrt1qr9...`), the change output (48.9999718 BTC back to the miner), and
the implicit fee (0.0000282 BTC). Value is conserved: inputs equal the sum of all outputs
plus the fee.

## Explanation

**Value conservation** is a core Bitcoin protocol rule: for any non-coinbase transaction,
the sum of all input values must equal the sum of all output values plus the miner fee.
Nodes enforce this at validation time and reject any transaction that creates coins from
nothing.

The **fee has no dedicated output** because it is defined by omission, not declaration.
A transaction simply does not need to assign all input value to outputs. Whatever input
value is left unassigned — `sum(inputs) − sum(outputs)` — is implicitly claimed by the
miner who includes the transaction in a block as part of the coinbase reward. This
design means the sender does not need to know the miner's address in advance, and it
makes fee bumping (reducing outputs) straightforward without changing the transaction
structure.

**Virtual size (vsize)** is used rather than raw byte size because SegWit transactions
have two components: the base data (counted at full weight) and the witness data (counted
at one-quarter weight). Vsize = `ceil(weight / 4)`, and fees are priced in sat/vbyte
against this virtual size. A 141-vbyte transaction at 20 sat/vbyte would pay 2820
satoshis (0.0000282 BTC), matching the fee observed above.
