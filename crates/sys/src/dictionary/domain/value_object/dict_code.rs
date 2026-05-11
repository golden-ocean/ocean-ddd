use crate::dictionary::domain::error::DictDomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictCode(String);

impl DictCode {
    pub fn new(code: impl Into<String>) -> Result<Self, DictDomainError> {
        let code = code.into();
        if code.is_empty() {
            return Err(DictDomainError::CodeRequired);
        }
        if code.chars().count() > 64 {
            return Err(DictDomainError::CodeLengthInvalid);
        }
        Ok(Self(code))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
