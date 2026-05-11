use serde::{Deserialize, Serialize};
use validator::Validate;

/// 统一的分页请求参数
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct PageQuery {
    /// 当前页码，默认 1
    #[serde(default = "default_page")]
    #[validate(range(min = 1, message = "页码必须大于 0"))]
    pub page: u64,

    /// 每页条数，默认 20
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 100, message = "每页条数必须在 1-100 之间"))]
    pub page_size: u64,
    // 如果需要全局统一排序
    // pub sort_by: Option<String>,
    // pub desc: Option<bool>,
}

// Serde 的默认值函数
fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

impl PageQuery {
    /// 为底层 SQL 查询提供 LIMIT 值
    pub fn limit(&self) -> i64 {
        // 强制最大只能取 100 条，防止恶意请求拖垮数据库
        self.page_size.min(100) as i64
    }

    /// 为底层 SQL 查询提供 OFFSET 值 (页码从 1 开始计算)
    pub fn offset(&self) -> i64 {
        let p = if self.page < 1 { 1 } else { self.page };
        ((p - 1) * self.page_size) as i64
    }
}

/// 统一的分页响应结果封装
#[derive(Debug, Serialize, Clone)]
pub struct PageRes<T> {
    /// 列表数据
    pub list: Vec<T>,
    /// 总记录数
    pub total: u64,
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub page_size: u64,
    /// 总页数 (根据 total 和 page_size 自动计算)
    pub total_pages: u64,
}

impl<T> PageRes<T> {
    /// 构造一个完整的包含数据的分页响应
    pub fn new(total: u64, page: u64, page_size: u64, list: Vec<T>) -> Self {
        // 计算总页数，注意防止除以 0
        let total_pages = if page_size == 0 {
            0
        } else {
            (total + page_size - 1) / page_size // 向上取整的整除算法
        };

        Self {
            list,
            total,
            page,
            page_size,
            total_pages,
        }
    }

    /// 构造一个空的分页响应 (用于查询结果为空时)
    pub fn empty(page: u64, page_size: u64) -> Self {
        Self {
            list: vec![],
            total: 0,
            page,
            page_size,
            total_pages: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_query_offset_and_limit() {
        // 正常情况
        let query = PageQuery {
            page: 2,
            page_size: 10,
        };
        assert_eq!(query.limit(), 10);
        assert_eq!(query.offset(), 10);

        // 边界情况：防呆设计，前端传了 page = 0，应视同 page = 1
        let query_zero = PageQuery {
            page: 0,
            page_size: 20,
        };
        assert_eq!(query_zero.offset(), 0);
    }

    #[test]
    fn test_page_result_total_pages_calculation() {
        // 刚好整除
        let res = PageRes::<String>::new(100, 1, 10, vec![]);
        assert_eq!(res.total_pages, 10);

        // 有余数，必须向上取整 (101 条数据，每页 10 条，应该是 11 页)
        let res_remainder = PageRes::<String>::new(101, 1, 10, vec![]);
        assert_eq!(res_remainder.total_pages, 11);

        // 边界情况：除数为 0 (防 Panic)
        let res_zero_size = PageRes::<String>::new(100, 1, 0, vec![]);
        assert_eq!(res_zero_size.total_pages, 0);

        // 边界情况：总数为 0
        let res_empty = PageRes::<String>::new(0, 1, 10, vec![]);
        assert_eq!(res_empty.total_pages, 0);
    }
}
