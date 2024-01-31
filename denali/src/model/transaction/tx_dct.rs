use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BigUintValue, BytesValue, U64Value},
    serde_raw::{TxDCTRaw, ValueSubTree},
};

#[derive(Debug)]
pub struct TxDCT {
    pub dct_token_identifier: BytesValue,
    pub nonce: U64Value,
    pub dct_value: BigUintValue,
}

impl InterpretableFrom<TxDCTRaw> for TxDCT {
    fn interpret_from(from: TxDCTRaw, context: &InterpreterContext) -> Self {
        TxDCT {
            dct_token_identifier: interpret_dct_token_identifier(from.token_identifier, context),
            nonce: interpret_opt_u64(from.nonce, context),
            dct_value: BigUintValue::interpret_from(from.value, context),
        }
    }
}

fn interpret_dct_token_identifier(
    dct_token_identifier: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BytesValue {
    if let Some(dct_token_identifier_raw) = dct_token_identifier {
        BytesValue::interpret_from(dct_token_identifier_raw, context)
    } else {
        BytesValue::empty()
    }
}

fn interpret_opt_u64(opt_u64: Option<ValueSubTree>, context: &InterpreterContext) -> U64Value {
    if let Some(u) = opt_u64 {
        U64Value::interpret_from(u, context)
    } else {
        U64Value::empty()
    }
}
