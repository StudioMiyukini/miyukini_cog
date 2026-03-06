use crate::errors::MiyucloudError;

/// Supported image formats for thumbnail generation
pub fn is_thumbnailable(mime_type: &str) -> bool {
    matches!(
        mime_type,
        "image/png" | "image/jpeg" | "image/webp" | "image/gif" | "image/bmp"
    )
}

/// Generate a thumbnail from image data
pub fn generate_thumbnail(
    data: &[u8],
    max_width: u32,
    max_height: u32,
) -> Result<Vec<u8>, MiyucloudError> {
    let img = image::load_from_memory(data)
        .map_err(|e| MiyucloudError::Internal(format!("Failed to decode image: {e}")))?;

    let thumbnail = img.thumbnail(max_width, max_height);

    let mut buf = std::io::Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buf, image::ImageFormat::Png)
        .map_err(|e| MiyucloudError::Internal(format!("Failed to encode thumbnail: {e}")))?;

    Ok(buf.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_thumbnailable() {
        assert!(is_thumbnailable("image/png"));
        assert!(is_thumbnailable("image/jpeg"));
        assert!(!is_thumbnailable("application/pdf"));
        assert!(!is_thumbnailable("text/plain"));
    }
}
