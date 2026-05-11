pub mod database;
pub mod jwt;
pub mod logger;
pub mod server;

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use std::sync::OnceLock;

// 全局单例
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// 获取全局静态配置引用
pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(|| {
        AppConfig::load().expect("🔥 致命错误：配置加载失败，请检查配置文件与环境变量")
    })
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub env: String,
    pub server: server::ServerConfig,
    pub database: database::DatabaseConfig,
    pub log: logger::LoggerConfig,
    pub jwt: jwt::JwtConfig,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        // 加载 .env 环境变量（如果存在）
        dotenvy::dotenv().ok();

        // 获取运行模式：dev, prod, test
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());

        let s = Config::builder()
            // 1. 加载默认配置
            .add_source(File::with_name("config/default").required(true))
            // 2. 加载环境特定配置 (如 config/dev.toml)，不强制存在
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // 3. ✨ 核心：支持环境变量覆盖
            // 规则：APP__DATABASE__URL 会覆盖 database.url
            .add_source(Environment::with_prefix("APP").separator("__"))
            // 4. 强制注入当前环境标识
            .set_override("env", run_mode)?
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::FileFormat;

    /// 内存加载辅助：避免任何文件系统/环境变量依赖
    fn load_config_from_str(toml: &str, env_name: &str) -> Result<AppConfig, config::ConfigError> {
        Config::builder()
            .add_source(config::File::from_str(toml, FileFormat::Toml))
            .set_override("env", env_name)?
            .build()?
            .try_deserialize()
    }
    #[test]
    fn test_minimal_valid_config() {
        // 仅提供必填项（验证默认值生效）
        let toml = r#"
            [server]
            [log]
            [database]
            url = "postgres://test:test@localhost/test"
            [jwt]
            secret = "min-secret-32-chars-required-here"
        "#;
        let cfg = load_config_from_str(toml, "test").unwrap();

        // 验证默认值（与各子模块 default_* 函数一致）
        assert_eq!(cfg.server.host, "127.0.0.1");
        assert_eq!(cfg.server.port, 3000);
        assert_eq!(cfg.database.max_connections, 10);
        assert_eq!(cfg.database.connect_timeout, 10000);
        assert_eq!(cfg.log.level, "info");
        assert_eq!(cfg.log.file_path, Some("logs/app.log".to_string()));
        assert_eq!(cfg.jwt.token_exp, 86400);
    }

    #[test]
    fn test_full_config_deserialization() {
        let toml = r#"
            [server]
            host = "0.0.0.0"
            port = 8080
            [database]
            url = "postgres://prod:prod@db/prod"
            max_connections = 50
            connect_timeout = 30000
            [log]
            level = "warn"
            file_path = "prod.log"
            [jwt]
            secret = "prod-secret-32-chars-minimum!!"
            token_exp = 3600
        "#;
        let cfg = load_config_from_str(toml, "prod").unwrap();

        assert_eq!(cfg.env, "prod");
        assert_eq!(cfg.server.port, 8080);
        assert_eq!(cfg.database.max_connections, 50);
        assert_eq!(cfg.log.level, "warn");
        assert_eq!(cfg.jwt.token_exp, 3600);
    }

    #[test]
    fn test_missing_required_database_field_fails() {
        // 缺失 database.url（必填）
        let toml = r#"
            [server]
            [log]
            [jwt]
            secret = "valid-secret-min-32-chars"
        "#;
        let err = load_config_from_str(toml, "test").unwrap_err();
        // 错误信息应该包含 database
        assert!(
            err.to_string().to_lowercase().contains("database"),
            "错误信息应提示缺失字段: {:?}",
            err
        );
    }
    #[test]
    fn test_missing_required_jwt_field_fails() {
        // 缺失 jwt.secret（必填）
        let toml = r#"
            [server]
            [log]
            [database]
            url = "test-url"
        "#;
        let err = load_config_from_str(toml, "test").unwrap_err();
        // 错误信息应该包含 jwt
        assert!(
            err.to_string().to_lowercase().contains("jwt"),
            "错误信息应提示缺失字段: {:?}",
            err
        );
    }

    #[test]
    fn test_jwt_debug_redaction() {
        let toml = r#"
            [server]
            [log]
            [database]
            url = "test-url"
            [jwt]
            secret = "REAL_SECRET_SHOULD_NEVER_APPEAR_IN_LOGS"
            token_exp = 7200
        "#;
        let cfg = load_config_from_str(toml, "test").unwrap();

        // 直接测试 Debug 实现
        let debug_output = format!("{:?}", cfg.jwt);

        assert!(
            debug_output.contains("******"),
            "Debug 输出必须包含脱敏标记: {}",
            debug_output
        );
        assert!(
            !debug_output.contains("REAL_SECRET"),
            "Debug 严禁泄露原始 secret: {}",
            debug_output
        );
        assert!(
            debug_output.contains("token_exp"),
            "其他字段应正常显示: {}",
            debug_output
        );
    }

    #[test]
    fn test_server_defaults() {
        let server: server::ServerConfig = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(server.host, "127.0.0.1");
        assert_eq!(server.port, 3000);
    }

    #[test]
    fn test_jwt_token_exp_default() {
        let jwt: jwt::JwtConfig =
            serde_json::from_value(serde_json::json!({"secret": "min32charsrequired"})).unwrap();
        assert_eq!(jwt.token_exp, 86400);
    }

    // 辅助函数：将工作目录切换到项目根目录
    fn setup_env() {
        // env!("CARGO_MANIFEST_DIR") 会获取当前 crate (shared) 的绝对路径
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // 向上退两层：crates/shared -> crates -> 根目录
        path.pop();
        path.pop();
        // 设置当前工作目录为根目录
        std::env::set_current_dir(path).unwrap();
    }
    #[test]
    fn test_sanity_check_dev_config() {
        // 1. 切换到项目根目录 (找到 config 文件夹)
        setup_env();
        // 2. 强制指定加载 dev 模式
        // 这会触发加载 config/default.toml 和 config/dev.toml
        unsafe { std::env::set_var("RUN_MODE", "dev") };
        // 3. 尝试加载
        let result = AppConfig::load();
        // 4. 清理环境
        unsafe { std::env::remove_var("RUN_MODE") };
        // 5. 断言：只要不报错，就说明 dev.toml 语法是合法的
        assert!(
            result.is_ok(),
            "🔥 致命错误: config/dev.toml (或 default.toml) 解析失败！\n请检查文件是否存在，或是否有拼写/语法错误。\n错误详情: {:?}",
            result.err()
        );
    }
}
