use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 全局通用的基础状态值对象 (如：启用/禁用)
/// 内存布局对齐 PostgreSQL 的 SMALLINT (i16)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i16)]
pub enum Status {
    /// 启用
    #[default]
    Active = 1,

    /// 停用
    Inactive = 0,
}

impl Status {
    pub fn is_active(&self) -> bool {
        matches!(self, Status::Active)
    }

    pub fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }

    /// 直接返回 i16，与 sqlx/postgres 完美对接
    pub fn to_i16(&self) -> i16 {
        *self as i16
    }
}

// 手动实现 Serde 序列化：转为 JSON 里的数字 1 或 0
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i16(*self as i16)
    }
}
// 手动实现 Serde 反序列化：允许前端传数字 1 或 0
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i16::deserialize(deserializer)?;
        match value {
            0 => Ok(Status::Inactive),
            1 => Ok(Status::Active),
            _ => Err(serde::de::Error::custom(format!("无效的状态值: {}", value))),
        }
    }
}
// 允许从 Status 无缝转为 i16
impl From<Status> for i16 {
    fn from(status: Status) -> Self {
        status as i16
    }
}

// 允许从数据库读出的 i16 安全地转换为 Status
impl TryFrom<i16> for Status {
    type Error = String;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Status::Active),
            0 => Ok(Status::Inactive),
            _ => Err(format!("非法的数据库状态值: {}", value)),
        }
    }
}
