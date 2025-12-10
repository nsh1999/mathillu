use std::path::Path;
use serde::{Deserialize, Serialize};

/// Configuration structure for saving/loading parameters.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub output_path: String,
    pub bands: u32,
    pub center_x: f64,
    pub center_y: f64,
    pub zoom: f64,
    pub end_center_x: Option<f64>,
    pub end_center_y: Option<f64>,
    pub end_zoom: Option<f64>,
    pub fps: f64,
    pub duration: f64,
    pub frames_dir: String,
    pub font_path: String,
    pub zoom_text_x: i32,
    pub zoom_text_y: i32,
    pub zoom_font_size: f32,
    pub function: String,
}

pub fn load_config(args: &mut crate::parameters::Args, config_path: Option<String>) {
    if let Some(config_path) = config_path {
        if let Ok(config_content) = std::fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str::<Config>(&config_content) {
                // When config is provided, use only config values, ignore command line
                args.width = config.width;
                args.height = config.height;
                args.max_iterations = config.max_iterations;
                args.output_path = Some(config.output_path);
                args.bands = config.bands;
                args.center_x = config.center_x;
                args.center_y = config.center_y;
                args.zoom = config.zoom;
                args.end_center_x = config.end_center_x;
                args.end_center_y = config.end_center_y;
                args.end_zoom = config.end_zoom;
                args.fps = config.fps;
                args.duration = config.duration;
                args.frames_dir = config.frames_dir;
                args.font_path = config.font_path;
                args.zoom_text_x = config.zoom_text_x;
                args.zoom_text_y = config.zoom_text_y;
                args.zoom_font_size = config.zoom_font_size;
                args.function = config.function;
            } else {
                eprintln!("Failed to parse config file: {}", config_path);
                std::process::exit(1);
            }
        } else {
            eprintln!("Failed to read config file: {}", config_path);
            std::process::exit(1);
        }
    }
}

pub fn save_config(args: &crate::parameters::Args, output_path: &str) {
    // Save config only if not loaded from config
    if args.config.is_none() {
        let config = Config {
            width: args.width,
            height: args.height,
            max_iterations: args.max_iterations,
            output_path: output_path.to_string(),
            bands: args.bands,
            center_x: args.center_x,
            center_y: args.center_y,
            zoom: args.zoom,
            end_center_x: args.end_center_x,
            end_center_y: args.end_center_y,
            end_zoom: args.end_zoom,
            fps: args.fps,
            duration: args.duration,
            frames_dir: args.frames_dir.clone(),
            font_path: args.font_path.clone(),
            zoom_text_x: args.zoom_text_x,
            zoom_text_y: args.zoom_text_y,
            zoom_font_size: args.zoom_font_size,
            function: args.function.clone(),
        };
        let config_toml = toml::to_string(&config).unwrap();
        let config_path = Path::new(&output_path).with_extension("conf").to_string_lossy().to_string();
        if let Err(e) = std::fs::write(&config_path, config_toml) {
            eprintln!("Failed to save config to {}: {}", config_path, e);
        } else {
            println!("Config saved to {}", config_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::Args;
    use std::fs;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: "test.png".to_string(),
            bands: 16,
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
            end_center_x: Some(-0.7),
            end_center_y: Some(0.1),
            end_zoom: Some(2.0),
            fps: 30.0,
            duration: 10.0,
            frames_dir: "frames".to_string(),
            font_path: "/System/Library/Fonts/Helvetica.ttc".to_string(),
            zoom_text_x: 10,
            zoom_text_y: 110,
            zoom_font_size: 20.0,
            function: "mandelbrot".to_string(),
        };

        let toml_string = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_string).unwrap();

        assert_eq!(config.width, deserialized.width);
        assert_eq!(config.height, deserialized.height);
        assert_eq!(config.max_iterations, deserialized.max_iterations);
        assert_eq!(config.output_path, deserialized.output_path);
        assert_eq!(config.bands, deserialized.bands);
        assert_eq!(config.center_x, deserialized.center_x);
        assert_eq!(config.center_y, deserialized.center_y);
        assert_eq!(config.zoom, deserialized.zoom);
        assert_eq!(config.end_center_x, deserialized.end_center_x);
        assert_eq!(config.end_center_y, deserialized.end_center_y);
        assert_eq!(config.end_zoom, deserialized.end_zoom);
        assert_eq!(config.fps, deserialized.fps);
        assert_eq!(config.duration, deserialized.duration);
        assert_eq!(config.frames_dir, deserialized.frames_dir);
        assert_eq!(config.font_path, deserialized.font_path);
        assert_eq!(config.zoom_text_x, deserialized.zoom_text_x);
        assert_eq!(config.zoom_text_y, deserialized.zoom_text_y);
        assert_eq!(config.zoom_font_size, deserialized.zoom_font_size);
        assert_eq!(config.function, deserialized.function);
    }

    #[test]
    fn test_load_config() {
        let config_content = r#"
width = 1024
height = 768
max_iterations = 2000
output_path = "loaded.png"
bands = 32
center_x = -0.75
center_y = 0.1
zoom = 2.0
end_center_x = -0.8
end_center_y = 0.2
end_zoom = 4.0
fps = 60.0
duration = 15.0
frames_dir = "test_frames"
font_path = "/test/font.ttf"
zoom_text_x = 20
zoom_text_y = 150
zoom_font_size = 24.0
function = "schrodinger"
"#;

        let temp_file = "/tmp/test_config.toml";
        fs::write(temp_file, config_content).unwrap();

        let mut args = Args {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: Some("original.png".to_string()),
            config: Some(temp_file.to_string()),
            bands: 16,
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
            end_center_x: None,
            end_center_y: None,
            end_zoom: None,
            fps: 30.0,
            duration: 10.0,
            frames_dir: "frames".to_string(),
            font_path: "/System/Library/Fonts/Helvetica.ttc".to_string(),
            zoom_text_x: 10,
            zoom_text_y: 110,
            zoom_font_size: 20.0,
            function: "mandelbrot".to_string(),
        };

        load_config(&mut args, Some(temp_file.to_string()));

        assert_eq!(args.width, 1024);
        assert_eq!(args.height, 768);
        assert_eq!(args.max_iterations, 2000);
        assert_eq!(args.output_path, Some("loaded.png".to_string()));
        assert_eq!(args.bands, 32);
        assert_eq!(args.center_x, -0.75);
        assert_eq!(args.center_y, 0.1);
        assert_eq!(args.zoom, 2.0);
        assert_eq!(args.end_center_x, Some(-0.8));
        assert_eq!(args.end_center_y, Some(0.2));
        assert_eq!(args.end_zoom, Some(4.0));
        assert_eq!(args.fps, 60.0);
        assert_eq!(args.duration, 15.0);
        assert_eq!(args.frames_dir, "test_frames");
        assert_eq!(args.font_path, "/test/font.ttf");
        assert_eq!(args.zoom_text_x, 20);
        assert_eq!(args.zoom_text_y, 150);
        assert_eq!(args.zoom_font_size, 24.0);
        assert_eq!(args.function, "schrodinger");

        fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_save_config() {
        let args = Args {
            width: 1200,
            height: 900,
            max_iterations: 1500,
            output_path: Some("output.png".to_string()),
            config: None, // Should save config since config is None
            bands: 24,
            center_x: -0.6,
            center_y: 0.05,
            zoom: 1.5,
            end_center_x: Some(-0.65),
            end_center_y: Some(0.1),
            end_zoom: Some(3.0),
            fps: 45.0,
            duration: 12.0,
            frames_dir: "my_frames".to_string(),
            font_path: "/custom/font.ttf".to_string(),
            zoom_text_x: 15,
            zoom_text_y: 120,
            zoom_font_size: 22.0,
            function: "mandelbrot".to_string(),
        };

        let temp_output = "/tmp/test_output.png";
        save_config(&args, temp_output);

        let config_path = "/tmp/test_output.conf";
        assert!(fs::metadata(config_path).is_ok());

        let config_content = fs::read_to_string(config_path).unwrap();
        let config: Config = toml::from_str(&config_content).unwrap();

        assert_eq!(config.width, 1200);
        assert_eq!(config.height, 900);
        assert_eq!(config.max_iterations, 1500);
        assert_eq!(config.output_path, "/tmp/test_output.png");
        assert_eq!(config.bands, 24);
        assert_eq!(config.center_x, -0.6);
        assert_eq!(config.center_y, 0.05);
        assert_eq!(config.zoom, 1.5);
        assert_eq!(config.end_center_x, Some(-0.65));
        assert_eq!(config.end_center_y, Some(0.1));
        assert_eq!(config.end_zoom, Some(3.0));
        assert_eq!(config.fps, 45.0);
        assert_eq!(config.duration, 12.0);
        assert_eq!(config.frames_dir, "my_frames");
        assert_eq!(config.font_path, "/custom/font.ttf");
        assert_eq!(config.zoom_text_x, 15);
        assert_eq!(config.zoom_text_y, 120);
        assert_eq!(config.zoom_font_size, 22.0);
        assert_eq!(config.function, "mandelbrot");

        fs::remove_file(config_path).ok();
    }

    #[test]
    fn test_save_config_with_existing_config() {
        let args = Args {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: Some("output.png".to_string()),
            config: Some("existing.conf".to_string()), // Should NOT save config
            bands: 16,
            center_x: -0.5,
            center_y: 0.0,
            zoom: 1.0,
            end_center_x: None,
            end_center_y: None,
            end_zoom: None,
            fps: 30.0,
            duration: 10.0,
            frames_dir: "frames".to_string(),
            font_path: "/System/Library/Fonts/Helvetica.ttc".to_string(),
            zoom_text_x: 10,
            zoom_text_y: 110,
            zoom_font_size: 20.0,
            function: "mandelbrot".to_string(),
        };

        let temp_output = "/tmp/test_output2.png";
        save_config(&args, temp_output);

        let config_path = "/tmp/test_output2.conf";
        // Should not exist because config was provided
        assert!(fs::metadata(config_path).is_err());
    }
}