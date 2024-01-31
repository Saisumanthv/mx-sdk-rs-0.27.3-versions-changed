use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/moax-dct-swap.wasm",
        moax_dct_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/unwrap_moax.scen.json", world());
}

#[test]
fn wrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/wrap_moax.scen.json", world());
}
