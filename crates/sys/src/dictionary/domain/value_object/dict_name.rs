use serde::{Deserialize, Serialize};

use crate::dictionary::domain::error::DictDomianError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DictName(String);

impl DictName {
    pub fn new(name: impl Into<String>) -> Result<Self, DictDomianError> {
        let name = name.into();
        if name.is_empty() {
            return Err(DictDomianError::NameRequired);
        }
        if name.chars().count() > 64 {
            return Err(DictDomianError::NameLengthInvalid);
        }
        Ok(Self(name))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
