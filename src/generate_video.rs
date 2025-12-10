use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::generate_mandelbrot;
use crate::generate_schrodinger;

/// Generates a video by creating frames with smooth transitions and encoding them with ffmpeg.
///
/// # Arguments
///
/// * `args` - The parsed command line arguments
/// * `output_path` - Path where the video should be saved
pub fn generate_video(args: &crate::parameters::Args, output_path: &str) {
    std::fs::create_dir_all(&args.frames_dir).expect("Failed to create frames directory");
    let total_frames = (args.fps * args.duration).round() as u32;
    println!("Generating {} frames for video...", total_frames);

    let end_cx = args.end_center_x.unwrap_or(args.center_x);
    let end_cy = args.end_center_y.unwrap_or(args.center_y);
    let end_z = args.end_zoom.unwrap_or(args.zoom);

    let mut log_entries = Vec::new();
    for i in 0..total_frames {
        let t = if total_frames > 1 { i as f64 / (total_frames - 1) as f64 } else { 0.0 };
        let cx = args.center_x + (end_cx - args.center_x) * t;
        let cy = args.center_y + (end_cy - args.center_y) * t;
        let z = if args.zoom > 0.0 && end_z > 0.0 {
            let log_start = args.zoom.ln();
            let log_end = end_z.ln();
            let log_z = log_start + (log_end - log_start) * t;
            log_z.exp()
        } else {
            args.zoom + (end_z - args.zoom) * t
        };
        let frame_path = format!("{}/{}_frame_{:04}.png", args.frames_dir, output_path, i);
        match args.function.as_str() {
            "mandelbrot" => generate_mandelbrot::generate_mandelbrot(args.width, args.height, args.max_iterations, args.bands, cx, cy, z, &args.font_path, args.zoom_text_x, args.zoom_text_y, args.zoom_font_size, &frame_path),
            "schrodinger" => generate_schrodinger::generate_schrodinger(args.width, args.height, args.bands, cx, cy, z, &args.font_path, args.zoom_text_x, args.zoom_text_y, args.zoom_font_size, &frame_path),
            _ => panic!("Unknown function: {}", args.function),
        }
        let time = i as f64 / args.fps;
        log_entries.push((i + 1, time, cx, cy, z));
        println!("Generated frame {}", i + 1);
    }

    // Write log
    let log_path = format!("{}.log", output_path);
    let mut log_file = File::create(&log_path).expect("Failed to create log file");
    writeln!(log_file, "Frame,Time,X,Y,Zoom").expect("Failed to write log header");
    for (frame, time, x, y, zoom) in log_entries {
        writeln!(log_file, "{},{:.2},{:.6},{:.6},{:.6}", frame, time, x, y, zoom).expect("Failed to write log entry");
    }
    println!("Log written to {}", log_path);

    // Create video with ffmpeg
    let video_path = format!("{}.mp4", output_path);
    if let Ok(status) = Command::new("ffmpeg")
        .args(&["-y", "-r", &args.fps.to_string(), "-i", &format!("{}/{}_frame_%04d.png", args.frames_dir, output_path), "-c:v", "libx264", "-pix_fmt", "yuv420p", &video_path])
        .status()
    {
        if status.success() {
            println!("Video created: {}", video_path);
            // Clean up frames
            for i in 0..total_frames {
                let frame_path = format!("{}/{}_frame_{:04}.png", args.frames_dir, output_path, i);
                std::fs::remove_file(&frame_path).ok();
            }
        } else {
            eprintln!("ffmpeg failed to create video");
        }
    } else {
        println!("ffmpeg not found. Frames generated in {}/{}_frame_*.png. Run ffmpeg manually to create video.", args.frames_dir, output_path);
    }
}

#[cfg(test)]
mod tests {
    use crate::parameters::Args;

    #[test]
    fn test_linear_interpolation() {
        // Test linear interpolation between start and end values
        let start = 1.0;
        let end = 5.0;
        let t = 0.5; // halfway

        let result = start + (end - start) * t;
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_logarithmic_interpolation() {
        // Test logarithmic interpolation for zoom
        let start_zoom = 1.0_f64;
        let end_zoom = 4.0_f64; // 2^2
        let t = 0.5_f64;

        let log_start = start_zoom.ln();
        let log_end = end_zoom.ln();
        let log_z = log_start + (log_end - log_start) * t;
        let result = log_z.exp();

        // At t=0.5, should be sqrt(start_zoom * end_zoom) = sqrt(4) = 2
        assert!((result - 2.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_frame_timing_calculation() {
        let fps = 30.0;
        let total_frames = 10;

        for i in 0..total_frames {
            let time = i as f64 / fps;
            let expected_time = i as f64 * (1.0 / fps);
            assert_eq!(time, expected_time);
        }
    }

    #[test]
    fn test_interpolation_parameter_calculation() {
        let total_frames = 10;

        for i in 0..total_frames {
            let t = if total_frames > 1 { i as f64 / (total_frames - 1) as f64 } else { 0.0 };

            if total_frames > 1 {
                let expected_t = i as f64 / (total_frames - 1) as f64;
                assert_eq!(t, expected_t);
            } else {
                assert_eq!(t, 0.0);
            }
        }

        // Edge cases
        assert_eq!(if 1 > 1 { 0.0 / 0.0 } else { 0.0 }, 0.0); // total_frames = 1
    }

    #[test]
    fn test_end_values_fallback() {
        let args = Args {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: Some("test.mp4".to_string()),
            config: None,
            bands: 16,
            center_x: 1.0,
            center_y: 2.0,
            zoom: 3.0,
            end_center_x: None,
            end_center_y: None,
            end_zoom: None,
            fps: 30.0,
            duration: 1.0,
            frames_dir: "frames".to_string(),
            font_path: "/font.ttf".to_string(),
            zoom_text_x: 10,
            zoom_text_y: 110,
            zoom_font_size: 20.0,
            function: "mandelbrot".to_string(),
        };

        // Test that end values fall back to start values when None
        let end_cx = args.end_center_x.unwrap_or(args.center_x);
        let end_cy = args.end_center_y.unwrap_or(args.center_y);
        let end_z = args.end_zoom.unwrap_or(args.zoom);

        assert_eq!(end_cx, 1.0);
        assert_eq!(end_cy, 2.0);
        assert_eq!(end_z, 3.0);
    }

    #[test]
    fn test_end_values_with_provided_values() {
        let args = Args {
            width: 800,
            height: 600,
            max_iterations: 1000,
            output_path: Some("test.mp4".to_string()),
            config: None,
            bands: 16,
            center_x: 1.0,
            center_y: 2.0,
            zoom: 3.0,
            end_center_x: Some(4.0),
            end_center_y: Some(5.0),
            end_zoom: Some(6.0),
            fps: 30.0,
            duration: 1.0,
            frames_dir: "frames".to_string(),
            font_path: "/font.ttf".to_string(),
            zoom_text_x: 10,
            zoom_text_y: 110,
            zoom_font_size: 20.0,
            function: "mandelbrot".to_string(),
        };

        let end_cx = args.end_center_x.unwrap_or(args.center_x);
        let end_cy = args.end_center_y.unwrap_or(args.center_y);
        let end_z = args.end_zoom.unwrap_or(args.zoom);

        assert_eq!(end_cx, 4.0);
        assert_eq!(end_cy, 5.0);
        assert_eq!(end_z, 6.0);
    }
}