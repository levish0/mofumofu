use axum::http::{HeaderName, HeaderValue};
use dotenvy::dotenv;
use std::env;
use std::sync::LazyLock;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub is_dev: bool,

    pub jwt_secret: String,
    pub auth_access_token_expire_time: i64,
    pub auth_refresh_token_expire_time: i64,

    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_max_connection: u32,
    pub db_min_connection: u32,

    // Redis
    // pub redis_host: String,
    // pub redis_port: String,
    // pub redis_ttl: u64,

    // OpenSearch
    pub opensearch_host: String,
    pub opensearch_port: String,
    pub opensearch_scheme: String,
    // pub opensearch_username: String,
    // pub opensearch_password: String,
    pub opensearch_verify_certs: bool,

    pub server_host: String,
    pub server_port: String,

    pub cors_allowed_origins: Vec<HeaderValue>,
    pub cors_allowed_headers: Vec<HeaderName>,
    pub cors_max_age: Option<u64>,
}

// LazyLock으로 자동 초기화
static CONFIG: LazyLock<DbConfig> = LazyLock::new(|| {
    dotenv().ok();

    let is_dev = matches!(
        env::var("ENVIRONMENT").as_deref(),
        Ok("dev") | Ok("development")
    );

    let cors_origins: Vec<HeaderValue> = match env::var("CORS_ALLOWED_ORIGINS").ok() {
        Some(origins) => origins
            .split(',')
            .filter_map(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    warn!("Empty origin found in CORS_ALLOWED_ORIGINS.");
                    None
                } else {
                    HeaderValue::from_str(trimmed_s).ok().or_else(|| {
                        warn!(
                            "Invalid origin '{}' found in CORS_ALLOWED_ORIGINS.",
                            trimmed_s
                        );
                        None
                    })
                }
            })
            .collect(),
        None => {
            vec![]
        }
    };

    let cors_headers: Vec<HeaderName> = match env::var("CORS_ALLOWED_HEADERS").ok() {
        Some(headers) => headers
            .split(',')
            .filter_map(|s| {
                let trimmed_s = s.trim();
                if trimmed_s.is_empty() {
                    warn!("Empty header name found in CORS_ALLOWED_HEADERS.");
                    None
                } else {
                    HeaderName::from_bytes(trimmed_s.as_bytes())
                        .ok()
                        .or_else(|| {
                            warn!(
                                "Invalid header name '{}' found in CORS_ALLOWED_HEADERS.",
                                trimmed_s
                            );
                            None
                        })
                }
            })
            .collect(),
        None => {
            vec![]
        }
    };

    DbConfig {
        is_dev,
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),

        auth_access_token_expire_time: env::var("AUTH_ACCESS_TOKEN_EXPIRE_TIME")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30), // 기본값 30분
        auth_refresh_token_expire_time: env::var("AUTH_REFRESH_TOKEN_EXPIRE_TIME")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(14), // 기본값 14일 (일주일)

        db_user: env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        db_password: env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        db_host: env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set"),
        db_port: env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set"),
        db_name: env::var("POSTGRES_NAME").expect("POSTGRES_NAME must be set"),
        db_max_connection: env::var("POSTGRES_MAX_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100),
        db_min_connection: env::var("POSTGRES_MIN_CONNECTION")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),

        // Redis
        /*
        redis_host: env::var("REDIS_HOST").expect("REDIS_HOST must be set"),
        redis_port: env::var("REDIS_PORT").expect("REDIS_PORT must be set"),
        redis_ttl: env::var("REDIS_TTL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3600),

        */
        // Opensearch
        opensearch_host: env::var("OPENSEARCH_HOST").expect("OPENSEARCH_HOST must be set"),
        opensearch_port: env::var("OPENSEARCH_PORT").expect("OPENSEARCH_HOST must be set"),
        opensearch_scheme: env::var("OPENSEARCH_SCHEME")
            .ok()
            .unwrap_or("http".to_string()),
        // opensearch_username: env::var("OPENSEARCH_USERNAME").expect("OPENSEARCH_USERNAME must be set"),
        // opensearch_password:env::var("OPENSEARCH_PASSWORD").expect("OPENSEARCH_PASSWORD must be set"),
        opensearch_verify_certs: env::var("OPENSEARCH_VERIFY_CERTS")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false),

        server_host: env::var("HOST").expect("HOST must be set in .env file"),
        server_port: env::var("PORT").expect("PORT must be set in .env file"),
        cors_allowed_origins: cors_origins,
        cors_allowed_headers: cors_headers,
        cors_max_age: env::var("CORS_MAX_AGE").ok().and_then(|v| v.parse().ok()),
    }
});

impl DbConfig {
    // 이제 단순히 CONFIG에 접근만 하면 됨
    pub fn get() -> &'static DbConfig {
        &CONFIG
    }
}
