use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::BigUintValue,
    serde_raw::ValueSubTree,
};

pub fn interpret_moax_value(
    opt_legacy_value: Option<ValueSubTree>,
    opt_moax_value: Option<ValueSubTree>,
    context: &InterpreterContext,
) -> BigUintValue {
    let mut moax_value = BigUintValue::default();
    if let Some(parsed_legacy_value) = opt_legacy_value {
        moax_value = BigUintValue::interpret_from(parsed_legacy_value, context);
    }
    if let Some(parsed_moax_value) = opt_moax_value {
        moax_value = BigUintValue::interpret_from(parsed_moax_value, context);
    }
    moax_value
}
