use core::marker::PhantomData;

use crate::{
    api::{CallValueApi, CallValueApiImpl, ErrorApi, ManagedTypeApi},
    types::{BigUint, DctTokenPayment, DctTokenType, ManagedType, ManagedVec, TokenIdentifier},
};

#[derive(Default)]
pub struct CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> CallValueWrapper<A>
where
    A: CallValueApi + ErrorApi + ManagedTypeApi,
{
    pub fn new() -> Self {
        CallValueWrapper {
            _phantom: PhantomData,
        }
    }

    /// Retrieves the MOAX call value from the VM.
    /// Will return 0 in case of an DCT transfer (cannot have both MOAX and DCT transfer simultaneously).
    pub fn moax_value(&self) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().moax_value())
    }

    /// Returns all DCT transfers that accompany this SC call.
    /// Will return 0 results if nothing was transfered, or just MOAX.
    /// Fully managed underlying types, very efficient.
    pub fn all_dct_transfers(&self) -> ManagedVec<A, DctTokenPayment<A>> {
        A::call_value_api_impl().get_all_dct_transfers()
    }

    /// Retrieves the DCT call value from the VM.
    /// Will return 0 in case of an MOAX transfer (cannot have both MOAX and DCT transfer simultaneously).
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_value(&self) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().dct_value())
    }

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for MOAX,
    /// but the MOAX TokenIdentifier is serialized as `MOAX`.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn token(&self) -> TokenIdentifier<A> {
        TokenIdentifier::from_raw_handle(A::call_value_api_impl().token())
    }

    /// Returns the nonce of the received DCT token.
    /// Will return 0 in case of MOAX or fungible DCT transfer.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_token_nonce(&self) -> u64 {
        A::call_value_api_impl().dct_token_nonce()
    }

    /// Returns the DCT token type.
    /// Will return "Fungible" for MOAX.
    /// Warning, not tested with multi transfer, use `all_dct_transfers` instead!
    pub fn dct_token_type(&self) -> DctTokenType {
        A::call_value_api_impl().dct_token_type()
    }

    pub fn require_moax(&self) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().require_moax())
    }

    pub fn require_dct(&self, token: &[u8]) -> BigUint<A> {
        BigUint::from_raw_handle(A::call_value_api_impl().require_dct(token))
    }

    /// Returns both the call value (either MOAX or DCT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    /// TODO: replace with multi transfer handling everywhere
    pub fn payment_token_pair(&self) -> (BigUint<A>, TokenIdentifier<A>) {
        let (amount_handle, token_handle) = A::call_value_api_impl().payment_token_pair();
        (
            BigUint::from_raw_handle(amount_handle),
            TokenIdentifier::from_raw_handle(token_handle),
        )
    }
}
