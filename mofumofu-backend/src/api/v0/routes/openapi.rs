use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::dto::follow::request::create::CreateFollowRequest;
use crate::dto::follow::request::delete::DeleteFollowRequest;
use crate::dto::follow::response::follow_list::FollowListResponse;
use crate::dto::hashtag::request::trending_hashtags::TrendingHashtagsRequest;
use crate::dto::hashtag::response::trending_hashtags::TrendingHashtagsResponse;
use crate::dto::post::request::create_post::CreatePostRequest;
use crate::dto::post::request::get_by_handle_and_slug::GetPostByHandleAndSlugRequest;
use crate::dto::post::request::thumbnail_image::PostThumbnailForm;
use crate::dto::post::request::{
    GetPostsRequest, PostSortOrder, SearchPostsRequest,
};
use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::dto::user::request::avatar_image::ProfileAvatarForm;
use crate::dto::user::request::banner_image::ProfileBannerForm;
use crate::dto::user::request::create::CreateUserRequest;
use crate::dto::user::request::get_profile::GetUserProfileRequest;
use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::dto::user::response::handle_check::HandleCheckResponse;
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
        crate::api::v0::routes::auth::github::github_sign_in,
        crate::api::v0::routes::auth::google::google_sign_in,
        crate::api::v0::routes::auth::sign_in::sign_in,
        crate::api::v0::routes::auth::sign_out::sign_out,
        crate::api::v0::routes::auth::refresh::refresh,
        crate::api::v0::routes::user::get_my_profile::get_my_profile,
        crate::api::v0::routes::user::create_user::create_user,
        crate::api::v0::routes::user::check_handle::check_handle_availability,
        crate::api::v0::routes::user::get_profile::get_profile,
        crate::api::v0::routes::user::update_profile::update_profile,
        crate::api::v0::routes::user::upload_avatar::upload_avatar,
        crate::api::v0::routes::user::upload_banner::upload_banner,
        crate::api::v0::routes::post::create_post::create_post,
        crate::api::v0::routes::post::get_post_by_handle_and_slug::get_post_by_handle_and_slug,
        crate::api::v0::routes::post::get_posts::get_posts,
        crate::api::v0::routes::post::search_posts::search_posts,
        crate::api::v0::routes::post::upload_thumbnail::upload_thumbnail,
        crate::api::v0::routes::follow::create_follow::api_create_follow,
        crate::api::v0::routes::follow::delete_follow::api_delete_follow,
        crate::api::v0::routes::follow::get_followers_list::get_followers,
        crate::api::v0::routes::follow::get_following_list::get_following,
        crate::api::v0::routes::hashtag::trending_hashtags::trending_hashtags
    ),
    components(
        schemas(
            AuthLoginRequest,
            AuthJWTResponse,
            CreateUserRequest,
            CreatePostRequest,
            GetPostByHandleAndSlugRequest,
            GetPostsRequest,
            SearchPostsRequest,
            PostSortOrder,
            PostInfoResponse,
            PostAuthor,
            PostListItem,
            GetPostsResponse,
            CreateFollowRequest,
            DeleteFollowRequest,
            FollowListResponse,
            GetUserProfileRequest,
            UpdateProfileRequest,
            HandleCheckResponse,
            UserInfoResponse,
            ErrorResponse,
            ProfileAvatarForm,
            ProfileBannerForm,
            PostThumbnailForm,
            TrendingHashtagsRequest,
            TrendingHashtagsResponse,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "User endpoints"),
        (name = "Post", description = "Post endpoints"),
        (name = "Follow", description = "Follow endpoints"),
        (name = "Hashtag", description = "Hashtag endpoints")
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
