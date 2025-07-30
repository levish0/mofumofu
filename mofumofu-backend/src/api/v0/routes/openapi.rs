use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::dto::follow::request::create::CreateFollowRequest;
use crate::dto::follow::request::delete::DeleteFollowRequest;
use crate::dto::follow::response::follow_list::FollowListResponse;
use crate::dto::post::request::create::CreatePostRequest;
use crate::dto::user::request::create::CreateUserRequest;
use crate::dto::user::request::get_profile::GetUserProfileRequest;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::error::errors::ErrorResponse;
use utoipa::openapi::security::{ApiKey, ApiKeyValue};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::v0::routes::auth::auth::login,
        crate::api::v0::routes::auth::auth::refresh,
        crate::api::v0::routes::user::get_my_profile::get_my_profile,
        crate::api::v0::routes::user::create_user::create_user,
        crate::api::v0::routes::user::get_profile::get_profile,
        crate::api::v0::routes::post::post::create_post,
        crate::api::v0::routes::follow::create_follow::api_create_follow,
        crate::api::v0::routes::follow::delete_follow::api_delete_follow,
        crate::api::v0::routes::follow::get_followers_list::get_followers,
        crate::api::v0::routes::follow::get_following_list::get_following
    ),
    components(
        schemas(
            AuthLoginRequest,
            AuthJWTResponse,
            CreateUserRequest,
            CreatePostRequest,
            CreateFollowRequest,
            DeleteFollowRequest,
            FollowListResponse,
            GetUserProfileRequest,
            UserInfoResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "User", description = "User endpoints"),
        (name = "Post", description = "Post endpoints"),
         (name = "Follow", description = "Follow endpoints"),
        (name = "Auth", description = "Authentication endpoints")
    ),
    modifiers(&SecurityAddon) // 보안 스키마 등록
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );

            components.add_security_scheme(
                "refresh_token_cookie",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("refresh_token"))),
            )
        }
    }
}
