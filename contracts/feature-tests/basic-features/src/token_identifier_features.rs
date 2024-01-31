dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_moax(&self) -> TokenIdentifier {
        TokenIdentifier::moax()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, bytes: &[u8]) -> bool {
        TokenIdentifier::from(bytes).is_valid_dct_identifier()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dct_identifier()
    }
}
