//! Lab 06 — decode a transaction and prove value conservation.

use crate::model::{DecodedInput, DecodedOutput, DecodedTransaction, OutPoint, PaymentAndChange};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Decode a transaction with enough verbosity to include every spent output's value.
pub fn decode_verbose_transaction<C: RpcClient>(
    client: &C,
    txid: &str,
) -> LabResult<DecodedTransaction> {
    let raw = client.call(
        None,
        "getrawtransaction",
        &[txid.to_owned(), "2".to_owned()],
    )?;
    let value = parse_cli_value(&raw)?;

    let decoded_txid = value
        .get("txid")
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or(LabError::MissingField("txid"))?;
    let vsize = value
        .get("vsize")
        .and_then(|v| v.as_u64())
        .ok_or(LabError::MissingField("vsize"))?;

    let vin = value
        .get("vin")
        .and_then(|v| v.as_array())
        .ok_or(LabError::MissingField("vin"))?;
    let inputs: LabResult<Vec<DecodedInput>> = vin
        .iter()
        .map(|input| {
            let prev_txid = input
                .get("txid")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("vin[].txid"))?;
            let vout = input
                .get("vout")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                .ok_or(LabError::MissingField("vin[].vout"))?;
            let previous_value = input
                .get("prevout")
                .and_then(|p| p.get("value"))
                .and_then(|v| v.as_f64())
                .ok_or(LabError::MissingField("vin[].prevout.value"))?;
            Ok(DecodedInput {
                previous_output: OutPoint {
                    txid: prev_txid,
                    vout,
                },
                previous_value,
            })
        })
        .collect();
    let inputs = inputs?;

    let vout = value
        .get("vout")
        .and_then(|v| v.as_array())
        .ok_or(LabError::MissingField("vout"))?;
    let outputs: LabResult<Vec<DecodedOutput>> = vout
        .iter()
        .map(|output| {
            let out_value = output
                .get("value")
                .and_then(|v| v.as_f64())
                .ok_or(LabError::MissingField("vout[].value"))?;
            let n = output
                .get("n")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32)
                .ok_or(LabError::MissingField("vout[].n"))?;
            let script_pub_key = output
                .get("scriptPubKey")
                .ok_or(LabError::MissingField("vout[].scriptPubKey"))?;
            let script_pub_key_hex = script_pub_key
                .get("hex")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("vout[].scriptPubKey.hex"))?;
            let address = script_pub_key
                .get("address")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);
            Ok(DecodedOutput {
                vout: n,
                value: out_value,
                address,
                script_pub_key_hex,
            })
        })
        .collect();
    let outputs = outputs?;

    Ok(DecodedTransaction {
        txid: decoded_txid,
        inputs,
        outputs,
        vsize,
    })
}

/// Return every previous output consumed by the transaction.
pub fn input_outpoints(transaction: &DecodedTransaction) -> Vec<OutPoint> {
    transaction
        .inputs
        .iter()
        .map(|input| input.previous_output.clone())
        .collect()
}

/// Identify the receiver payment and optional change output.
pub fn identify_payment_and_change(
    transaction: &DecodedTransaction,
    receiver_address: &str,
) -> LabResult<PaymentAndChange> {
    let payment = transaction
        .outputs
        .iter()
        .find(|output| output.address.as_deref() == Some(receiver_address))
        .cloned()
        .ok_or_else(|| {
            LabError::Parse(format!(
                "no output matching receiver address: {receiver_address}"
            ))
        })?;

    // The change output is the first non-receiver, non-OP_RETURN output.
    let change = transaction
        .outputs
        .iter()
        .find(|output| {
            output.address.as_deref() != Some(receiver_address)
                && !output.script_pub_key_hex.starts_with("6a") // OP_RETURN
        })
        .cloned();

    Ok(PaymentAndChange { payment, change })
}

/// Calculate `sum(inputs) - sum(outputs)`.
///
/// The result is rounded to the nearest satoshi (8 decimal places) to avoid
/// floating-point accumulation errors.
pub fn calculate_fee(transaction: &DecodedTransaction) -> LabResult<f64> {
    let input_sum: f64 = transaction
        .inputs
        .iter()
        .map(|input| input.previous_value)
        .sum();
    let output_sum: f64 = transaction
        .outputs
        .iter()
        .map(|output| output.value)
        .sum();
    let fee_raw = input_sum - output_sum;
    if fee_raw < 0.0 {
        return Err(LabError::Parse(format!(
            "impossible negative fee: {fee_raw} (inputs={input_sum}, outputs={output_sum})"
        )));
    }
    // Round to 8 decimal places (1 satoshi precision).
    let fee = (fee_raw * 1_000_000_00.0).round() / 1_000_000_00.0;
    Ok(fee)
}
