use serde::{Deserialize, Serialize};

use crate::dictionary::domain::error::DictDomianError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DictItemColor(String);

impl DictItemColor {
    /// 创建并校验颜色 (比如只允许标准的 Hex 颜色码 #RRGGBB 或 #RGB)
    pub fn new(color: impl Into<String>) -> Result<Self, DictDomianError> {
        let color_str = color.into();
        // 校验必须以 # 开头，且长度为 4 或 7
        if !color_str.starts_with('#') || (color_str.len() != 4 && color_str.len() != 7) {
            return Err(DictDomianError::ItemColorFormatInvalid);
        }
        // 进阶：你可以用 regex 库严格校验字符是否在 A-F, 0-9 之间
        Ok(Self(color_str))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
