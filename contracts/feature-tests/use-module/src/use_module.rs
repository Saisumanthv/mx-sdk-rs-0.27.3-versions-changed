#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;
mod only_owner_derived_mod;
mod only_owner_mod;

dharitri_wasm::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - DctModule
/// - GovernanceModule
/// - PauseModule
#[dharitri_wasm::contract]
pub trait UseModule:
    internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + only_owner_mod::OnlyOwnerModule
    + only_owner_derived_mod::OnlyOwnerDerivedModule
    + dharitri_wasm_module_dns::DnsModule
    + dharitri_wasm_modules::dct::DctModule
    + dharitri_wasm_module_features::FeaturesModule
    + dharitri_wasm_modules::governance::GovernanceModule
    + dharitri_wasm_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + dharitri_wasm_module_pause::PauseModule
{
    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }
}
