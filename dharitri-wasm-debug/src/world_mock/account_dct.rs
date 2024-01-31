use num_bigint::BigUint;
use num_traits::Zero;

use crate::key_hex;
use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::{self, Write},
};

use super::{DctInstanceMetadata, DctInstances, DctRoles};

#[derive(Clone, Default, Debug)]
pub struct DctData {
    pub token_identifier: Vec<u8>,
    pub instances: DctInstances,
    pub last_nonce: u64,
    pub roles: DctRoles,
    pub frozen: bool,
}

impl DctData {
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty_dct()
            && self.last_nonce == 0
            && self.roles.is_empty()
            && !self.frozen
    }

    pub fn get_roles(&self) -> Vec<Vec<u8>> {
        self.roles.get()
    }
}

#[derive(Clone, Default, Debug)]
pub struct AccountDct(HashMap<Vec<u8>, DctData>);

impl AccountDct {
    pub fn get_by_identifier(&self, identifier: &[u8]) -> Option<&DctData> {
        self.0.get(identifier)
    }

    /// Will provide a clone.
    pub fn get_roles(&self, identifier: &[u8]) -> Vec<Vec<u8>> {
        self.get_by_identifier_or_default(identifier).get_roles()
    }

    pub fn set_roles(&mut self, token_identifier: Vec<u8>, roles: Vec<Vec<u8>>) {
        let dct_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DctData {
                token_identifier,
                instances: DctInstances::new(),
                last_nonce: 0,
                roles: DctRoles::default(),
                frozen: false,
            });
        dct_data.roles = DctRoles::new(roles);
    }

    /// Will provide a clone.
    pub fn get_by_identifier_or_default(&self, identifier: &[u8]) -> DctData {
        if let Some(value) = self.0.get(identifier) {
            value.clone()
        } else {
            DctData::default()
        }
    }

    pub fn get_mut_by_identifier(&mut self, identifier: &[u8]) -> Option<&mut DctData> {
        self.0.get_mut(identifier)
    }

    pub fn new_from_raw_map(hash: HashMap<Vec<u8>, DctData>) -> Self {
        AccountDct(hash)
    }

    pub fn increase_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: DctInstanceMetadata,
    ) {
        let dct_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DctData {
                token_identifier,
                instances: DctInstances::new(),
                last_nonce: nonce,
                roles: DctRoles::default(),
                frozen: false,
            });
        dct_data.instances.increase_balance(nonce, value, metadata);
    }

    pub fn set_dct_balance(
        &mut self,
        token_identifier: Vec<u8>,
        nonce: u64,
        value: &BigUint,
        metadata: DctInstanceMetadata,
    ) {
        let dct_data = self
            .0
            .entry(token_identifier.clone())
            .or_insert_with(|| DctData {
                token_identifier,
                instances: DctInstances::new(),
                last_nonce: nonce,
                roles: DctRoles::default(),
                frozen: false,
            });
        dct_data.instances.set_balance(nonce, value, metadata);
    }

    pub fn get_dct_balance(&self, token_identifier: &[u8], nonce: u64) -> BigUint {
        if let Some(dct_data) = self.get_by_identifier(token_identifier) {
            if let Some(instance) = dct_data.instances.get_by_nonce(nonce) {
                instance.balance.clone()
            } else {
                BigUint::zero()
            }
        } else {
            BigUint::zero()
        }
    }

    pub fn iter(&self) -> Iter<Vec<u8>, DctData> {
        self.0.iter()
    }
}

impl fmt::Display for DctData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dct_buf = String::new();
        write!(
            dct_buf,
            "{{
                token_identifier: {},
                instances: [{}],
                last_nonce: {},
                roles: [{}],
                frozen: {},
            }}",
            key_hex(self.token_identifier.as_slice()),
            self.instances,
            self.last_nonce,
            self.roles,
            self.frozen
        )?;
        Ok(())
    }
}

impl fmt::Display for AccountDct {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dct_buf = String::new();
        let dct_keys: Vec<Vec<u8>> = self.clone().0.iter().map(|(k, _)| k.clone()).collect();

        for key in &dct_keys {
            let value = self.0.get(key).unwrap();
            write!(dct_buf, "\n\t\t\t{} -> {}", key_hex(key.as_slice()), value)?;
        }
        Ok(())
    }
}
