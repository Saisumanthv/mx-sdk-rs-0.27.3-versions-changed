use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::BytesKey,
    serde_raw::CheckDctRaw,
};

use super::CheckDctData;

#[derive(Debug)]
pub enum CheckDct {
    Short(BytesKey),
    Full(CheckDctData),
}

impl InterpretableFrom<CheckDctRaw> for CheckDct {
    fn interpret_from(from: CheckDctRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctRaw::Full(m) => CheckDct::Full(CheckDctData::interpret_from(m, context)),
            CheckDctRaw::Short(v) => CheckDct::Short(BytesKey::interpret_from(v, context)),
        }
    }
}
