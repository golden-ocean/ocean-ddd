use serde::Deserialize;

// --- JWT 配置 ---
#[derive(Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    #[serde(default = "default_token_exp")]
    pub token_exp: u64, // 秒
}

// --- 默认参数 ---
fn default_token_exp() -> u64 {
    86400
} // 1天

// 手动实现 Debug 以脱敏 Secret
impl std::fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtConfig")
            .field("secret", &"******") // 脱敏
            .field("token_exp", &self.token_exp)
            .finish()
    }
}
