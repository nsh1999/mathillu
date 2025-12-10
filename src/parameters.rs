use clap::{Parser, CommandFactory, FromArgMatches};

/// CLI argument parser.
#[derive(Parser)]
pub struct Args {
    /// Width of the output image.
    #[clap(short, long, default_value = "800")]
    pub width: u32,

    /// Height of the output image.
    #[clap(short, long, default_value = "600")]
    pub height: u32,

    /// Maximum number of iterations for the Mandelbrot calculation.
    #[clap(short, long, default_value = "1000")]
    pub max_iterations: u32,

    /// Path to save the generated image or video.
    #[clap(short, long)]
    pub output_path: Option<String>,

    /// Path to config file to load parameters from.
    #[clap(long)]
    pub config: Option<String>,

    /// Number of color bands for the iteration-based coloring.
    #[clap(short, long, default_value = "16")]
    pub bands: u32,

    /// X coordinate of the center of the Mandelbrot set.
    #[clap(long, default_value = "-0.5")]
    pub center_x: f64,

    /// Y coordinate of the center of the Mandelbrot set.
    #[clap(long, default_value = "0.0")]
    pub center_y: f64,

    /// Zoom level (1.0 = default, higher values zoom in).
    #[clap(long, default_value = "1.0")]
    pub zoom: f64,

    /// End X coordinate for video transition.
    #[clap(long)]
    pub end_center_x: Option<f64>,

    /// End Y coordinate for video transition.
    #[clap(long)]
    pub end_center_y: Option<f64>,

    /// End zoom level for video transition.
    #[clap(long)]
    pub end_zoom: Option<f64>,

    /// Frames per second for video.
    #[clap(long, default_value = "30.0")]
    pub fps: f64,

    /// Duration of video in seconds.
    #[clap(long, default_value = "10.0")]
    pub duration: f64,

    /// Directory to save video frames.
    #[clap(long, default_value = "frames")]
    pub frames_dir: String,

    /// Path to the font file for zoom text.
    #[clap(long, default_value = "/System/Library/Fonts/Helvetica.ttc")]
    pub font_path: String,

    /// X position of the zoom text.
    #[clap(long, default_value = "10")]
    pub zoom_text_x: i32,

    /// Y position of the zoom text.
    #[clap(long, default_value = "110")]
    pub zoom_text_y: i32,

    /// Font size for the zoom text.
    #[clap(long, default_value = "20.0")]
    pub zoom_font_size: f32,

    /// Function to generate: 'mandelbrot' or 'schrodinger'.
    #[clap(long, default_value = "mandelbrot")]
    pub function: String,
}

pub fn prepare_parameters() -> (Args, String) {
    let matches = Args::command().get_matches();
    let mut args = Args::from_arg_matches(&matches).unwrap();

    // Load config if provided
    let config_path = args.config.clone();
    crate::config::load_config(&mut args, config_path);

    // Ensure output_path is set
    let output_path = match args.output_path {
        Some(p) => p,
        None => {
            eprintln!("output_path is required");
            std::process::exit(1);
        }
    };
    args.output_path = Some(output_path.clone()); // for consistency, but not needed

    (args, output_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_default_values() {
        let args = Args {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: Some("test.png".to_string()),
            config: None,
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

        assert_eq!(args.width, 800);
        assert_eq!(args.height, 600);
        assert_eq!(args.max_iterations, 1000);
        assert_eq!(args.output_path, Some("test.png".to_string()));
        assert_eq!(args.config, None);
        assert_eq!(args.bands, 16);
        assert_eq!(args.center_x, -0.5);
        assert_eq!(args.center_y, 0.0);
        assert_eq!(args.zoom, 1.0);
        assert_eq!(args.end_center_x, None);
        assert_eq!(args.end_center_y, None);
        assert_eq!(args.end_zoom, None);
        assert_eq!(args.fps, 30.0);
        assert_eq!(args.duration, 10.0);
        assert_eq!(args.frames_dir, "frames");
        assert_eq!(args.font_path, "/System/Library/Fonts/Helvetica.ttc");
        assert_eq!(args.zoom_text_x, 10);
        assert_eq!(args.zoom_text_y, 110);
        assert_eq!(args.zoom_font_size, 20.0);
        assert_eq!(args.function, "mandelbrot");
    }

    #[test]
    fn test_args_custom_values() {
        let args = Args {
            width: 1024,
            height: 768,
            max_iterations: 2000,
            output_path: Some("custom.png".to_string()),
            config: Some("config.conf".to_string()),
            bands: 32,
            center_x: -0.75,
            center_y: 0.1,
            zoom: 2.0,
            end_center_x: Some(-0.8),
            end_center_y: Some(0.2),
            end_zoom: Some(4.0),
            fps: 60.0,
            duration: 15.0,
            frames_dir: "custom_frames".to_string(),
            font_path: "/custom/font.ttf".to_string(),
            zoom_text_x: 20,
            zoom_text_y: 150,
            zoom_font_size: 24.0,
            function: "schrodinger".to_string(),
        };

        assert_eq!(args.width, 1024);
        assert_eq!(args.height, 768);
        assert_eq!(args.max_iterations, 2000);
        assert_eq!(args.output_path, Some("custom.png".to_string()));
        assert_eq!(args.config, Some("config.conf".to_string()));
        assert_eq!(args.bands, 32);
        assert_eq!(args.center_x, -0.75);
        assert_eq!(args.center_y, 0.1);
        assert_eq!(args.zoom, 2.0);
        assert_eq!(args.end_center_x, Some(-0.8));
        assert_eq!(args.end_center_y, Some(0.2));
        assert_eq!(args.end_zoom, Some(4.0));
        assert_eq!(args.fps, 60.0);
        assert_eq!(args.duration, 15.0);
        assert_eq!(args.frames_dir, "custom_frames");
        assert_eq!(args.font_path, "/custom/font.ttf");
        assert_eq!(args.zoom_text_x, 20);
        assert_eq!(args.zoom_text_y, 150);
        assert_eq!(args.zoom_font_size, 24.0);
        assert_eq!(args.function, "schrodinger");
    }
}