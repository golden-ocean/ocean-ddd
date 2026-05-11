use serde::{Deserialize, Serialize};

use crate::dictionary::domain::error::DictDomianError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DictItemLabel(String);

impl DictItemLabel {
    pub fn new(label: impl Into<String>) -> Result<Self, DictDomianError> {
        let label = label.into();
        if label.is_empty() {
            return Err(DictDomianError::ItemLabelRequired);
        }
        if label.chars().count() > 64 {
            return Err(DictDomianError::ItemLabelLengthInvalid);
        }
        Ok(Self(label))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
