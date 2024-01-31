#[test]
fn send_tx_repeat_without_data_go() {
    dharitri_wasm_debug::denali_go("denali/send_tx_repeat_without_data.scen.json");
}

#[test]
fn send_tx_repeat_with_data_go() {
    dharitri_wasm_debug::denali_go("denali/send_tx_repeat_with_data.scen.json");
}
