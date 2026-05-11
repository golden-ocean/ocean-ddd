use crate::dictionary::domain::error::DictDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictItemValue(String);

impl DictItemValue {
    pub fn new(value: impl Into<String>) -> Result<Self, DictDomainError> {
        let value = value.into();
        if value.is_empty() {
            return Err(DictDomainError::ItemValueRequired);
        }
        if value.chars().count() > 64 {
            return Err(DictDomainError::ItemValueLengthInvalid);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
