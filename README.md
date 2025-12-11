# Mathillu

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust application for generating stunning mathematical visualizations, including Mandelbrot sets and Schrödinger equation probability density plots. Create beautiful images and smooth animated videos of complex mathematical phenomena.

## ✨ Features

- **Mandelbrot Set Visualization**: Generate intricate fractal images with customizable parameters
- **Schrödinger Equation**: Visualize quantum wave functions and probability densities
- **Video Generation**: Create smooth animated transitions between different views with easing
- **Configurable Parameters**: Extensive customization options for colors, zoom, positioning, and more
- **High Performance**: Written in Rust for maximum speed and memory efficiency
- **Config File Support**: Save and load parameter configurations
- **Cross-platform**: Works on Windows, macOS, and Linux

## [Sample video on YouTube](https://youtu.be/WQlP19UGV8U)

## 🚀 Quick Start

### Installation

#### Option 1: Download Pre-built Binary

Download the latest release from the [Releases](https://github.com/nsh1999/mathillu/releases) page.

#### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/nsh1999/mathillu.git
cd mathillu

# Build in release mode
cargo build --release

# The binary will be available at target/release/mathillu
```

### Basic Usage

Generate a Mandelbrot set image:

```bash
./mathillu --output-path mandelbrot.png
```

Generate a Schrödinger equation visualization:

```bash
./mathillu --function schrodinger --output-path schrodinger.png
```

Create an animated video with smooth zoom:

```bash
./mathillu --end-zoom 10.0 --output-path zoom_animation.mp4
```

## 📖 Usage Examples

### Image Generation

#### High-resolution Mandelbrot set
```bash
./mathillu --width 2048 --height 1536 --max-iterations 2000 --output-path mandelbrot_4k.png
```

#### Custom color scheme and positioning
```bash
./mathillu --center-x 200 --center-y -100 --zoom 2.0 --bands 32 --output-path custom_mandelbrot.png
```

#### Schrödinger equation visualization
```bash
./mathillu --function schrodinger --width 1200 --height 900 --zoom 1.5 --output-path quantum_wave.png
```

### Video Generation

#### Smooth zoom animation
```bash
./mathillu --center-x 0 --center-y 0 --zoom 1.0 \
          --end-center-x 300 --end-center-y -200 --end-zoom 100.0 \
          --duration 15.0 --fps 60.0 \
          --output-path mandelbrot_zoom.mp4
```

#### Position and zoom transition
```bash
./mathillu --function schrodinger \
          --center-x 0 --center-y 0 --zoom 1.0 \
          --end-center-x 200 --end-center-y 150 --end-zoom 3.0 \
          --output-path quantum_transition.mp4
```

**Note:** Video transitions use smooth easing (smoothstep) for natural-looking animations. Center movements and zoom changes accelerate smoothly through the middle of the transition and decelerate at the start and end.

### Configuration Files

Save parameters to a config file:

```bash
./mathillu --width 1600 --height 1200 --max-iterations 1500 --bands 24 --output-path config_example.png
# This creates config_example.conf
```

Load parameters from a config file:

```bash
./mathillu --config config_example.conf --output-path loaded_example.png
```

## ⚙️ Configuration Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--width` | `-w` | 800 | Width of the output image |
| `--height` | `-h` | 600 | Height of the output image |
| `--max-iterations` | `-m` | 1000 | Maximum iterations for Mandelbrot calculation |
| `--output-path` | `-o` | Required | Path to save the generated image/video |
| `--config` | | | Path to config file to load parameters from |
| `--bands` | `-b` | 16 | Number of color bands for coloring |
| `--center-x` | | 0.0 | X center offset in pixels from image center |
| `--center-y` | | 0.0 | Y center offset in pixels from image center |
| `--zoom` | | 1.0 | Zoom level (higher = more zoomed in) |
| `--function` | | mandelbrot | Function to generate: 'mandelbrot' or 'schrodinger' |
| `--end-center-x` | | | End X coordinate for video transition |
| `--end-center-y` | | | End Y coordinate for video transition |
| `--end-zoom` | | | End zoom level for video transition |
| `--fps` | | 30.0 | Frames per second for video |
| `--duration` | | 10.0 | Duration of video in seconds |
| `--frames-dir` | | frames | Directory to save video frames |
| `--font-path` | | /System/Library/Fonts/Helvetica.ttc | Path to font file for zoom text |
| `--zoom-text-x` | | 10 | X position of zoom text |
| `--zoom-text-y` | | 110 | Y position of zoom text |
| `--zoom-font-size` | | 20.0 | Font size for zoom text |

## 🏗️ Building from Source

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Cargo (comes with Rust)

### Build Commands

```bash
# Debug build
cargo build

# Release build (recommended for performance)
cargo build --release

# Run tests
cargo test

# Run with optimizations
cargo run --release -- [arguments]
```

### Dependencies

The project uses the following main dependencies:

- `image` - Image processing and I/O
- `clap` - Command-line argument parsing
- `imageproc` - Image processing utilities
- `rusttype` - Font rendering
- `serde` & `toml` - Configuration file handling

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

The test suite includes:
- Unit tests for all mathematical functions
- Configuration file serialization/deserialization
- Color conversion accuracy
- Parameter validation
- File I/O operations

## 📊 Mathematical Background

### Coordinate System
The application uses a pixel-based coordinate system where:
- `center_x = 0`, `center_y = 0` centers the view at (0,0) in the complex plane
- Positive `center_x` shifts the view right, negative shifts left
- Positive `center_y` shifts the view down, negative shifts up
- Values are in pixel units relative to the image center

### Mandelbrot Set
The Mandelbrot set is a fractal defined by the equation:
```
z₀ = 0
zₙ₊₁ = zₙ² + c
```

Where `c` is a complex number. Points that remain bounded as n→∞ belong to the set.

### Schrödinger Equation
Visualizes the probability density |ψ|² of a 2D Gaussian wave packet, representing quantum particle behavior.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Fork the repository
2. Clone your fork: `git clone https://github.com/nsh1999/mathillu.git`
3. Create a feature branch: `git checkout -b feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -am 'Add some feature'`
7. Push to the branch: `git push origin feature-name`
8. Submit a pull request

### Code Style

This project follows Rust's standard formatting and linting guidelines:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses the excellent [image](https://crates.io/crates/image) crate for image processing
- Inspired by the beauty of mathematical visualization

## 📞 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/nsh1999/mathillu/issues) page
2. Create a new issue with detailed information
3. Include your operating system, Rust version, and command used

---

*Made with ❤️ and mathematics*</content>
<parameter name="filePath">README.md