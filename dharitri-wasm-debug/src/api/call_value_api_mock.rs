use crate::{tx_mock::TxPanic, DebugApi};
use dharitri_wasm::{
    api::{CallValueApi, CallValueApiImpl, Handle},
    err_msg,
    types::{BigUint, DctTokenType, ManagedType},
};

impl DebugApi {
    fn fail_if_more_than_one_dct_transfer(&self) {
        if self.dct_num_transfers() > 1 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::TOO_MANY_DCT_TRANSFERS.to_vec(),
            });
        }
    }
}

impl CallValueApi for DebugApi {
    type CallValueApiImpl = DebugApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        DebugApi::new_from_static()
    }
}

impl CallValueApiImpl for DebugApi {
    fn check_not_payable(&self) {
        if BigUint::<DebugApi>::from_raw_handle(self.moax_value()) > 0u32 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_MOAX.to_vec(),
            });
        }
        if self.dct_num_transfers() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCT.to_vec(),
            });
        }
    }

    #[inline]
    fn moax_value(&self) -> Handle {
        self.insert_new_big_uint(self.input_ref().moax_value.clone())
    }

    #[inline]
    fn dct_value(&self) -> Handle {
        self.fail_if_more_than_one_dct_transfer();
        self.dct_value_by_index(0)
    }

    #[inline]
    fn token(&self) -> Handle {
        self.fail_if_more_than_one_dct_transfer();
        self.token_by_index(0)
    }

    #[inline]
    fn dct_token_nonce(&self) -> u64 {
        self.fail_if_more_than_one_dct_transfer();
        self.dct_token_nonce_by_index(0)
    }

    #[inline]
    fn dct_token_type(&self) -> DctTokenType {
        self.fail_if_more_than_one_dct_transfer();
        self.dct_token_type_by_index(0)
    }

    #[inline]
    fn dct_num_transfers(&self) -> usize {
        self.input_ref().dct_values.len()
    }

    #[inline]
    fn dct_value_by_index(&self, index: usize) -> Handle {
        if let Some(dct_value) = self.input_ref().dct_values.get(index) {
            self.insert_new_big_uint(dct_value.value.clone())
        } else {
            self.insert_new_big_uint_zero()
        }
    }

    #[inline]
    fn token_by_index(&self, index: usize) -> Handle {
        if let Some(dct_value) = self.input_ref().dct_values.get(index) {
            self.insert_new_managed_buffer(dct_value.token_identifier.clone())
        } else {
            self.insert_new_managed_buffer(Vec::new())
        }
    }

    #[inline]
    fn dct_token_nonce_by_index(&self, index: usize) -> u64 {
        if let Some(dct_value) = self.input_ref().dct_values.get(index) {
            dct_value.nonce
        } else {
            0
        }
    }

    #[inline]
    fn dct_token_type_by_index(&self, index: usize) -> DctTokenType {
        if self.dct_token_nonce_by_index(index) == 0 {
            DctTokenType::Fungible
        } else {
            DctTokenType::NonFungible
        }
    }
}
