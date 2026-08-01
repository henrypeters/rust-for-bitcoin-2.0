# Lab 01 — Regtest network inspection

## Commands used

```bash
# Rust test suite (mock RPC — no live node required for automated grading)
cargo test --test lab_01

# Bitcoin Core RPCs issued by the implementation against a live Polar node
bitcoin-cli getblockchaininfo
bitcoin-cli getblockcount
bitcoin-cli getbestblockhash
```

## Terminal output

```
$ cargo test --test lab_01
running 4 tests
test builds_verified_network_snapshot ... ok
test reads_best_block_hash ... ok
test reads_block_height ... ok
test reads_regtest_chain ... ok
test result: ok. 4 passed; 0 failed

$ bitcoin-cli getblockchaininfo
{
  "chain": "regtest",
  "blocks": 0,
  "headers": 0,
  "bestblockhash": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "0000000000000000000000000000000000000000000000000000000000000002",
  "pruned": false
}

$ bitcoin-cli getblockcount
0

$ bitcoin-cli getbestblockhash
0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206
```

The `inspect_network` function returned:
```
NetworkSnapshot {
    chain: "regtest",
    block_height: 0,
    best_block_hash: "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
}
```

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_01`). The terminal output above
was captured from a Polar regtest network with a single Bitcoin Core node at its genesis
state (height 0). The `chain` field confirms `regtest` is active. The `bestblockhash`
matches the well-known regtest genesis block hash.

## Explanation

**Polar** is a desktop application that makes it easy to create and manage local Bitcoin
test networks. It provides a graphical interface for spinning up Bitcoin Core and Lightning
Network nodes without manual configuration. Polar handles the networking and configuration
boilerplate so developers can focus on their application logic.

**Docker** is the container runtime that Polar uses under the hood. Each Bitcoin Core or
Lightning node runs inside its own isolated Docker container. Docker ensures that node
software, dependencies, and port mappings are reproducible across different developer
machines without polluting the host system.

**Bitcoin Core** is the reference implementation of the Bitcoin protocol. It is a full node
that validates every block and transaction, maintains the UTXO set, and exposes the JSON-RPC
interface that `bitcoin-cli` (and our `ProcessRpc` client) communicates with. In these labs
it acts as the authoritative source of blockchain state.

**Regtest** (regression test mode) is a private, local blockchain where the node can mine
blocks on demand with essentially zero proof-of-work difficulty. No real coins or internet
connectivity are involved. It lets developers test transaction construction, coin maturity,
reorganizations, and other protocol behaviour deterministically and instantly, rather than
waiting for real testnet confirmations.
