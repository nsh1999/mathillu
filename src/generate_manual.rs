use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

/// Generates a manual/custom visualization.
///
/// This is a placeholder function that can be customized for specific visualization needs.
/// Currently, it generates a simple gradient pattern for demonstration.
///
/// # Arguments
///
/// * `width` - Width of the output image.
/// * `height` - Height of the output image.
/// * `max_iterations` - Maximum number of iterations (currently unused).
/// * `bands` - Number of color bands.
/// * `center_x` - X center offset in pixels from image center.
/// * `center_y` - Y center offset in pixels from image center.
/// * `zoom` - Zoom level.
/// * `m_size` - Size of the mathematical space (square).
/// * `font_path` - Path to font file.
/// * `zoom_text_x` - X position of zoom text.
/// * `zoom_text_y` - Y position of zoom text.
/// * `zoom_font_size` - Font size for zoom text.
/// * `output_path` - Path to save the generated image.
pub fn generate_manual(width: u32, height: u32, max_iterations: u32, bands: u32, center_x: f64, center_y: f64, zoom: f64, m_size: f64, font_path: &str, zoom_text_x: i32, zoom_text_y: i32, zoom_font_size: f32, output_path: &str) {
    let mut imgbuf = ImageBuffer::new(width, height);

    // Generate a simple gradient pattern
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Create a gradient based on position
        let r = ((x as f32 / width as f32) * 255.0) as u8;
        let g = ((y as f32 / height as f32) * 255.0) as u8;
        let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;

        *pixel = Rgba([r, g, b, 255]);
    }

    // Add zoom text if font is available
    if let Ok(font_data) = std::fs::read(font_path) {
        let font = Font::try_from_vec(font_data).expect("Failed to load font");
        let scale = Scale::uniform(zoom_font_size);
        let text = format!("Manual Mode - Zoom: {:.2}", zoom);

        draw_text_mut(&mut imgbuf, Rgba([255, 255, 255, 255]), zoom_text_x, zoom_text_y, scale, &font, &text);
    }

    // Save the image
    imgbuf.save(output_path).expect("Failed to save image");
    println!("Manual visualization saved to: {}", output_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_generate_manual_creates_file() {
        let output_path = "/tmp/test_manual.png";
        let font_path = "/System/Library/Fonts/Helvetica.ttc"; // Use system font for testing

        generate_manual(
            100, 100, 50, 8, 0.0, 0.0, 1.0, 10.0,
            font_path, 5, 80, 12.0, output_path
        );

        assert!(Path::new(output_path).exists());

        // Cleanup
        let _ = fs::remove_file(output_path);
    }
}