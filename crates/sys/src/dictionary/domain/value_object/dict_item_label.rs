use crate::dictionary::domain::error::DictDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictItemLabel(String);

impl DictItemLabel {
    pub fn new(label: impl Into<String>) -> Result<Self, DictDomainError> {
        let label = label.into();
        if label.is_empty() {
            return Err(DictDomainError::ItemLabelRequired);
        }
        if label.chars().count() > 64 {
            return Err(DictDomainError::ItemLabelLengthInvalid);
        }
        Ok(Self(label))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
