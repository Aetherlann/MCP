// Image and video decoding
// This would integrate:
// - image crate for PNG/JPEG/WEBP
// - FFmpeg for video
// - OS decoders (WMF/AVFoundation/VAAPI)

use anyhow::Result;

pub fn decode_image(data: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let img = image::load_from_memory(data)?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    Ok((width, height, rgba.into_raw()))
}
