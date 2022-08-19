use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_text, text_size};
use rusttype::{Font, Scale};

use crate::assets::Assets;

/// Given some text, builds a megamind meme image.
///
///# Example
///```rust
///use no_bitches::build_megamind_meme;
///
///let meme = build_megamind_meme("No Unsafe Code?", None);
///meme.save("./meme.png").unwrap();
///
///let font_meme = build_megamind_meme("No Font Size?", Some(42_f32));
///font_meme.save("./meme.png").unwrap();
///```
pub fn build_megamind_meme(image_text: &str, font_size: Option<f32>) -> RgbaImage {
    // Load the base image
    let base_image = {
        // Read from program data
        let raw_base_image = Assets::get("megamind_clean.jpg")
            .expect("Failed to load base image. Was it not packed into the library?");

        // Parse into a usable format
        let mut reader = ImageReader::new(Cursor::new(raw_base_image.data));
        reader.set_format(image::ImageFormat::Jpeg);
        reader.decode().expect("Failed to decode base image")
    };

    // Load our font
    let raw_font = Assets::get("impact_unicode.ttf")
        .expect("Failed to load font. Was it not packed into the library?");
    let font = Font::try_from_bytes(&raw_font.data).expect("Failed to parse font");

    // Set up a font scaler
    let font_scale = Scale {
        x: font_size.unwrap_or(40.0),
        y: font_size.unwrap_or(40.0),
    };

    // Get the text bounding box
    let (text_width, _) = text_size(font_scale, &font, image_text);

    // Draw the text
    draw_text(
        &base_image,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        (base_image.width() as i32 / 2) - (text_width / 2),
        10,
        font_scale,
        &font,
        image_text,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_megamind_meme() {
        let image = build_megamind_meme("Hello, world!", None);
        image.save("./meme.png").unwrap();
    }
}
