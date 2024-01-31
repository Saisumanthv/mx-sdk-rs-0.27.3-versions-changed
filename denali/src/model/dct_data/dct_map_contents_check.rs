use crate::{
    interpret_trait::{InterpretableFrom, InterpreterContext},
    model::BytesKey,
    serde_raw::CheckDctMapContentsRaw,
};

use std::collections::BTreeMap;

use super::CheckDct;

#[derive(Debug)]
pub struct CheckDctMapContents {
    pub contents: BTreeMap<BytesKey, CheckDct>,
    pub other_dcts_allowed: bool,
}

impl CheckDctMapContents {
    pub fn contains_token(&self, token_identifier: &[u8]) -> bool {
        let token_id_conv = BytesKey::from(token_identifier.to_vec());
        self.contents.contains_key(&token_id_conv)
    }
}

impl InterpretableFrom<CheckDctMapContentsRaw> for CheckDctMapContents {
    fn interpret_from(from: CheckDctMapContentsRaw, context: &InterpreterContext) -> Self {
        CheckDctMapContents {
            contents: from
                .contents
                .into_iter()
                .map(|(k, v)| {
                    (
                        BytesKey::interpret_from(k, context),
                        CheckDct::interpret_from(v, context),
                    )
                })
                .collect(),
            other_dcts_allowed: from.other_dcts_allowed,
        }
    }
}
