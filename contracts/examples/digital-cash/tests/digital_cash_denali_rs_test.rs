use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract_builder(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

// verify_ed25519 not implemented
// #[test]
// fn claim_moax_rs() {
//     dharitri_wasm_debug::denali_rs("denali/claim-moax.scen.json", world());
// }

// verify_ed25519 not implemented
// #[test]
// fn claim_dct_rs() {
//     dharitri_wasm_debug::denali_rs("denali/claim-dct.scen.json", world());
// }

#[test]
fn fund_moax_and_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/fund-moax-and-dct.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    dharitri_wasm_debug::denali_rs("denali/set-accounts.scen.json", world());
}

#[test]
fn withdraw_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw-moax.scen.json", world());
}

#[test]
fn withdraw_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/withdraw-dct.scen.json", world());
}
