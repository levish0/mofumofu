use crate::config::db_config::DbConfig;
use crate::service::error::protocol::follow::{
    FOLLOW_ALREADY_FOLLOWING, FOLLOW_CANNOT_FOLLOW_SELF, FOLLOW_NOT_EXIST,
};
use crate::service::error::protocol::general::{BAD_REQUEST, VALIDATION_ERROR};
use crate::service::error::protocol::oauth::{
    OAUTH_INVALID_AUTH_URL, OAUTH_INVALID_REDIRECT_URL, OAUTH_INVALID_TOKEN_URL,
    OAUTH_TOKEN_EXCHANGE_FAILED, OAUTH_USER_INFO_FETCH_FAILED, OAUTH_USER_INFO_PARSE_FAILED,
};
use crate::service::error::protocol::system::{
    SYS_DATABASE_ERROR, SYS_HASHING_ERROR, SYS_NOT_FOUND, SYS_TOKEN_CREATION_ERROR,
    SYS_TRANSACTION_ERROR,
};
use crate::service::error::protocol::user::{
    USER_HANDLE_GENERATION_FAILED, USER_IMAGE_TOO_LARGE, USER_INVALID_PASSWORD, USER_INVALID_TOKEN,
    USER_NO_REFRESH_TOKEN, USER_NOT_FOUND, USER_NOT_VERIFIED, USER_TOKEN_EXPIRED,
    USER_UNAUTHORIZED,
};
use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sea_orm::{DbErr, TransactionError};
use serde::Serialize;
use tracing::error;
use utoipa::ToSchema;
// 이 모듈은 애플리케이션의 오류 처리 시스템을 구현합니다.
// 주요 기능:
// 1. 다양한 오류 유형 정의 (사용자, 문서, 권한, 시스템 등)
// 2. 오류를 HTTP 응답으로 변환하는 메커니즘
// 3. 데이터베이스 오류를 애플리케이션 오류로 변환하는 기능

// ErrorResponse 구조체: API 응답에서 오류를 표현하기 위한 구조체
// status: HTTP 상태 코드
// code: 오류 코드 문자열
// details: 개발 환경에서만 표시되는 상세 오류 메시지 (선택적)
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

// From 트레이트 구현: 데이터베이스 오류를 애플리케이션 오류로 변환
// 이를 통해 ? 연산자를 사용하여 데이터베이스 오류를 자동으로 처리할 수 있음
impl From<DbErr> for Errors {
    fn from(err: sea_orm::DbErr) -> Self {
        error!("Database error: {}", err);
        Errors::DatabaseError(err.to_string())
    }
}

// 트랜잭션 오류를 애플리케이션 오류로 변환
impl From<TransactionError<DbErr>> for Errors {
    fn from(err: TransactionError<DbErr>) -> Self {
        error!("Transaction error: {}", err);
        Errors::TransactionError(err.to_string())
    }
}

// 애플리케이션에서 발생할 수 있는 모든 오류 유형을 정의하는 열거형
// 카테고리별로 구분되어 있으며, 일부 오류는 추가 정보를 포함할 수 있음
pub enum Errors {
    // 사용자 관련 오류
    UserInvalidPassword, // 잘못된 비밀번호
    UserNotVerified,
    UserNotFound,     // 사용자를 찾을 수 없음
    UserUnauthorized, // 인증되지 않은 사용자
    UserHandleGenerationFailed,
    UserImageTooLarge,
    UserTokenExpired, // 만료된 토큰
    UserNoRefreshToken,
    UserInvalidToken, // 유효하지 않은 토큰

    // follow 관련 오류
    FollowCannotFollowSelf,
    FollowAlreadyFollowing,
    FollowNotExist,

    // oauth
    OauthInvalidAuthUrl,
    OauthInvalidTokenUrl,
    OauthInvalidRedirectUrl,
    OauthTokenExchangeFailed,
    OauthUserInfoFetchFailed,
    OauthUserInfoParseFailed,

    // 일반 오류
    BadRequestError(String), // 잘못된 요청 (추가 정보 포함)
    ValidationError(String), // 유효성 검사 오류 (추가 정보 포함)

    // 시스템 오류
    DatabaseError(String),      // 데이터베이스 오류 (추가 정보 포함)
    TransactionError(String),   // 트랜잭션 오류 (추가 정보 포함)
    NotFound(String),           // 리소스를 찾을 수 없음 (추가 정보 포함)
    HashingError(String),       // 해싱 오류 (추가 정보 포함)
    TokenCreationError(String), // 토큰 생성 오류 (추가 정보 포함)
}

// IntoResponse 트레이트 구현: Errors를 HTTP 응답으로 변환
// 각 오류 유형에 적절한 HTTP 상태 코드와 오류 코드를 매핑
impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        // 오류 유형에 따라 상태 코드, 오류 코드, 상세 정보를 결정
        let (status, code, details) = match self {
            // 사용자 관련 오류 - 주로 401 Unauthorized 또는 404 Not Found
            Errors::UserInvalidPassword => (StatusCode::UNAUTHORIZED, USER_INVALID_PASSWORD, None),
            Errors::UserNotVerified => (StatusCode::UNAUTHORIZED, USER_NOT_VERIFIED, None),
            Errors::UserNotFound => (StatusCode::NOT_FOUND, USER_NOT_FOUND, None),
            Errors::UserUnauthorized => (StatusCode::UNAUTHORIZED, USER_UNAUTHORIZED, None),
            Errors::UserHandleGenerationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                USER_HANDLE_GENERATION_FAILED,
                None,
            ),
            Errors::UserImageTooLarge => (StatusCode::BAD_REQUEST, USER_IMAGE_TOO_LARGE, None),
            Errors::UserTokenExpired => (StatusCode::UNAUTHORIZED, USER_TOKEN_EXPIRED, None),
            Errors::UserNoRefreshToken => (StatusCode::UNAUTHORIZED, USER_NO_REFRESH_TOKEN, None),
            Errors::UserInvalidToken => (StatusCode::UNAUTHORIZED, USER_INVALID_TOKEN, None),

            // Follow
            Errors::FollowCannotFollowSelf => {
                (StatusCode::BAD_REQUEST, FOLLOW_CANNOT_FOLLOW_SELF, None)
            }
            Errors::FollowAlreadyFollowing => {
                (StatusCode::CONFLICT, FOLLOW_ALREADY_FOLLOWING, None)
            }
            Errors::FollowNotExist => (StatusCode::NOT_FOUND, FOLLOW_NOT_EXIST, None),

            // Oauth
            Errors::OauthInvalidAuthUrl => (StatusCode::BAD_REQUEST, OAUTH_INVALID_AUTH_URL, None),
            Errors::OauthInvalidTokenUrl => {
                (StatusCode::BAD_REQUEST, OAUTH_INVALID_TOKEN_URL, None)
            }
            Errors::OauthInvalidRedirectUrl => {
                (StatusCode::BAD_REQUEST, OAUTH_INVALID_REDIRECT_URL, None)
            }
            Errors::OauthTokenExchangeFailed => {
                (StatusCode::BAD_REQUEST, OAUTH_TOKEN_EXCHANGE_FAILED, None)
            }
            Errors::OauthUserInfoFetchFailed => {
                (StatusCode::BAD_REQUEST, OAUTH_USER_INFO_FETCH_FAILED, None)
            }
            Errors::OauthUserInfoParseFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                OAUTH_USER_INFO_PARSE_FAILED,
                None,
            ),

            // 일반 오류 - 400 Bad Request
            Errors::BadRequestError(msg) => (StatusCode::BAD_REQUEST, BAD_REQUEST, Some(msg)),
            Errors::ValidationError(msg) => (StatusCode::BAD_REQUEST, VALIDATION_ERROR, Some(msg)),

            // 시스템 오류 - 주로 500 Internal Server Error
            Errors::TransactionError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SYS_TRANSACTION_ERROR,
                Some(msg),
            ),
            Errors::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SYS_DATABASE_ERROR,
                Some(msg),
            ),
            Errors::NotFound(msg) => (StatusCode::NOT_FOUND, SYS_NOT_FOUND, Some(msg)),
            Errors::HashingError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SYS_HASHING_ERROR,
                Some(msg),
            ),
            Errors::TokenCreationError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                SYS_TOKEN_CREATION_ERROR,
                Some(msg),
            ),
        };

        // 개발 환경에서만 상세 오류 정보 포함
        let is_dev = DbConfig::get().is_dev;

        // 오류 응답 구성
        let body = ErrorResponse {
            status: status.as_u16(),
            code: code.to_string(),
            details: if is_dev { details } else { None }, // 개발 환경에서만 상세 정보 표시
        };

        // HTTP 응답으로 변환하여 반환
        (status, Json(body)).into_response()
    }
}

// 404 오류 처리 핸들러 함수
// 요청된 경로가 존재하지 않을 때 호출되는 전역 핸들러
pub async fn handler_404<B>(req: Request<B>) -> impl IntoResponse {
    // 요청 경로와 HTTP 메서드 추출
    let path = req.uri().path();
    let method = req.method().to_string();

    // 로그에 404 오류 기록
    error!(
        "404 Error: Requested path {} with method {} not found.",
        path, method
    );

    // NotFound 오류 반환 - 이는 IntoResponse를 통해 적절한 HTTP 응답으로 변환됨
    Errors::NotFound("The requested resource was not found.".to_string())
}
