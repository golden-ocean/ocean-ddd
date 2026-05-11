use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

use crate::infrastructure::configs::database::DatabaseConfig;

/// 初始化数据库连接池
///
/// 流程：
/// 1. 从全局配置加载数据库 URL 和连接池参数
/// 2. 尝试建立连接池（带超时校验）
/// 3. 执行 SQL 迁移脚本同步表结构
pub async fn init_db(cfg: &DatabaseConfig) -> anyhow::Result<PgPool> {
    info!(
        "正在初始化数据库连接池，目标地址: {}",
        mask_db_url(&cfg.url)
    );

    // 1. 创建连接池
    let pool = PgPoolOptions::new()
        // 设置连接池最小连接数
        .min_connections(cfg.min_connections)
        // 设置连接池最大连接数
        .max_connections(cfg.max_connections)
        // 设置获取连接的超时时间
        .acquire_timeout(Duration::from_millis(cfg.connect_timeout))
        // 设置空闲连接存活时间
        .idle_timeout(Duration::from_secs(600))
        .connect(&cfg.url)
        .await
        .context("🔥 致命错误：无法连接到 PostgreSQL 数据库")?;

    info!("✅ 数据库连接池初始化成功！");
    // 2. 运行迁移脚本
    // 注意：路径相对于当前 crate (shared) 的根目录
    // 如果你的 migrations 文件夹在项目总根目录，需要用 ../../migrations
    // info!("正在检查并运行数据库迁移...");
    // sqlx::migrate!("../../migrations")
    //     .run(&pool)
    //     .await
    //     .context("🔥 错误：无法完成数据库迁移")?;

    // info!("✅ 数据库初始化完成并已同步最新表结构");

    Ok(pool)
}

/// 辅助函数：脱敏数据库 URL 用于日志输出（保留用户名，隐藏密码）
fn mask_db_url(url: &str) -> String {
    // 目标匹配: postgres://username:password@host:port/db
    if let Some(proto_idx) = url.find("://") {
        let after_proto = &url[proto_idx + 3..];

        if let Some(at_idx) = after_proto.find('@') {
            let credentials = &after_proto[..at_idx];
            let suffix = &after_proto[at_idx..]; // @host:port/db

            // 如果存在用户名和密码的冒号分隔符
            if let Some(colon_idx) = credentials.find(':') {
                let prefix = &url[..proto_idx + 3]; // postgres://
                let username = &credentials[..colon_idx];
                return format!("{}{}:******{}", prefix, username, suffix);
            }
        }
    }
    // 如果解析失败，安全起见全部打码
    "***[Redacted DB URL]***".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_db_url() {
        let db_url = "postgres://myuser:mypassword@localhost:5432/mydb";
        assert_eq!(
            mask_db_url(db_url),
            "postgres://myuser:******@localhost:5432/mydb"
        );

        let invalid_url = "invalid-url";
        assert_eq!(mask_db_url(invalid_url), "***[Redacted DB URL]***");
    }
}
