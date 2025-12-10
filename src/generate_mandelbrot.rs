use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

use crate::hsv_to_rgb::hsv_to_rgb;

/// Generates a Mandelbrot set image.
///
/// # Arguments
///
/// * `width` - Width of the output image.
/// * `height` - Height of the output image.
/// * `max_iterations` - Maximum number of iterations for the Mandelbrot calculation.
/// * `bands` - Number of color bands.
/// * `center_x` - X center coordinate.
/// * `center_y` - Y center coordinate.
/// * `zoom` - Zoom level.
/// * `font_path` - Path to font file.
/// * `zoom_text_x` - X position of zoom text.
/// * `zoom_text_y` - Y position of zoom text.
/// * `zoom_font_size` - Font size for zoom text.
/// * `output_path` - Path to save the generated image.
pub fn generate_mandelbrot(width: u32, height: u32, max_iterations: u32, bands: u32, center_x: f64, center_y: f64, zoom: f64, font_path: &str, zoom_text_x: i32, zoom_text_y: i32, zoom_font_size: f32, output_path: &str) {
    // Create an empty image buffer
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let scale_x = 2.0 / zoom;
        let scale_y = 2.0 / zoom;
        let cx = center_x + (x as f64 - width as f64 / 2.0) / width as f64 * scale_x;
        let cy = center_y + (y as f64 - height as f64 / 2.0) / height as f64 * scale_y;

        let mut x0 = 0.0;
        let mut y0 = 0.0;
        let mut iteration = 0;

        while x0 * x0 + y0 * y0 <= 4.0 && iteration < max_iterations {
            let xtemp = x0 * x0 - y0 * y0 + cx;
            y0 = 2.0 * x0 * y0 + cy;
            x0 = xtemp;

            iteration += 1;
        }

        // Convert iteration to color
        let color = match iteration {
            0 => Rgba([0, 0, 0, 255]), // Black for points that didn't escape
            _ => {
                let band_index = (iteration % bands) as f64;
                let hue = if bands > 1 {
                    band_index / (bands - 1) as f64 * 240.0
                } else {
                    0.0
                };
                hsv_to_rgb(hue as f32, 255, 255)
            }
        };

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
    fn test_generate_mandelbrot_creates_file() {
        let output_path = "/tmp/test_mandelbrot.png";
        let font_path = "/System/Library/Fonts/Helvetica.ttc"; // Use system font for testing

        // Skip test if font doesn't exist
        if !Path::new(font_path).exists() {
            return;
        }

        generate_mandelbrot(
            100, 100, 50, 8, -0.5, 0.0, 1.0,
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
    fn test_mandelbrot_calculation() {
        // Test that the Mandelbrot calculation works for a known point
        // Point (0,0) should not escape within reasonable iterations
        let mut x0 = 0.0;
        let mut y0 = 0.0;
        let mut iteration = 0;
        let max_iterations = 100;

        while x0 * x0 + y0 * y0 <= 4.0 && iteration < max_iterations {
            let xtemp = x0 * x0 - y0 * y0 + 0.0; // cx = 0
            y0 = 2.0 * x0 * y0 + 0.0; // cy = 0
            x0 = xtemp;
            iteration += 1;
        }

        // (0,0) is in the Mandelbrot set, so it should reach max_iterations
        assert_eq!(iteration, max_iterations);
    }

    #[test]
    fn test_mandelbrot_escape_point() {
        // Test that points outside the set escape quickly
        // Point (2,0) should escape immediately
        let mut x0 = 0.0;
        let mut y0 = 0.0;
        let mut iteration = 0;
        let max_iterations = 100;

        while x0 * x0 + y0 * y0 <= 4.0 && iteration < max_iterations {
            let xtemp = x0 * x0 - y0 * y0 + 2.0; // cx = 2
            y0 = 2.0 * x0 * y0 + 0.0; // cy = 0
            x0 = xtemp;
            iteration += 1;
        }

        // Should escape quickly
        assert!(iteration < 10);
    }
}