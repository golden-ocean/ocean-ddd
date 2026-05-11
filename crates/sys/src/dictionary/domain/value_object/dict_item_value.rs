use serde::{Deserialize, Serialize};

use crate::dictionary::domain::error::DictDomianError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DictItemValue(String);

impl DictItemValue {
    pub fn new(value: impl Into<String>) -> Result<Self, DictDomianError> {
        let value = value.into();
        if value.is_empty() {
            return Err(DictDomianError::ItemValueRequired);
        }
        if value.chars().count() > 64 {
            return Err(DictDomianError::ItemValueLengthInvalid);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
