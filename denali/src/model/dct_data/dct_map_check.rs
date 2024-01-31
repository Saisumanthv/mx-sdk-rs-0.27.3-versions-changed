use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    serde_raw::CheckDctMapRaw,
};

use super::CheckDctMapContents;

#[derive(Debug)]
pub enum CheckDctMap {
    Unspecified,
    Star,
    Equal(CheckDctMapContents),
}

impl InterpretableFrom<CheckDctMapRaw> for CheckDctMap {
    fn interpret_from(from: CheckDctMapRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckDctMapRaw::Unspecified => CheckDctMap::Unspecified,
            CheckDctMapRaw::Star => CheckDctMap::Star,
            CheckDctMapRaw::Equal(m) => {
                CheckDctMap::Equal(CheckDctMapContents::interpret_from(m, context))
            },
        }
    }
}

impl CheckDctMap {
    pub fn is_star(&self) -> bool {
        matches!(self, CheckDctMap::Star)
    }
}
