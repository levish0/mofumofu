pub mod user {
    pub const USER_INVALID_PASSWORD: &str = "user:invalid_password";

    pub const USER_NOT_VERIFIED: &str = "user:not_verified";
    pub const USER_NOT_FOUND: &str = "user:not_found";
    pub const USER_UNAUTHORIZED: &str = "user:unauthorized";
    pub const USER_HANDLE_GENERATION_FAILED: &str = "user:handle_generation_failed";
    pub const USER_HANDLE_ALREADY_EXISTS: &str = "user:handle_already_exists";
    pub const USER_TOKEN_EXPIRED: &str = "user:token_expired";
    pub const USER_NO_REFRESH_TOKEN: &str = "user:no_refresh_token";
    pub const USER_INVALID_TOKEN: &str = "user:invalid_token";
}
pub mod post {
    pub const POST_NOT_FOUND: &str = "post:not_found";
}

pub mod follow {
    pub const FOLLOW_CANNOT_FOLLOW_SELF: &str = "follow:cannot_follow_self";
    pub const FOLLOW_ALREADY_FOLLOWING: &str = "follow:already_following";
    pub const FOLLOW_NOT_EXIST: &str = "follow:not_exist";
}

pub mod oauth {
    pub const OAUTH_INVALID_AUTH_URL: &str = "oauth:invalid_auth_url";
    pub const OAUTH_INVALID_TOKEN_URL: &str = "oauth:invalid_token_url";
    pub const OAUTH_INVALID_REDIRECT_URL: &str = "oauth:invalid_redirect_url";
    pub const OAUTH_TOKEN_EXCHANGE_FAILED: &str = "oauth:token_exchange_failed";
    pub const OAUTH_USER_INFO_FETCH_FAILED: &str = "oauth:user_info_fetch failed";
    pub const OAUTH_USER_INFO_PARSE_FAILED: &str = "oauth:user_info_parse failed";
}
pub mod general {
    pub const BAD_REQUEST: &str = "general:bad_request";
    pub const VALIDATION_ERROR: &str = "general:validation_error";
}

pub mod system {
    pub const SYS_INTERNAL_ERROR: &str = "system:internal_error";
    pub const SYS_HASHING_ERROR: &str = "system:hashing_error";
    pub const SYS_NOT_FOUND: &str = "system:not_found";
    pub const SYS_TRANSACTION_ERROR: &str = "system:transaction_error";
    pub const SYS_DATABASE_ERROR: &str = "system:database_error";
    pub const SYS_TOKEN_CREATION_ERROR: &str = "system:token_creation_error";
}
