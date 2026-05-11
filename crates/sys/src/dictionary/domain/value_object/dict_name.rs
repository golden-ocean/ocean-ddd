use crate::dictionary::domain::error::DictDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictName(String);

impl DictName {
    pub fn new(name: impl Into<String>) -> Result<Self, DictDomainError> {
        let name = name.into();
        if name.is_empty() {
            return Err(DictDomainError::NameRequired);
        }
        if name.chars().count() > 64 {
            return Err(DictDomainError::NameLengthInvalid);
        }
        Ok(Self(name))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
