mod generate_mandelbrot;
mod generate_schrodinger;
mod generate_video;
mod hsv_to_rgb;
mod config;
mod parameters;

fn main() {
    let (args, output_path) = parameters::prepare_parameters();

    let is_video = args.end_center_x.is_some() || args.end_center_y.is_some() || args.end_zoom.is_some();

    if is_video {
        generate_video::generate_video(&args, &output_path);
    } else {
        match args.function.as_str() {
            "mandelbrot" => generate_mandelbrot::generate_mandelbrot(args.width, args.height, args.max_iterations, args.bands, args.center_x, args.center_y, args.zoom, &args.font_path, args.zoom_text_x, args.zoom_text_y, args.zoom_font_size, &output_path),
            "schrodinger" => generate_schrodinger::generate_schrodinger(args.width, args.height, args.bands, args.center_x, args.center_y, args.zoom, &args.font_path, args.zoom_text_x, args.zoom_text_y, args.zoom_font_size, &output_path),
            _ => panic!("Unknown function: {}", args.function),
        }
    }

    // Save config only if not loaded from config
    config::save_config(&args, &output_path);
}