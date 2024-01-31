use std::rc::Rc;

use denali::model::{TxCall, TxDCT, TxExpect};

use crate::{
    tx_execution::sc_call_with_async_and_callback,
    tx_mock::{generate_tx_hash_dummy, TxInput, TxInputDCT},
    world_mock::BlockchainMock,
};

use super::check_tx_output;

pub fn execute(
    state: &mut Rc<BlockchainMock>,
    tx_id: &str,
    tx: &TxCall,
    expect: &Option<TxExpect>,
) {
    let tx_input = TxInput {
        from: tx.from.value.into(),
        to: tx.to.value.into(),
        moax_value: tx.moax_value.value.clone(),
        dct_values: tx_dct_transfers_from_denali(tx.dct_value.as_slice()),
        func_name: tx.function.as_bytes().to_vec(),
        args: tx
            .arguments
            .iter()
            .map(|scen_arg| scen_arg.value.clone())
            .collect(),
        gas_limit: tx.gas_limit.value,
        gas_price: tx.gas_price.value,
        tx_hash: generate_tx_hash_dummy(tx_id),
    };
    let tx_result = sc_call_with_async_and_callback(tx_input, state, true);
    if let Some(tx_expect) = expect {
        check_tx_output(tx_id, tx_expect, &tx_result);
    }
}

pub fn tx_dct_transfers_from_denali(denali_transf_dct: &[TxDCT]) -> Vec<TxInputDCT> {
    denali_transf_dct
        .iter()
        .map(tx_dct_transfer_from_denali)
        .collect()
}

pub fn tx_dct_transfer_from_denali(denali_transf_dct: &TxDCT) -> TxInputDCT {
    TxInputDCT {
        token_identifier: denali_transf_dct.dct_token_identifier.value.clone(),
        nonce: denali_transf_dct.nonce.value,
        value: denali_transf_dct.dct_value.value.clone(),
    }
}
