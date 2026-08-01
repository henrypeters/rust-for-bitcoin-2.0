# Lab 02 — Wallets and addresses

## Commands used

```bash
# Rust test suite
cargo test --test lab_02

# Create the two wallets
bitcoin-cli createwallet "miner"
bitcoin-cli createwallet "receiver"

# Confirm both are loaded
bitcoin-cli listwallets

# Generate addresses with labels
bitcoin-cli -rpcwallet=miner    getnewaddress "mining"
bitcoin-cli -rpcwallet=receiver getnewaddress "classmate"

# Verify each address belongs to the expected wallet
bitcoin-cli -rpcwallet=miner    getaddressinfo <miner_address>
bitcoin-cli -rpcwallet=receiver getaddressinfo <receiver_address>
```

## Terminal output

```
$ bitcoin-cli createwallet "miner"
{ "name": "miner", "warning": "" }

$ bitcoin-cli createwallet "receiver"
{ "name": "receiver", "warning": "" }

$ bitcoin-cli listwallets
[ "miner", "receiver" ]

$ bitcoin-cli -rpcwallet=miner getnewaddress "mining"
bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe

$ bitcoin-cli -rpcwallet=receiver getnewaddress "classmate"
bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak

$ bitcoin-cli -rpcwallet=miner getaddressinfo bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe
{
  "address": "bcrt1qm5xk2v8ekzqynfm3a7v5wzxcqwrpezs8dfrnhe",
  "ismine": true,
  "iswatchonly": false,
  "isscript": false,
  "iswitness": true,
  "label": "mining"
}

$ bitcoin-cli -rpcwallet=receiver getaddressinfo bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak
{
  "address": "bcrt1qr9lf3y4w7nhpzx4v5djgtz6vt3s2yfx0gxfhak",
  "ismine": true,
  "iswatchonly": false,
  "label": "classmate"
}
```

Both addresses start with `bcrt1` confirming they are native SegWit (bech32) addresses on
the regtest chain.

## Evidence references

All four automated Rust tests pass (`cargo test --test lab_02`). The terminal output
demonstrates that both wallets are loaded, both addresses carry the `bcrt1` regtest prefix,
and `getaddressinfo` returns `"ismine": true` in the correct wallet context for each address.

## Explanation

Bitcoin Core can host multiple independent wallets at the same time. When an RPC call
operates on wallet-specific data — balances, addresses, transactions — the node needs to
know *which* wallet to query. That is done by appending `-rpcwallet=<name>` to the
`bitcoin-cli` command, which causes it to target the `/wallet/<name>` RPC endpoint instead
of the default node-wide endpoint.

Without the correct wallet context the node either returns data from the wrong wallet or
returns an error because it cannot determine which wallet to use. For example, calling
`getnewaddress` without `-rpcwallet` on a node with multiple wallets loaded will fail with
"No wallet is loaded." This demonstrates that wallet scope is mandatory for any call that
touches wallet state.

The `bcrt1` prefix is the bech32 human-readable part (HRP) for regtest native SegWit
addresses. Mainnet uses `bc1`, testnet3 uses `tb1`, and regtest uses `bcrt1`, so the
prefix alone proves which network an address belongs to and prevents accidentally sending
regtest coins to a mainnet address or vice versa.
