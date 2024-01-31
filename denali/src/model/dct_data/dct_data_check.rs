use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::{CheckValue, U64Value},
    serde_raw::CheckDctDataRaw,
};

use super::CheckDctInstances;

#[derive(Debug, Default)]
pub struct CheckDctData {
    pub instances: CheckDctInstances,
    pub last_nonce: CheckValue<U64Value>,
    pub frozen: CheckValue<U64Value>,
}

impl InterpretableFrom<CheckDctDataRaw> for CheckDctData {
    fn interpret_from(from: CheckDctDataRaw, context: &InterpreterContext) -> Self {
        CheckDctData {
            instances: CheckDctInstances::interpret_from(from.instances, context),
            last_nonce: CheckValue::<U64Value>::interpret_from(from.last_nonce, context),
            frozen: CheckValue::<U64Value>::interpret_from(from.frozen, context),
        }
    }
}
