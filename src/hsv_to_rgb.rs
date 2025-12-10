use image::Rgba;

/// Converts HSV to RGB.
///
/// # Arguments
///
/// * `h` - Hue (0.0-360.0)
/// * `s` - Saturation (0-255)
/// * `v` - Value/Brightness (0-255)
pub fn hsv_to_rgb(h: f32, s: u8, v: u8) -> Rgba<u8> {
    let hi = (h / 60.0) as usize % 6;
    let f = (h % 60.0) / 60.0;
    let p = v as f32 * (255.0 - s as f32) / 255.0;
    let q = v as f32 * (255.0 - (s as f32 * f)) / 255.0;
    let t = v as f32 * (255.0 - (s as f32 * (1.0 - f))) / 255.0;

    match hi {
        0 => Rgba([v, (t) as u8, (p) as u8, 255]),
        1 => Rgba([(q) as u8, v, (p) as u8, 255]),
        2 => Rgba([(p) as u8, v, (t) as u8, 255]),
        3 => Rgba([(p) as u8, (q) as u8, v, 255]),
        4 => Rgba([(t) as u8, (p) as u8, v, 255]),
        _ => Rgba([v, (p) as u8, (q) as u8, 255]), // case 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv_to_rgb_red() {
        // Hue 0 should give red
        let color = hsv_to_rgb(0.0, 255, 255);
        assert_eq!(color, Rgba([255, 0, 0, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_green() {
        // Hue 120 should give green
        let color = hsv_to_rgb(120.0, 255, 255);
        assert_eq!(color, Rgba([0, 255, 0, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_blue() {
        // Hue 240 should give blue
        let color = hsv_to_rgb(240.0, 255, 255);
        assert_eq!(color, Rgba([0, 0, 255, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_yellow() {
        // Hue 60 should give yellow
        let color = hsv_to_rgb(60.0, 255, 255);
        assert_eq!(color, Rgba([255, 255, 0, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_cyan() {
        // Hue 180 should give cyan
        let color = hsv_to_rgb(180.0, 255, 255);
        assert_eq!(color, Rgba([0, 255, 255, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_magenta() {
        // Hue 300 should give magenta
        let color = hsv_to_rgb(300.0, 255, 255);
        assert_eq!(color, Rgba([255, 0, 255, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_black() {
        // Value 0 should give black regardless of hue/saturation
        let color = hsv_to_rgb(120.0, 255, 0);
        assert_eq!(color, Rgba([0, 0, 0, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_white() {
        // Saturation 0 should give white/gray
        let color = hsv_to_rgb(120.0, 0, 255);
        assert_eq!(color, Rgba([255, 255, 255, 255]));
    }

    #[test]
    fn test_hsv_to_rgb_gray() {
        // Saturation 0 with lower value should give gray
        let color = hsv_to_rgb(120.0, 0, 128);
        assert_eq!(color, Rgba([128, 128, 128, 255]));
    }
}