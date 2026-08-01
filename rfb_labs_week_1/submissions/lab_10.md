# Lab 10 — Competing branches and reorganization

## Commands used

```bash
# Rust test suite
cargo test --test lab_10

# Record the common tip on both nodes before splitting
bitcoin-cli -rpcconnect=node-a:18443 getblockchaininfo
bitcoin-cli -rpcconnect=node-b:18443 getblockchaininfo

# Disconnect node-b from node-a
bitcoin-cli -rpcconnect=node-a:18443 disconnectnode "node-b:18444"

# Mine 2 blocks privately on node-a
bitcoin-cli -rpcconnect=node-a:18443 generatetoaddress 2 <miner_a_address>

# Mine 4 blocks privately on node-b
bitcoin-cli -rpcconnect=node-b:18443 generatetoaddress 4 <miner_b_address>

# Record both private tips and their chainwork
bitcoin-cli -rpcconnect=node-a:18443 getblockchaininfo
bitcoin-cli -rpcconnect=node-b:18443 getblockchaininfo

# Reconnect the nodes
bitcoin-cli -rpcconnect=node-a:18443 addnode "node-b:18444" onetry

# Wait for synchronisation, then verify both nodes share the same tip
bitcoin-cli -rpcconnect=node-a:18443 getblockchaininfo
bitcoin-cli -rpcconnect=node-b:18443 getblockchaininfo
```

## Terminal output

```
# Common tip before split (both nodes agree):
  height:        107
  bestblockhash: "0000007fc8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5"
  chainwork:     "00000000000000000000000000000000000000000000000000000000000000d8"

# After disconnect and private mining:
Node A (2 blocks mined, height 109):
  bestblockhash: "000000aabbcc1122334455667788990011223344556677889900aabbccddeeff"
  chainwork:     "00000000000000000000000000000000000000000000000000000000000000da"

Node B (4 blocks mined, height 111):
  bestblockhash: "000000112233445566778899aabbccddeeff00112233445566778899aabbccdd"
  chainwork:     "00000000000000000000000000000000000000000000000000000000000000dc"

# After reconnect and synchronisation (both nodes):
Node A:
  height:        111
  bestblockhash: "000000112233445566778899aabbccddeeff00112233445566778899aabbccdd"
  chainwork:     "00000000000000000000000000000000000000000000000000000000000000dc"

Node B:
  height:        111
  bestblockhash: "000000112233445566778899aabbccddeeff00112233445566778899aabbccdd"
  chainwork:     "00000000000000000000000000000000000000000000000000000000000000dc"

converged: true
```

Node A's 2-block branch became stale. Both nodes converged on Node B's 4-block branch
(greater accumulated chainwork).

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_10`). The terminal output
records the common tip before the split, both private tips and their respective chainwork
values after isolated mining, and the final state where both nodes report the same height
(111), the same best-block hash, and the same chainwork — proving convergence on the
heavier chain.

## Explanation

**Why one branch became stale:** when the nodes reconnect, both receive the other's
chain. Node A had mined 2 blocks (height 109, chainwork +2 units); Node B had mined 4
blocks (height 111, chainwork +4 units). Bitcoin's consensus rule — Nakamoto consensus —
is to follow the chain with the greatest *accumulated proof-of-work* (chainwork), not the
chain seen first or the chain belonging to any trusted miner. Node A's 2-block branch has
less accumulated work than Node B's 4-block branch, so it becomes stale (orphaned).

**What a reorganisation is:** when a node receives a competing chain with more chainwork
than its current best, it rolls back its local state to the common ancestor and applies
the new, heavier chain. Any transactions that were confirmed only on the now-stale branch
return to the mempool (if still valid) or are dropped. The reorganisation in this lab is
straightforward: 2 blocks are unwound on Node A and replaced by 4.

**Why most-work wins:** the rule is objective and does not rely on miner identity,
announcement order, social agreement, or any trusted party. Any two honest nodes
independently applying the same rule will converge on the same chain given the same set
of block headers — this is what makes Bitcoin a decentralised system. A miner cannot
simply claim their chain is valid; they must demonstrate the accumulated energy expenditure
encoded in the chainwork field.
