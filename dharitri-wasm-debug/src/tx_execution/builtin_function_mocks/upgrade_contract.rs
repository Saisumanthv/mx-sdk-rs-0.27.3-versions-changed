use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

pub fn execute_upgrade_contract(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    if tx_input.args.len() < 2 {
        return (
            TxResult::from_vm_error("upgradeContract expects at least 2 arguments".to_string()),
            BlockchainUpdate::empty(),
        );
    }

    let new_code = tx_input.args[0].clone();

    // tx_input.args[1] is the code metadata, we ignore for now
    // TODO: model code metadata in Denali

    let args = if tx_input.args.len() > 2 {
        tx_input.args[2..].to_vec()
    } else {
        Vec::new()
    };

    tx_cache.with_account_mut(&tx_input.to, |account| {
        account.contract_path = Some(new_code);
    });

    let exec_input = TxInput {
        from: tx_input.from,
        to: tx_input.to,
        moax_value: tx_input.moax_value,
        dct_values: Vec::new(),
        func_name: b"init".to_vec(),
        args,
        gas_limit: tx_input.gas_limit,
        gas_price: tx_input.gas_price,
        tx_hash: tx_input.tx_hash,
    };

    default_execution(exec_input, tx_cache)
}
