// Image and video decoding
use anyhow::Result;
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Png,
    Jpeg,
    Gif,
    Webp,
    Bmp,
    Unknown,
}

pub struct DecodedImage {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
    pub format: ImageType,
}

impl DecodedImage {
    pub fn size_bytes(&self) -> usize {
        self.rgba_data.len()
    }
}

pub fn detect_image_type(data: &[u8]) -> ImageType {
    if data.len() < 8 {
        return ImageType::Unknown;
    }

    // PNG magic number
    if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
        return ImageType::Png;
    }

    // JPEG magic number
    if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return ImageType::Jpeg;
    }

    // GIF magic number
    if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        return ImageType::Gif;
    }

    // WebP magic number
    if data.len() >= 12 && data.starts_with(b"RIFF") && &data[8..12] == b"WEBP" {
        return ImageType::Webp;
    }

    // BMP magic number
    if data.starts_with(b"BM") {
        return ImageType::Bmp;
    }

    ImageType::Unknown
}

pub fn decode_image(data: &[u8]) -> Result<DecodedImage> {
    let image_type = detect_image_type(data);

    match image_type {
        ImageType::Webp => decode_webp(data),
        _ => decode_standard_image(data, image_type),
    }
}

fn decode_standard_image(data: &[u8], detected_type: ImageType) -> Result<DecodedImage> {
    let cursor = Cursor::new(data);
    let img = image::load(cursor, ImageFormat::from_path("dummy.img")?)?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    Ok(DecodedImage {
        width,
        height,
        rgba_data: rgba.into_raw(),
        format: detected_type,
    })
}

fn decode_webp(data: &[u8]) -> Result<DecodedImage> {
    let decoder = webp::Decoder::new(data);
    let decoded = decoder.decode()
        .ok_or_else(|| anyhow::anyhow!("Failed to decode WebP"))?;

    let (width, height) = decoded.dimensions();
    let rgba_data = decoded.to_owned().into_vec();

    Ok(DecodedImage {
        width,
        height,
        rgba_data,
        format: ImageType::Webp,
    })
}

#[cfg(feature = "svg")]
pub fn decode_svg(data: &[u8], target_width: Option<u32>, target_height: Option<u32>) -> Result<DecodedImage> {
    use resvg::usvg;

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(data, &opt)?;

    let size = tree.size();
    let width = target_width.unwrap_or(size.width() as u32);
    let height = target_height.unwrap_or(size.height() as u32);

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

    let transform = tiny_skia::Transform::from_scale(
        width as f32 / size.width(),
        height as f32 / size.height(),
    );

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Convert RGBA to our format
    let rgba_data = pixmap.take();

    Ok(DecodedImage {
        width,
        height,
        rgba_data,
        format: ImageType::Unknown, // SVG doesn't fit our enum
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_png() {
        let png_magic = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_image_type(&png_magic), ImageType::Png);
    }

    #[test]
    fn test_detect_jpeg() {
        let jpeg_magic = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(detect_image_type(&jpeg_magic), ImageType::Jpeg);
    }

    #[test]
    fn test_detect_gif() {
        let gif_magic = b"GIF89a";
        assert_eq!(detect_image_type(gif_magic), ImageType::Gif);
    }

    #[test]
    fn test_detect_unknown() {
        let unknown = [0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_image_type(&unknown), ImageType::Unknown);
    }
}
