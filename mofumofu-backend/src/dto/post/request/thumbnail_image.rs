use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostThumbnailForm {
    pub slug: String,
    #[schema(format = Binary, content_media_type = "image/*")]
    file: String,
}