use dharitri_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn world() -> BlockchainMock {
    BlockchainMock::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/external_steps/external_steps.scen.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err1.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err2.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err1.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err2.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err3.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_balance_err_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-balance.err.json", world());
}

#[test]
fn set_check_balance_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-balance.scen.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_code_err_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-code.err.json", world());
}

#[test]
fn set_check_code() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-code.scen.json", world());
}

#[test]
#[should_panic]
fn set_check_dct_err_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-dct.err.json", world());
}

#[test]
fn set_check_dct_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-dct.scen.json", world());
}

#[test]
#[should_panic]
fn set_check_nonce_err_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-nonce.err.json", world());
}

#[test]
fn set_check_nonce_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/set-check/set-check-nonce.scen.json", world());
}

#[test]
#[should_panic]
fn set_check_storage_err1_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err1.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err2_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err2.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err3_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err3.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err4_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err4.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err5_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err5.json",
        world(),
    );
}

#[test]
fn set_check_storage_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.scen.json",
        world(),
    );
}

#[test]
#[should_panic]
fn set_check_username_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.err.json",
        world(),
    );
}

#[test]
fn set_check_username_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.scen.json",
        world(),
    );
}

#[test]
fn builtin_func_dct_transfer() {
    dharitri_wasm_debug::denali_rs("tests/denali/builtin-func-dct-transfer.scen.json", world());
}

#[test]
#[should_panic]
fn dct_non_zero_balance_check_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/dct-non-zero-balance-check-err.scen.json",
        world(),
    );
}

#[test]
#[should_panic]
fn dct_zero_balance_check_err_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/dct-zero-balance-check-err.scen.json",
        world(),
    );
}

#[test]
fn multi_transfer_dct_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/multi-transfer-dct.scen.json", world());
}

#[test]
fn transfer_moax_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/transfer-moax.scen.json", world());
}

#[test]
fn transfer_dct_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/transfer-dct.scen.json", world());
}

#[test]
fn validator_reward_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/validatorReward.scen.json", world());
}
