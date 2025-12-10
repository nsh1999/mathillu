use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

use crate::hsv_to_rgb::hsv_to_rgb;

/// Generates an image based on Schrödinger's equation (2D Gaussian wave packet).
///
/// # Arguments
///
/// * `width` - Width of the output image.
/// * `height` - Height of the output image.
/// * `bands` - Number of color bands.
/// * `center_x` - X center coordinate.
/// * `center_y` - Y center coordinate.
/// * `zoom` - Zoom level.
/// * `font_path` - Path to font file.
/// * `zoom_text_x` - X position of zoom text.
/// * `zoom_text_y` - Y position of zoom text.
/// * `zoom_font_size` - Font size for zoom text.
/// * `output_path` - Path to save the generated image.
pub fn generate_schrodinger(width: u32, height: u32, bands: u32, center_x: f64, center_y: f64, zoom: f64, font_path: &str, zoom_text_x: i32, zoom_text_y: i32, zoom_font_size: f32, output_path: &str) {
    // Create an empty image buffer
    let mut imgbuf = ImageBuffer::new(width, height);

    let sigma = 0.5; // Standard deviation for the Gaussian

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let scale_x = 2.0 / zoom;
        let scale_y = 2.0 / zoom;
        let cx = center_x + (x as f64 - width as f64 / 2.0) / width as f64 * scale_x;
        let cy = center_y + (y as f64 - height as f64 / 2.0) / height as f64 * scale_y;

        // Compute distance from center
        let r_squared = cx * cx + cy * cy;

        // Probability density |ψ|^2 for Gaussian wave packet
        let density = (-r_squared / (2.0 * sigma * sigma)).exp();

        // Convert density to color band
        let band_index = if bands > 1 {
            (density * (bands - 1) as f64).round() as f64
        } else {
            0.0
        };

        let hue = if bands > 1 {
            band_index / (bands - 1) as f64 * 240.0
        } else {
            0.0
        };

        let color = hsv_to_rgb(hue as f32, 255, 255);

        *pixel = color;
    }

    // Draw zoom text
    let font_data = std::fs::read(font_path).expect("Failed to read font file");
    let font = Font::try_from_vec(font_data).expect("Failed to load font");
    let scale = Scale { x: zoom_font_size, y: zoom_font_size };
    let text = format!("ZOOM {:.1}", zoom);
    draw_text_mut(&mut imgbuf, Rgba([0, 0, 0, 255]), zoom_text_x, zoom_text_y, scale, &font, &text);

    imgbuf.save(output_path).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_generate_schrodinger_creates_file() {
        let output_path = "/tmp/test_schrodinger.png";
        let font_path = "/System/Library/Fonts/Helvetica.ttc"; // Use system font for testing

        // Skip test if font doesn't exist
        if !Path::new(font_path).exists() {
            return;
        }

        generate_schrodinger(
            100, 100, 8, 0.0, 0.0, 1.0,
            font_path, 5, 80, 12.0, output_path
        );

        assert!(Path::new(output_path).exists());

        // Check file size is reasonable (should be > 0)
        let metadata = fs::metadata(output_path).unwrap();
        assert!(metadata.len() > 1000); // PNG files should be at least this big

        // Clean up
        fs::remove_file(output_path).ok();
    }

    #[test]
    fn test_gaussian_wave_packet_calculation() {
        let sigma = 0.5_f64;

        // Test center point (0,0) - should have maximum density
        let r_squared_center = 0.0_f64 * 0.0_f64 + 0.0_f64 * 0.0_f64;
        let density_center = (-r_squared_center / (2.0_f64 * sigma * sigma)).exp();
        assert_eq!(density_center, 1.0_f64); // exp(0) = 1

        // Test point at distance sigma
        let r_squared_sigma = sigma * sigma;
        let density_sigma = (-r_squared_sigma / (2.0_f64 * sigma * sigma)).exp();
        let expected = (-0.5_f64).exp(); // exp(-0.5)
        assert!((density_sigma - expected).abs() < 1e-10);

        // Test point farther away - density should be lower
        let r_squared_far = 4.0_f64 * sigma * sigma; // 2*sigma distance
        let density_far = (-r_squared_far / (2.0_f64 * sigma * sigma)).exp();
        assert!(density_far < density_center);
        assert!(density_far < density_sigma);
    }

    #[test]
    fn test_density_to_band_conversion() {
        let bands = 8;

        // Test maximum density (1.0) should map to highest band
        let band_index_max = (1.0 * (bands - 1) as f64).round() as f64;
        assert_eq!(band_index_max, (bands - 1) as f64);

        // Test zero density should map to band 0
        let band_index_min = (0.0 * (bands - 1) as f64).round() as f64;
        assert_eq!(band_index_min, 0.0);

        // Test 0.5 density
        let band_index_mid = (0.5 * (bands - 1) as f64).round() as f64;
        assert_eq!(band_index_mid, 4.0); // 0.5 * 7 = 3.5, rounds to 4
    }
}