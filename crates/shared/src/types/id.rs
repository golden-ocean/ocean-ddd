use serde::{Deserialize, Serialize};
// use sqlx::{
//     Postgres, Type,
//     decode::Decode,
//     encode::Encode,
//     postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
// };
use std::marker::PhantomData;
use uuid::Uuid;

/// 强类型泛型 ID 包装器
/// 运行时零成本，编译时保证绝对的类型安全
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)] // 告诉 Serde 序列化时直接输出内部的 Uuid 字符串，不要带大括号
pub struct Id<T> {
    pub value: Uuid,
    #[serde(skip)] // 忽略幽灵数据
    _marker: PhantomData<T>,
}

// 比较
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T> Eq for Id<T> {}

// 哈希
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

// Clone
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
// Copy
impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    /// 创建一个新的随机 ID (v7)
    pub fn new() -> Self {
        Self {
            value: Uuid::now_v7(),
            _marker: PhantomData,
        }
    }

    /// 从现有 Uuid 转换
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self {
            value: uuid,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self {
            value: Uuid::nil(),
            _marker: PhantomData,
        }
    }
}

// 打印特征
impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> std::str::FromStr for Id<T> {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_uuid(Uuid::parse_str(s)?))
    }
}

// ==========================================
// 核心：手动实现 SQLx 映射，绕过 #[sqlx(transparent)] 宏的限制
// 完全委托给内部的 Uuid 进行处理
// ==========================================

// impl<T> Type<Postgres> for Id<T> {
//     fn type_info() -> PgTypeInfo {
//         <Uuid as Type<Postgres>>::type_info()
//     }

//     fn compatible(ty: &PgTypeInfo) -> bool {
//         <Uuid as Type<Postgres>>::compatible(ty)
//     }
// }

// impl<'r, T> Decode<'r, Postgres> for Id<T> {
//     fn decode(
//         value: PgValueRef<'r>,
//     ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
//         let uuid = <Uuid as Decode<'r, Postgres>>::decode(value)?;
//         Ok(Self::from_uuid(uuid))
//     }
// }

// impl<'q, T> Encode<'q, Postgres> for Id<T> {
//     fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
//         <Uuid as Encode<'q, Postgres>>::encode_by_ref(&self.value, buf)
//     }
// }
