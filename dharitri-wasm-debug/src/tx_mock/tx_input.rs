use crate::display_util::*;
use alloc::vec::Vec;
use dharitri_wasm::types::{Address, H256};
use num_bigint::BigUint;
use num_traits::Zero;
use std::fmt;

#[derive(Clone, Debug)]
pub struct TxInput {
    pub from: Address,
    pub to: Address,
    pub moax_value: BigUint,
    pub dct_values: Vec<TxInputDCT>,
    pub func_name: Vec<u8>,
    pub args: Vec<Vec<u8>>,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub tx_hash: H256,
}

impl fmt::Display for TxInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TxInput {{ func: {}, args: {:?}, call_value: {}, dct_value: {:?}, from: 0x{}, to: 0x{}\n}}", 
            String::from_utf8(self.func_name.clone()).unwrap(),
            self.args,
            self.moax_value,
            self.dct_values,
            address_hex(&self.from),
            address_hex(&self.to))
    }
}

impl TxInput {
    pub fn add_arg(&mut self, arg: Vec<u8>) {
        self.args.push(arg);
    }

    pub fn dummy() -> Self {
        TxInput {
            from: Address::zero(),
            to: Address::zero(),
            moax_value: BigUint::zero(),
            dct_values: Vec::new(),
            func_name: Vec::new(),
            args: Vec::new(),
            gas_limit: 0,
            gas_price: 0,
            tx_hash: H256::zero(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TxInputDCT {
    pub token_identifier: Vec<u8>,
    pub nonce: u64,
    pub value: BigUint,
}
