use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::infrastructure::configs::logger::LoggerConfig;

/// 初始化全局追踪与日志系统
/// 接收全局的日志配置作为参数
pub fn init_logger(cfg: &LoggerConfig) {
    // 1. 动态构造兜底过滤规则
    // 使用配置文件中的 level (例如 "info" 或 "debug")，并追加框架的过滤规则（屏蔽 sqlx 杂音）
    let fallback_directive = format!("{},sqlx=warn,tower_http=debug", cfg.level);

    // 2. 核心逻辑：优先读取系统环境变量 RUST_LOG
    // 如果环境变量没设置，就使用我们上面用 LogConfig 拼接出来的兜底规则
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(fallback_directive));

    // 3. 控制台格式化输出层
    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(false) // 在终端看日志时，关掉 thread_id 会让画面更清爽
        .with_line_number(true);

    // 💡 进阶提示：如果你未来想把日志输出到文件
    // 既然你的 LogConfig 里有 file_path，你可以引入 `tracing-appender` 库
    // 在这里构建一个 file_layer 然后加到下面的 registry 中去。

    // 4. 注册全局 Subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(stdout_layer)
        .init();
}
