dharitri_wasm::imports!();

#[dharitri_wasm::module]
#[only_owner]
pub trait OnlyOwnerModule: super::only_owner_derived_mod::OnlyOwnerDerivedModule {
    #[endpoint]
    fn only_owner_mod_endpoint(&self) {}
}
