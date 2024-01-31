use core::marker::PhantomData;

use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{CallTypeApi, StorageReadApi},
    contract_base::SendWrapper,
    io::EndpointResult,
    types::{BigUint, ManagedAddress, ManagedBuffer, TokenIdentifier},
};
use alloc::{string::String, vec::Vec};

pub struct SendDct<SA>
where
    SA: CallTypeApi + StorageReadApi + 'static,
{
    _phantom: PhantomData<SA>,
    pub(super) to: ManagedAddress<SA>,
    pub(super) token_identifier: TokenIdentifier<SA>,
    pub(super) amount: BigUint<SA>,
    pub data: ManagedBuffer<SA>,
}

impl<SA> SendDct<SA>
where
    SA: CallTypeApi + StorageReadApi + 'static,
{
    pub fn new(
        to: ManagedAddress<SA>,
        token_identifier: TokenIdentifier<SA>,
        amount: BigUint<SA>,
        data: ManagedBuffer<SA>,
    ) -> Self {
        Self {
            _phantom: PhantomData,
            to,
            token_identifier,
            amount,
            data,
        }
    }
}

impl<SA> EndpointResult for SendDct<SA>
where
    SA: CallTypeApi + StorageReadApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self) {
        SendWrapper::new().transfer_dct_via_async_call(
            &self.to,
            &self.token_identifier,
            0,
            &self.amount,
            self.data.clone(),
        );
    }
}

impl<SA> TypeAbi for SendDct<SA>
where
    SA: CallTypeApi + StorageReadApi + 'static,
{
    fn type_name() -> String {
        "SendDct".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
