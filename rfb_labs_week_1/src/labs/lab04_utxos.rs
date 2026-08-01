//! Lab 04 — inspect UTXOs and outpoints.

use crate::model::{OutPoint, Utxo};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;
    let value = parse_cli_value(&raw)?;
    let array = value
        .as_array()
        .ok_or_else(|| LabError::Parse("expected array from listunspent".to_owned()))?;

    array
        .iter()
        .map(|entry| {
            let txid = entry
                .get("txid")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("txid"))?;
            let vout = entry
                .get("vout")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                .ok_or(LabError::MissingField("vout"))?;
            let address = entry
                .get("address")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);
            let script_pub_key = entry
                .get("scriptPubKey")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("scriptPubKey"))?;
            let amount = entry
                .get("amount")
                .and_then(|v| v.as_f64())
                .ok_or(LabError::MissingField("amount"))?;
            let confirmations = entry
                .get("confirmations")
                .and_then(|v| v.as_u64())
                .ok_or(LabError::MissingField("confirmations"))?;
            let spendable = entry
                .get("spendable")
                .and_then(|v| v.as_bool())
                .ok_or(LabError::MissingField("spendable"))?;

            Ok(Utxo {
                txid,
                vout,
                address,
                script_pub_key,
                amount,
                confirmations,
                spendable,
            })
        })
        .collect()
}

/// Select one spendable UTXO, preferring the one with the most confirmations.
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|utxo| utxo.spendable)
        .max_by_key(|utxo| utxo.confirmations)
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    OutPoint {
        txid: utxo.txid.clone(),
        vout: utxo.vout,
    }
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    utxos
        .iter()
        .filter(|utxo| utxo.spendable)
        .map(|utxo| utxo.amount)
        .sum()
}
