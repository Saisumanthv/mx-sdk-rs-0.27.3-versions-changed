use std::rc::Rc;

use dharitri_wasm::types::H256;
use denali::model::TxTransfer;

use crate::{
    sc_call::tx_dct_transfers_from_denali, tx_execution::sc_call, tx_mock::TxInput,
    world_mock::BlockchainMock,
};

pub fn execute(state: &mut Rc<BlockchainMock>, tx_transfer: &TxTransfer) {
    let tx_input = TxInput {
        from: tx_transfer.from.value.into(),
        to: tx_transfer.to.value.into(),
        moax_value: tx_transfer.moax_value.value.clone(),
        dct_values: tx_dct_transfers_from_denali(tx_transfer.dct_value.as_slice()),
        func_name: Vec::new(),
        args: Vec::new(),
        gas_limit: tx_transfer.gas_limit.value,
        gas_price: tx_transfer.gas_price.value,
        tx_hash: H256::zero(),
    };
    sc_call(tx_input, state, true).assert_ok();
}
