#[test]
fn auction_batch_go() {
    dharitri_wasm_debug::denali_go("denali/auction_batch.scen.json");
}

#[test]
fn auction_single_token_moax_go() {
    dharitri_wasm_debug::denali_go("denali/auction_single_token_moax.scen.json");
}

#[test]
fn bid_first_moax_go() {
    dharitri_wasm_debug::denali_go("denali/bid_first_moax.scen.json");
}

#[test]
fn bid_second_moax_go() {
    dharitri_wasm_debug::denali_go("denali/bid_second_moax.scen.json");
}

#[test]
fn bid_third_moax_go() {
    dharitri_wasm_debug::denali_go("denali/bid_third_moax.scen.json");
}

#[test]
fn end_auction_go() {
    dharitri_wasm_debug::denali_go("denali/end_auction.scen.json");
}
