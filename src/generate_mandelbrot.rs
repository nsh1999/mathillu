use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

use crate::hsv_to_rgb::hsv_to_rgb;

/// Base center coordinates for the Mandelbrot set
const BASE_CENTER_X: f64 = 0.0;
const BASE_CENTER_Y: f64 = 0.0;

/// Calculates the Mandelbrot iteration count for a given point in the complex plane.
///
/// # Arguments
///
/// * `cx` - Real part of the complex number.
/// * `cy` - Imaginary part of the complex number.
/// * `max_iterations` - Maximum number of iterations to perform.
///
/// # Returns
///
/// The number of iterations before the point escapes the Mandelbrot set,
/// or `max_iterations` if it doesn't escape within the limit.
fn calc_mandelbrot(cx: f64, cy: f64, max_iterations: u32) -> u32 {
    let mut x0 = 0.0;
    let mut y0 = 0.0;
    let mut iteration = 0;

    while x0 * x0 + y0 * y0 <= 4.0 && iteration < max_iterations {
        let xtemp = x0 * x0 - y0 * y0 + cx;
        y0 = 2.0 * x0 * y0 + cy;
        x0 = xtemp;
        iteration += 1;
    }

    iteration
}

/// Maps integer image coordinates to floating-point virtual image coordinates.
///
/// # Arguments
///
/// * `x` - X coordinate in the image (0 to width-1).
/// * `y` - Y coordinate in the image (0 to height-1).
/// * `width` - Width of the image.
/// * `height` - Height of the image.
/// * `zoom` - Zoom level.
/// * `center_x` - X center offset in pixels from base center.
/// * `center_y` - Y center offset in pixels from base center.
/// * `m_size` - Size of the mathematical space (square).
///
/// # Returns
///
/// A tuple (cx, cy) representing the complex plane coordinates.
fn coordinate_mapper(x: u32, y: u32, width: u32, height: u32, zoom: f64, center_x: f64, center_y: f64, m_size: f64) -> (f64, f64) {
    // Calculate scales based on output dimensions and zoom level
    // Base ranges for zoom = 1.0 (full Mandelbrot view)
    let base_range = m_size; // Square mathematical space
    
    // Handle zoom: positive = zoom in, negative = zoom out
    let zoom_factor = if zoom > 0.0 { 1.0 / zoom } else { zoom.abs().max(0.1) };
    let (scale_x, scale_y) = if width > height {
        // Wide image: base on height, extend width
        let base_scale = if zoom >= 0.0 { base_range * zoom_factor } else { base_range / zoom_factor };
        (base_scale * (width as f64 / height as f64), base_scale)
    } else if height > width {
        // Tall image: base on width, extend height  
        let base_scale = if zoom >= 0.0 { base_range * zoom_factor } else { base_range / zoom_factor };
        (base_scale, base_scale * (height as f64 / width as f64))
    } else {
        // Square image
        let base_scale = if zoom >= 0.0 { base_range * zoom_factor } else { base_range / zoom_factor };
        (base_scale, base_scale)
    };

    // Fixed units per pixel based on zoom=1.0 scale
    let fixed_zoom_factor = 1.0; // zoom=1.0
    let fixed_base_scale = if true { base_range * fixed_zoom_factor } else { base_range / fixed_zoom_factor }; // assuming zoom >=0
    let fixed_scale_x = if width > height {
        fixed_base_scale * (width as f64 / height as f64)
    } else if height > width {
        fixed_base_scale
    } else {
        fixed_base_scale
    };
    let fixed_scale_y = if width > height {
        fixed_base_scale
    } else if height > width {
        fixed_base_scale * (height as f64 / width as f64)
    } else {
        fixed_base_scale
    };
    let fixed_units_per_pixel_x = fixed_scale_x / width as f64;
    let fixed_units_per_pixel_y = fixed_scale_y / height as f64;

    // Convert center offsets from pixels to actual coordinates using fixed units
    let effective_center_x = BASE_CENTER_X + center_x;
    let effective_center_y = BASE_CENTER_Y + center_y;
    let actual_center_x = effective_center_x * fixed_units_per_pixel_x;
    let actual_center_y = effective_center_y * fixed_units_per_pixel_y;

    // Normalize pixel coordinates to -1 to 1
    let x_norm = (x as f64 / width as f64) * 2.0 - 1.0;
    let y_norm = (y as f64 / height as f64) * 2.0 - 1.0;

    // Map to complex plane
    let cx = x_norm * (scale_x / 2.0) + actual_center_x;
    let cy = y_norm * (scale_y / 2.0) + actual_center_y;

    (cx, cy)
}

/// Generates a Mandelbrot set image.
///
/// # Arguments
///
/// * `width` - Width of the output image.
/// * `height` - Height of the output image.
/// * `max_iterations` - Maximum number of iterations for the Mandelbrot calculation.
/// * `bands` - Number of color bands.
/// * `center_x` - X center coordinate (normalized -1 to 1).
/// * `center_y` - Y center coordinate (normalized -1 to 1).
/// * `zoom` - Zoom level.
/// * `m_size` - Size of the mathematical space (square).
/// * `font_path` - Path to font file.
/// * `zoom_text_x` - X position of zoom text.
/// * `zoom_text_y` - Y position of zoom text.
/// * `zoom_font_size` - Font size for zoom text.
/// * `output_path` - Path to save the generated image.
pub fn generate_mandelbrot(width: u32, height: u32, max_iterations: u32, bands: u32, center_x: f64, center_y: f64, zoom: f64, m_size: f64, font_path: &str, zoom_text_x: i32, zoom_text_y: i32, zoom_font_size: f32, output_path: &str) {
    // Validate zoom level
    let zoom = if zoom <= 0.0 { 1.0 } else { zoom };

    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (cx, cy) = coordinate_mapper(x, y, width, height, zoom, center_x, center_y, m_size);

        let iteration = calc_mandelbrot(cx, cy, max_iterations);

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
    let font_data = match std::fs::read(font_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read font file '{}': {}", font_path, e);
            eprintln!("Please ensure the font file exists and the path is correct.");
            std::process::exit(1);
        }
    };
    let font = Font::try_from_vec(font_data).expect("Failed to load font");
    let scale = Scale { x: zoom_font_size, y: zoom_font_size };
    let text = format!("ZOOM {:.1}", zoom);
    draw_text_mut(&mut imgbuf, Rgba([0, 0, 0, 255]), zoom_text_x, zoom_text_y, scale, &font, &text);

    imgbuf.save(output_path).unwrap_or_else(|e| {
        eprintln!("Failed to save image to '{}': {}", output_path, e);
        eprintln!("Please ensure the output directory exists and you have write permissions.");
        std::process::exit(1);
    });
}

/// Adds a coordinate grid to an image and saves it with "_grid" suffix.
///
/// # Arguments
///
/// * `input_path` - Path to the input PNG file.
///
/// # Returns
///
/// Result indicating success or failure.
pub fn add_grid_to_image(input_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?.to_rgba8();
    let (width, height) = img.dimensions();
    let mut img = img;

    // Draw vertical grid lines every 50 pixels
    for x in (0..width).step_by(50) {
        let line_index = x / 50;
        let is_center_line = x == (width / 2 / 50) * 50; // Check if this is the center line
        let line_width = if is_center_line { 5 } else { 3 };

        let color = if line_index % 2 == 0 {
            Rgba([0, 0, 255, 255]) // Blue for even indices
        } else {
            Rgba([0, 0, 0, 255]) // Black for odd indices
        };

        // Draw line_width-pixel wide vertical line
        for dx in 0..line_width {
            if x + dx < width {
                imageproc::drawing::draw_line_segment_mut(
                    &mut img,
                    (x as f32 + dx as f32, 0.0),
                    (x as f32 + dx as f32, height as f32 - 1.0),
                    color,
                );
            }
        }
    }

    // Draw horizontal grid lines every 50 pixels
    for y in (0..height).step_by(50) {
        let line_index = y / 50;
        let is_center_line = y == (height / 2 / 50) * 50; // Check if this is the center line
        let line_width = if is_center_line { 5 } else { 3 };

        let color = if line_index % 2 == 0 {
            Rgba([0, 0, 255, 255]) // Blue for even indices
        } else {
            Rgba([0, 0, 0, 255]) // Black for odd indices
        };

        // Draw line_width-pixel wide horizontal line
        for dy in 0..line_width {
            if y + dy < height {
                imageproc::drawing::draw_line_segment_mut(
                    &mut img,
                    (0.0, y as f32 + dy as f32),
                    (width as f32 - 1.0, y as f32 + dy as f32),
                    color,
                );
            }
        }
    }

    // Create output path
    let output_path = if input_path.ends_with(".png") {
        format!("{}_grid.png", &input_path[..input_path.len() - 4])
    } else {
        format!("{}_grid.png", input_path)
    };

    img.save(&output_path)?;
    println!("Grid image saved to: {}", output_path);
    Ok(())
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
            100, 100, 50, 8, 0.0, 0.0, 1.0, 10.0,
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
        let iteration = calc_mandelbrot(0.0, 0.0, 100);

        // (0,0) is in the Mandelbrot set, so it should reach max_iterations
        assert_eq!(iteration, 100);
    }

    #[test]
    fn test_mandelbrot_escape_point() {
        // Test that points outside the set escape quickly
        // Point (2,0) should escape immediately
        let iteration = calc_mandelbrot(2.0, 0.0, 100);

        // Should escape quickly
        assert!(iteration < 10);
    }
}