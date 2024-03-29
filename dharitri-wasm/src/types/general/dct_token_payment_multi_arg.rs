use alloc::string::String;

use crate::{
    abi::TypeAbi,
    api::{EndpointFinishApi, ManagedTypeApi},
    types::{BigUint, ManagedVecItem},
    ArgId, ContractCallArg, DynArg, DynArgInput, DynArgOutput, EndpointResult,
};

use super::{DctTokenPayment, TokenIdentifier};

/// Thin wrapper around DctTokenPayment, which has different I/O behaviour:
/// - as input, is built from 3 arguments instead of 1: token identifier, nonce, value
/// - as output, it becomes 3 results instead of 1: token identifier, nonce, value
#[derive(Clone, PartialEq, Debug)]
pub struct DctTokenPaymentMultiArg<M: ManagedTypeApi> {
    obj: DctTokenPayment<M>,
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for DctTokenPaymentMultiArg<M> {
    #[inline]
    fn from(obj: DctTokenPayment<M>) -> Self {
        DctTokenPaymentMultiArg { obj }
    }
}

impl<M: ManagedTypeApi> DctTokenPaymentMultiArg<M> {
    pub fn into_dct_token_payment(self) -> DctTokenPayment<M> {
        self.obj
    }
}

impl<M: ManagedTypeApi> ManagedVecItem for DctTokenPaymentMultiArg<M> {
    const PAYLOAD_SIZE: usize = DctTokenPayment::<M>::PAYLOAD_SIZE;
    const SKIPS_RESERIALIZATION: bool = DctTokenPayment::<M>::SKIPS_RESERIALIZATION;
    type Ref<'a> = Self;

    #[inline]
    fn from_byte_reader<Reader: FnMut(&mut [u8])>(reader: Reader) -> Self {
        DctTokenPayment::from_byte_reader(reader).into()
    }

    #[inline]
    unsafe fn from_byte_reader_as_borrow<'a, Reader: FnMut(&mut [u8])>(
        reader: Reader,
    ) -> Self::Ref<'a> {
        Self::from_byte_reader(reader)
    }

    #[inline]
    fn to_byte_writer<R, Writer: FnMut(&[u8]) -> R>(&self, writer: Writer) -> R {
        self.obj.to_byte_writer(writer)
    }
}

impl<M> DynArg for DctTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn dyn_load<I: DynArgInput>(loader: &mut I, arg_id: ArgId) -> Self {
        let token_identifier = TokenIdentifier::dyn_load(loader, arg_id);
        let token_nonce = u64::dyn_load(loader, arg_id);
        let amount = BigUint::dyn_load(loader, arg_id);
        DctTokenPayment::new(token_identifier, token_nonce, amount).into()
    }
}

impl<M> EndpointResult for DctTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    type DecodeAs = DctTokenPaymentMultiArg<M>;

    #[inline]
    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi,
    {
        self.obj.token_identifier.finish::<FA>();
        self.obj.token_nonce.finish::<FA>();
        self.obj.amount.finish::<FA>();
    }
}

impl<M> ContractCallArg for &DctTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        self.obj.token_identifier.push_dyn_arg(output);
        self.obj.token_nonce.push_dyn_arg(output);
        self.obj.amount.push_dyn_arg(output);
    }
}

impl<M> ContractCallArg for DctTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn push_dyn_arg<O: DynArgOutput>(&self, output: &mut O) {
        ContractCallArg::push_dyn_arg(&self, output)
    }
}

impl<M> TypeAbi for DctTokenPaymentMultiArg<M>
where
    M: ManagedTypeApi,
{
    fn type_name() -> String {
        crate::types::MultiArg3::<TokenIdentifier<M>, u64, BigUint<M>>::type_name()
    }

    fn is_multi_arg_or_result() -> bool {
        true
    }
}
