#![no_std]
#![allow(clippy::type_complexity)]

dharitri_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting MOAX and DCT payments.
#[dharitri_wasm::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value(
        &self,
    ) -> MultiResult2<BigUint, ManagedVec<Self::Api, DctTokenPayment<Self::Api>>> {
        (
            self.call_value().moax_value(),
            self.call_value().all_dct_transfers(),
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedVec<DctTokenPayment<Self::Api>>,
    ) -> ManagedVec<DctTokenPayment<Self::Api>> {
        payments
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(&self, #[payment] payment: BigUint) -> MultiResult2<BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let (payment, _) = self.call_value().payment_token_pair();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
        self.call_value().payment_token_pair().into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_1(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<BigUint, TokenIdentifier> {
        let payment = self.call_value().dct_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiResult2<BigUint, TokenIdentifier> {
        let payment = self.call_value().dct_value();
        let token = self.call_value().token();
        (payment, token).into()
    }
}
