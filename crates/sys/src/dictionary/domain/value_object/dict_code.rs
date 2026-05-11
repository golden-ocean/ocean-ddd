use serde::{Deserialize, Serialize};

use crate::dictionary::domain::error::DictDomianError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DictCode(String);

impl DictCode {
    pub fn new(code: impl Into<String>) -> Result<Self, DictDomianError> {
        let code = code.into();
        if code.is_empty() {
            return Err(DictDomianError::CodeRequired);
        }
        if code.chars().count() > 64 {
            return Err(DictDomianError::CodeLengthInvalid);
        }
        Ok(Self(code))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
