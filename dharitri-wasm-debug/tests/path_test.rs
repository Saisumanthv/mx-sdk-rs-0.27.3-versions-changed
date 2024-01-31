use dharitri_wasm_debug::*;

fn world() -> BlockchainMock {
    BlockchainMock::new()
}

#[test]
fn local_path_test() {
    dharitri_wasm_debug::denali_rs("tests/denali/path_test.scen.json", world());
}

#[test]
fn nested_path_test() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/external_steps/external_path_test.scen.json",
        world(),
    );
}
