use crate::service::error::errors::Errors;
use infer;

pub fn validate_and_get_image_info(data: &[u8], max_size: usize) -> Result<(String, &'static str), Errors> {
    if data.is_empty() {
        return Err(Errors::BadRequestError("Empty file".to_string()));
    }

    // Check file size
    if data.len() > max_size {
        return Err(Errors::FileTooLargeError(format!(
            "Image too large: {} bytes (max: {} bytes)",
            data.len(),
            max_size
        )));
    }

    // Use infer crate to detect file type
    match infer::get(data) {
        Some(kind) => {
            match kind.mime_type() {
                "image/jpeg" => Ok((kind.mime_type().to_string(), "jpg")),
                "image/png" => Ok((kind.mime_type().to_string(), "png")),
                "image/gif" => Ok((kind.mime_type().to_string(), "gif")),
                "image/webp" => Ok((kind.mime_type().to_string(), "webp")),
                _ => Err(Errors::BadRequestError(
                    format!("Unsupported image format: {}", kind.mime_type())
                )),
            }
        }
        None => Err(Errors::BadRequestError(
            "Cannot determine file type or unsupported format".to_string()
        )),
    }
}

pub fn generate_image_hash(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}