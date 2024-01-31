#[test]
fn use_module_dns_register_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_features.scen.json");
}

#[test]
fn use_module_internal_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_internal.scen.json");
}

#[test]
fn use_module_only_owner_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_only_owner.scen.json");
}

#[test]
fn use_module_no_endpoint_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_no_endpoint.scen.json");
}

#[test]
fn use_module_pause_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_pause.scen.json");
}

// Governance module tests

#[test]
fn cancel_defeated_proposal_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_governance/cancel_defeated_proposal.scen.json");
}

#[test]
fn change_configuration_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_governance/change_configuration.scen.json");
}

#[test]
fn init_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_governance/init.scen.json");
}

#[test]
fn invalid_proposals_go() {
    dharitri_wasm_debug::denali_go("denali/use_module_governance/invalid_proposals.scen.json");
}

#[test]
fn withdraw_governance_tokens_go() {
    dharitri_wasm_debug::denali_go(
        "denali/use_module_governance/withdraw_governance_tokens.scen.json",
    );
}
