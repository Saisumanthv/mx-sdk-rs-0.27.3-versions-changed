#[test]
fn claim_moax_go() {
    dharitri_wasm_debug::denali_go("denali/claim-moax.scen.json");
}

#[test]
fn claim_dct_go() {
    dharitri_wasm_debug::denali_go("denali/claim-dct.scen.json");
}

#[test]
fn fund_moax_and_dct_go() {
    dharitri_wasm_debug::denali_go("denali/fund-moax-and-dct.scen.json");
}

#[test]
fn set_accounts_go() {
    dharitri_wasm_debug::denali_go("denali/set-accounts.scen.json");
}

#[test]
fn withdraw_moax_go() {
    dharitri_wasm_debug::denali_go("denali/withdraw-moax.scen.json");
}

#[test]
fn withdraw_dct_go() {
    dharitri_wasm_debug::denali_go("denali/withdraw-dct.scen.json");
}
