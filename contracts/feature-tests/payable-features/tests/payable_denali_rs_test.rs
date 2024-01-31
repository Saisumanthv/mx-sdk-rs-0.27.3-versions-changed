use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/payable-features");
    blockchain.register_contract_builder(
        "file:output/payable-features.wasm",
        payable_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn call_value_check_rs() {
    dharitri_wasm_debug::denali_rs("denali/call-value-check.scen.json", world());
}

#[test]
fn payable_multiple_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_multiple.scen.json", world());
}

#[test]
fn payable_any_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_1.scen.json", world());
}

#[test]
fn payable_any_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_2.scen.json", world());
}

#[test]
fn payable_any_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_3.scen.json", world());
}

#[test]
fn payable_any_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_4.scen.json", world());
}

#[test]
fn payable_moax_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_1.scen.json", world());
}

#[test]
fn payable_moax_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_2.scen.json", world());
}

#[test]
fn payable_moax_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_3.scen.json", world());
}

#[test]
fn payable_moax_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_4.scen.json", world());
}

#[test]
fn payable_token_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_1.scen.json", world());
}

#[test]
fn payable_token_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_2.scen.json", world());
}

#[test]
fn payable_token_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_3.scen.json", world());
}

#[test]
fn payable_token_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_4.scen.json", world());
}
