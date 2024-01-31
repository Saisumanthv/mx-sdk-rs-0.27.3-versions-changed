use dharitri_wasm::types::Address;
use num_bigint::BigUint;

use crate::{tx_mock::TxPanic, world_mock::DctInstanceMetadata};

use super::TxCache;

impl TxCache {
    pub fn subtract_moax_balance(&self, address: &Address, call_value: &BigUint) {
        self.with_account_mut(address, |account| {
            assert!(
                &account.moax_balance >= call_value,
                "failed transfer (insufficient funds)"
            );
            account.moax_balance -= call_value;
        })
    }

    pub fn subtract_tx_gas(&self, address: &Address, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.moax_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.moax_balance -= &gas_cost;
        });
    }

    pub fn increase_moax_balance(&self, address: &Address, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.moax_balance += amount;
        });
    }

    #[allow(clippy::redundant_closure)] // clippy is wrong here, `.unwrap_or_else(panic_insufficient_funds)` won't compile
    pub fn subtract_dct_balance(
        &self,
        address: &Address,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> DctInstanceMetadata {
        self.with_account_mut(address, |account| {
            let dct_data_map = &mut account.dct;
            let dct_data = dct_data_map
                .get_mut_by_identifier(dct_token_identifier)
                .unwrap_or_else(|| panic_insufficient_funds());

            let dct_instances = &mut dct_data.instances;
            let dct_instance = dct_instances
                .get_mut_by_nonce(nonce)
                .unwrap_or_else(|| panic_insufficient_funds());
            let dct_balance = &mut dct_instance.balance;
            if &*dct_balance < value {
                panic_insufficient_funds();
            }

            *dct_balance -= value;

            dct_instance.metadata.clone()
        })
    }

    pub fn increase_dct_balance(
        &self,
        address: &Address,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
        dct_metadata: DctInstanceMetadata,
    ) {
        self.with_account_mut(address, |account| {
            account.dct.increase_balance(
                dct_token_identifier.to_vec(),
                nonce,
                value,
                dct_metadata,
            );
        });
    }

    pub fn transfer_dct_balance(
        &self,
        from: &Address,
        to: &Address,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) {
        let metadata = self.subtract_dct_balance(from, dct_token_identifier, nonce, value);

        self.increase_dct_balance(to, dct_token_identifier, nonce, value, metadata);
    }
}

fn panic_insufficient_funds() -> ! {
    std::panic::panic_any(TxPanic {
        status: 10,
        message: b"insufficient funds".to_vec(),
    });
}
