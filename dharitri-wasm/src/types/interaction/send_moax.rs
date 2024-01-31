use core::marker::PhantomData;

use crate::{
    abi::{OutputAbi, TypeAbi, TypeDescriptionContainer},
    api::{CallTypeApi, SendApiImpl},
    io::EndpointResult,
    types::{BigUint, ManagedAddress, ManagedBuffer},
};
use alloc::{string::String, vec::Vec};

pub struct SendMoax<SA>
where
    SA: CallTypeApi + 'static,
{
    _phantom: PhantomData<SA>,
    pub to: ManagedAddress<SA>,
    pub amount: BigUint<SA>,
    pub data: ManagedBuffer<SA>,
}

impl<SA> SendMoax<SA>
where
    SA: CallTypeApi + 'static,
{
    pub fn new(to: ManagedAddress<SA>, amount: BigUint<SA>, data: ManagedBuffer<SA>) -> Self {
        Self {
            _phantom: PhantomData,
            to,
            amount,
            data,
        }
    }
}

impl<SA> EndpointResult for SendMoax<SA>
where
    SA: CallTypeApi + 'static,
{
    type DecodeAs = ();

    #[inline]
    fn finish<FA>(&self) {
        SA::send_api_impl().direct_moax(&self.to, &self.amount, self.data.clone());
    }
}

impl<SA> TypeAbi for SendMoax<SA>
where
    SA: CallTypeApi + 'static,
{
    fn type_name() -> String {
        "SendMoax".into()
    }

    /// No ABI output.
    fn output_abis(_: &[&'static str]) -> Vec<OutputAbi> {
        Vec::new()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}
