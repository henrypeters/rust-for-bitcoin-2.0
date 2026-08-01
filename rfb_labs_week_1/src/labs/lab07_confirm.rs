//! Lab 07 — confirm a transaction and prove block membership.

use crate::labs::lab05_mempool::get_raw_mempool;
use crate::model::ConfirmationReport;
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Mine exactly one block and return its hash.
pub fn mine_one_block<C: RpcClient>(client: &C, miner_address: &str) -> LabResult<String> {
    let raw = client.call(
        None,
        "generatetoaddress",
        &["1".to_owned(), miner_address.to_owned()],
    )?;
    let value = parse_cli_value(&raw)?;
    let array = value
        .as_array()
        .ok_or_else(|| LabError::Parse("expected array of block hashes".to_owned()))?;
    array
        .first()
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| LabError::Parse("expected at least one block hash".to_owned()))
}

/// Return true only when this node's mempool contains no transactions.
pub fn mempool_is_empty<C: RpcClient>(client: &C) -> LabResult<bool> {
    let mempool = get_raw_mempool(client)?;
    Ok(mempool.is_empty())
}

/// Return a transaction's confirmation count in the selected wallet.
pub fn transaction_confirmations<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
) -> LabResult<i64> {
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let value = parse_cli_value(&raw)?;
    value
        .get("confirmations")
        .and_then(|v| v.as_i64())
        .ok_or(LabError::MissingField("confirmations"))
}

/// Mine, locate the transaction's block, and prove that the block contains the TXID.
pub fn confirm_and_locate_transaction<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    txid: &str,
    miner_address: &str,
) -> LabResult<ConfirmationReport> {
    // 1. Mine one block.
    mine_one_block(client, miner_address)?;

    // 2. Check the mempool.
    let empty = mempool_is_empty(client)?;

    // 3. Read gettransaction for blockhash and confirmations.
    let raw = client.call(Some(wallet_name), "gettransaction", &[txid.to_owned()])?;
    let tx_value = parse_cli_value(&raw)?;
    let confirmations = tx_value
        .get("confirmations")
        .and_then(|v| v.as_i64())
        .ok_or(LabError::MissingField("confirmations"))?;
    let block_hash = tx_value
        .get("blockhash")
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or(LabError::MissingField("blockhash"))?;

    // 4. Read getblock and verify that its `tx` array contains txid.
    let block_raw = client.call(
        None,
        "getblock",
        &[block_hash.clone(), "1".to_owned()],
    )?;
    let block_value = parse_cli_value(&block_raw)?;
    let tx_array = block_value
        .get("tx")
        .and_then(|v| v.as_array())
        .ok_or(LabError::MissingField("tx"))?;
    let transaction_is_in_block = tx_array
        .iter()
        .any(|v| v.as_str() == Some(txid));

    Ok(ConfirmationReport {
        txid: txid.to_owned(),
        block_hash,
        confirmations,
        mempool_is_empty: empty,
        transaction_is_in_block,
    })
}
