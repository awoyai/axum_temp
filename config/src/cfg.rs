use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub db: DB,
    pub server: Server,
    pub log: Log,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DB {
    pub link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)
    /// `0.0.0.0:3000`
    pub address: String,
    /// 服务器ssl
    pub ssl: bool,
    /// 响应数据gzip
    pub content_gzip: bool,
    /// 缓存时间
    pub cache_time: u64,
    /// 缓存方式
    pub cache_method: u32,
    /// api 前缀  例如："/api_v1"
    pub api_prefix: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Log {
    /// `log_level` 日志输出等级
    pub log_level: String,
    /// `dir` 日志输出文件夹
    pub dir: String,
    /// `file` 日志输出文件名
    pub file: String,
    /// 允许操作日志输出
    pub enable_oper_log: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Jwt {
    /// JWT 密钥
    pub jwt_secret: String,
    /// JWT 过期时间
    pub jwt_exp: i64,
}
