use core::convert::TryInto;

dharitri_wasm::imports!();

#[dharitri_wasm::module]
pub trait ManagedAddressFeatures {
    #[endpoint]
    fn maddress_from_array(&self, array: &[u8; 32]) -> ManagedAddress {
        array.into()
    }

    #[endpoint]
    fn maddress_from_managed_buffer(
        &self,
        managed_buffer: ManagedBuffer,
    ) -> SCResult<ManagedAddress> {
        managed_buffer.try_into().into()
    }
}
