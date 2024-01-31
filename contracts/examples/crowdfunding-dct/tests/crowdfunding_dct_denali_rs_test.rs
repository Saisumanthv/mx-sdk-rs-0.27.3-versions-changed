use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/crowdfunding-dct");

    blockchain.register_contract_builder(
        "file:output/crowdfunding-dct.wasm",
        crowdfunding_dct::ContractBuilder,
    );
    blockchain
}

#[test]
fn crowdfunding_claim_failed_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn crowdfunding_claim_successful_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-claim-successful.scen.json", world());
}

#[test]
fn crowdfunding_claim_too_early_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-claim-too-early.scen.json", world());
}

#[test]
fn crowdfunding_fund_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-fund.scen.json", world());
}

#[test]
fn crowdfunding_fund_too_late_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn crowdfunding_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/crowdfunding-init.scen.json", world());
}

#[test]
fn moax_crowdfunding_claim_failed_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-claim-failed.scen.json", world());
}

#[test]
fn moax_crowdfunding_claim_successful_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-claim-successful.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_claim_too_early_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/moax-crowdfunding-claim-too-early.scen.json",
        world(),
    );
}

#[test]
fn moax_crowdfunding_fund_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-fund.scen.json", world());
}

#[test]
fn moax_crowdfunding_fund_too_late_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-fund-too-late.scen.json", world());
}

#[test]
fn moax_crowdfunding_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/moax-crowdfunding-init.scen.json", world());
}
