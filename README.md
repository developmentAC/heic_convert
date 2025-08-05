# HEIC to PNG/JPG Converter

A command-line tool written in Rust for converting HEIC (High Efficiency Image Container) files to PNG or JPG format. Neat-O!

Date: 5 August 2025


![logo](graphics/logo.png)

## Table of Contents
- [HEIC to PNG/JPG Converter](#heic-to-pngjpg-converter)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Build from Source](#build-from-source)
    - [Install globally (optional)](#install-globally-optional)
  - [Usage](#usage)
    - [Basic Usage](#basic-usage)
    - [Advanced Usage](#advanced-usage)
    - [Command-line Options](#command-line-options)
    - [Get Detailed Help](#get-detailed-help)
  - [How It Works](#how-it-works)
  - [Output](#output)
  - [Error Handling](#error-handling)
  - [Alternative Solutions](#alternative-solutions)
  - [Dependencies](#dependencies)
  - [Contributing](#contributing)
  - [License](#license)
  - [Troubleshooting](#troubleshooting)
    - ["HEIC format support is not available"](#heic-format-support-is-not-available)
    - ["Input file does not exist"](#input-file-does-not-exist)
    - ["Failed to save image"](#failed-to-save-image)
  - [Version History](#version-history)
    - [A Work In Progress](#a-work-in-progress)

## Features

- Convert HEIC files to PNG or JPG formats
- Command-line interface with flexible options
- Automatic output filename generation
- Support for custom output paths
- Comprehensive help with examples
- Fallback to external tools (ImageMagick, FFmpeg) if needed

## Installation

### Prerequisites

For HEIC conversion support, you need one of the following installed:

**Option 1: libheif (Recommended)**
```bash
# macOS
brew install libheif

# Ubuntu/Debian
sudo apt-get install libheif-dev

# Fedora/CentOS
sudo dnf install libheif-devel
```

**Option 2: ImageMagick**
```bash
# macOS
brew install imagemagick

# Ubuntu/Debian
sudo apt-get install imagemagick

# Fedora/CentOS
sudo dnf install ImageMagick
```

**Option 3: FFmpeg**
```bash
# macOS
brew install ffmpeg

# Ubuntu/Debian
sudo apt-get install ffmpeg

# Fedora/CentOS
sudo dnf install ffmpeg
```

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd heic2png

# Build the project
cargo build --release

# The binary will be available at ./target/release/heic2png
```

### Install globally (optional)
```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Convert HEIC to PNG (default format)
heic2png -i photo.heic

# Convert HEIC to JPG
heic2png -i photo.heic -f jpg

# Specify custom output filename
heic2png -i photo.heic -o converted_photo.png
```

### Advanced Usage

```bash
# Convert with custom output directory
heic2png -i /path/to/photo.heic -o /output/dir/converted.jpg -f jpg

# Convert multiple files (shell script)
for file in *.heic; do heic2png -i "$file" -f png; done
```

### Command-line Options

```
Options:
  -i, --input <FILE>     Input HEIC file path
  -o, --output <FILE>    Output file path (optional, will auto-generate if not provided)
  -f, --format <FORMAT>  Output format: png, jpg, jpeg [default: png]
      --bighelp          Show detailed help with examples
  -h, --help             Print help
  -V, --version          Print version
```

### Get Detailed Help

```bash
heic2png --bighelp
```

This shows comprehensive examples and usage patterns.

## How It Works

The tool attempts conversion in the following order:

1. **Native image crate support**: If the Rust `image` crate can handle HEIC files directly
2. **ImageMagick**: If `convert` command is available
3. **FFmpeg**: If `ffmpeg` command is available
4. **Error with suggestions**: If no conversion method is available

## Output

- If no output file is specified, the tool generates one based on the input filename
- Example: `photo.heic` → `photo.png` (or `photo.jpg` if JPG format is selected)
- The tool preserves the original directory unless a different output path is specified

## Error Handling

The tool provides helpful error messages and suggestions when:

- Input file doesn't exist
- Input file is not a HEIC/HEIF file
- No conversion tools are available
- Conversion fails

## Alternative Solutions

If this tool doesn't work for your setup, you can use:

**Command-line alternatives:**
```bash
# ImageMagick
convert input.heic output.png

# FFmpeg
ffmpeg -i input.heic output.png
```

**GUI alternatives:**
- macOS Preview app: Open HEIC → Export as PNG/JPEG
- macOS Photos app: Export as JPEG
- Online converters: convertio.co, cloudconvert.com

## Dependencies

- `clap`: Command-line argument parsing
- `image`: Image processing and format conversion
- `anyhow`: Error handling

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Specify your license here]

## Troubleshooting

### "HEIC format support is not available"

This means none of the conversion methods are available. Install one of:
- libheif: `brew install libheif`
- ImageMagick: `brew install imagemagick`
- FFmpeg: `brew install ffmpeg`

### "Input file does not exist"

Check that:
- The file path is correct
- You have read permissions for the file
- The file hasn't been moved or deleted

### "Failed to save image"

Check that:
- You have write permissions to the output directory
- The output directory exists (the tool will try to create it)
- There's enough disk space

## Version History

- **0.1.0**: Initial release with basic HEIC to PNG/JPG conversion support

### A Work In Progress

Check back often to see the evolution of the project!! _RInfomaid_ is a work-in-progress. Updates will come periodically.

If you would like to contribute to this project, **then please do!** For instance, if you see some low-hanging fruit or task that you could easily complete, that could add value to the project, then I would love to have your insight.

Otherwise, please create an Issue for bugs or errors. Since I am a teaching faculty member at Allegheny College, I may not have all the time necessary to quickly fix the bugs. I welcome the OpenSource Community to further the development of this project. Much thanks in advance.

If you appreciate this project, please consider clicking the project's _Star_ button. :-)