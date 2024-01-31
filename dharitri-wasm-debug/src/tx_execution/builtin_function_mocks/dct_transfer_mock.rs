use dharitri_wasm::{api::DCT_TRANSFER_FUNC_NAME, types::Address};
use num_bigint::BigUint;
use num_traits::Zero;

use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxInputDCT, TxLog, TxResult},
};

pub fn execute_dct_transfer(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        let err_result = TxResult::from_vm_error("DCTTransfer too few arguments".to_string());
        return (err_result, BlockchainUpdate::empty());
    }

    let token_identifier = tx_input.args[0].clone();
    let value = BigUint::from_bytes_be(tx_input.args[1].as_slice());

    let dct_values = vec![TxInputDCT {
        token_identifier: token_identifier.clone(),
        nonce: 0,
        value: value.clone(),
    }];

    let dct_transfer_log = dct_transfer_event_log(
        tx_input.from.clone(),
        tx_input.to.clone(),
        token_identifier,
        &value,
    );

    let func_name = tx_input.args.get(2).map(Vec::clone).unwrap_or_default();
    let args = if tx_input.args.len() > 2 {
        tx_input.args[3..].to_vec()
    } else {
        Vec::new()
    };

    let exec_input = TxInput {
        from: tx_input.from,
        to: tx_input.to,
        moax_value: BigUint::zero(),
        dct_values,
        func_name,
        args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
    };

    let (mut tx_result, blockchain_updates) = default_execution(exec_input, tx_cache);

    // prepends dct log
    tx_result.result_logs = [&[dct_transfer_log][..], tx_result.result_logs.as_slice()].concat();

    (tx_result, blockchain_updates)
}

pub fn dct_transfer_event_log(
    from: Address,
    to: Address,
    dct_token_identifier: Vec<u8>,
    dct_value: &BigUint,
) -> TxLog {
    let nonce_topic = Vec::<u8>::new();
    TxLog {
        address: from,
        endpoint: DCT_TRANSFER_FUNC_NAME.to_vec(),
        topics: vec![
            dct_token_identifier,
            nonce_topic,
            dct_value.to_bytes_be(),
            to.to_vec(),
        ],
        data: vec![],
    }
}
