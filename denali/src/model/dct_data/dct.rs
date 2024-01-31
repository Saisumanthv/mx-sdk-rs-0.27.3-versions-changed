use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{BytesKey, BytesValue, U64Value},
    serde_raw::DctRaw,
};

use super::{DctObject, Instance};

#[derive(Debug)]
pub enum Dct {
    Short(BytesKey),
    Full(DctObject),
}

impl InterpretableFrom<DctRaw> for Dct {
    fn interpret_from(from: DctRaw, context: &InterpreterContext) -> Self {
        match from {
            DctRaw::Short(short_dct) => {
                Dct::Short(BytesKey::interpret_from(short_dct, context))
            },
            DctRaw::Full(full_dct) => Dct::Full(DctObject {
                token_identifier: full_dct
                    .token_identifier
                    .map(|b| BytesValue::interpret_from(b, context)),
                instances: full_dct
                    .instances
                    .into_iter()
                    .map(|instance| Instance::interpret_from(instance, context))
                    .collect(),
                last_nonce: full_dct
                    .last_nonce
                    .map(|b| U64Value::interpret_from(b, context)),
                roles: full_dct
                    .roles
                    .into_iter()
                    .map(|role| BytesValue::interpret_from(role, context))
                    .collect(),
                frozen: full_dct
                    .frozen
                    .map(|b| U64Value::interpret_from(b, context)),
            }),
        }
    }
}
