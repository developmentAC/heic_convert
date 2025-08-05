// External crate imports for error handling, CLI parsing, image processing, and system interaction
use anyhow::{Context, Result, anyhow};     // Error handling with context
use clap::{Parser, ValueEnum};              // Command-line argument parsing
use image::{DynamicImage, ImageFormat};     // Image processing library
use std::fs;                                // File system operations
use std::path::{Path, PathBuf};             // Path handling utilities
use std::process::Command;                  // External command execution

// use colored::Colorize;

mod toml_extract; // Extract and print the version information according to the toml file

// Enum to represent supported output image formats
#[derive(Clone, Debug, ValueEnum)]
enum OutputFormat {
    Png,    // PNG format
    Jpg,    // JPEG format (alternative naming)
    Jpeg,   // JPEG format (standard naming)
}

impl OutputFormat {
    // Convert our enum to the image crate's ImageFormat enum
    fn to_image_format(&self) -> ImageFormat {
        match self {
            OutputFormat::Png => ImageFormat::Png,
            OutputFormat::Jpg | OutputFormat::Jpeg => ImageFormat::Jpeg,
        }
    }

    // Get the file extension string for the format
    fn extension(&self) -> &str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Jpg | OutputFormat::Jpeg => "jpg",
        }
    }
}

// Command-line interface structure using clap derive macros
#[derive(Parser)]
#[command(name = "heic_convert")]
#[command(about = "Convert HEIC images to PNG or JPG format")]
#[command(version)]
struct Cli {
    /// Input HEIC file path - the source file to convert
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output file path - where to save the converted image (auto-generated if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output format - PNG (default), JPG, or JPEG
    #[arg(short, long, value_enum, default_value = "png")]
    format: OutputFormat,

    /// Show detailed help with usage examples
    #[arg(long)]
    bighelp: bool,
}

// Display comprehensive help information with detailed usage examples
fn print_bighelp() {
    println!("HEIC to PNG/JPG Converter - Detailed Help");
    println!("=========================================");
    println!();
    println!(
        "This tool converts HEIC (High Efficiency Image Container) files to PNG or JPG format."
    );
    println!();
    println!("BASIC USAGE:");
    println!("  heic_convert -i input.heic                    # Convert to PNG (default)");
    println!("  heic_convert -i input.heic -f jpg             # Convert to JPG");
    println!("  heic_convert -i input.heic -o output.png      # Specify output file");
    println!();
    println!("EXAMPLES:");
    println!("  # Convert a single HEIC file to PNG:");
    println!("  heic_convert -i photo.heic");
    println!("  # Output: photo.png");
    println!();
    println!("  # Convert to JPG format:");
    println!("  heic_convert -i photo.heic -f jpg");
    println!("  # Output: photo.jpg");
    println!();
    println!("  # Specify custom output filename:");
    println!("  heic_convert -i IMG_1234.heic -o my_photo.png");
    println!("  # Output: my_photo.png");
    println!();
    println!("  # Convert with different output directory:");
    println!("  heic_convert -i /path/to/photo.heic -o /output/dir/converted.jpg -f jpg");
    println!("  # Output: /output/dir/converted.jpg");
    println!();
    println!("  # Convert multiple files (use in shell script):");
    println!("  for file in *.heic; do heic_convert -i \"$file\" -f png; done");
    println!("  # Converts all HEIC files in current directory to PNG");
    println!();
    println!("OPTIONS:");
    println!("  -i, --input <FILE>     Input HEIC file path");
    println!("  -o, --output <FILE>    Output file path (optional)");
    println!("  -f, --format <FORMAT>  Output format: png, jpg, jpeg [default: png]");
    println!("  --bighelp              Show this detailed help");
    println!("  -h, --help             Show basic help");
    println!("  -V, --version          Show version");
    println!();
    println!("NOTES:");
    println!(
        "  - If no output file is specified, the tool will generate one based on the input filename"
    );
    println!("  - Supported output formats: PNG, JPG/JPEG");
    println!("  - The tool preserves image quality during conversion");
    println!("  - Requires libheif system library to be installed (brew install libheif)");
    println!();
    println!("SYSTEM REQUIREMENTS:");
    println!("  - macOS: Install libheif via Homebrew: brew install libheif");
    println!("  - Linux: Install libheif via package manager: apt-get install libheif-dev");
    println!("  - Windows: Install libheif development libraries");
    println!();
    println!("ALTERNATIVE METHODS:");
    println!("  If this tool doesn't work, you can also use:");
    println!("  - ImageMagick: convert input.heic output.png");
    println!("  - FFmpeg: ffmpeg -i input.heic output.png");
    println!("  - Online converters: convertio.co, cloudconvert.com");
}

// Generate an output file path based on input filename and desired format
// This function creates a new filename with the appropriate extension in the same directory
fn generate_output_path(input: &Path, format: &OutputFormat) -> PathBuf {
    let stem = input.file_stem().unwrap_or_default();      // Get filename without extension
    let parent = input.parent().unwrap_or(Path::new(".")); // Get parent directory, default to current
    // Combine parent directory, filename stem, and new extension
    parent.join(format!("{}.{}", stem.to_string_lossy(), format.extension()))
}

// Check if ImageMagick is available on the system by running 'convert -version'
fn check_imagemagick_available() -> bool {
    match Command::new("convert")
        .arg("-version")
        .output() 
    {
        Ok(output) => output.status.success(),
        Err(_) => false,  // Command failed to execute (likely not installed)
    }
}

// Check if FFmpeg is available on the system by running 'ffmpeg -version'
fn check_ffmpeg_available() -> bool {
    match Command::new("ffmpeg")
        .arg("-version")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => false,  // Command failed to execute (likely not installed)
    }
}

// Convert HEIC file using ImageMagick's 'convert' command
fn convert_with_imagemagick(input_path: &Path, output_path: &Path) -> Result<()> {
    println!(
        "Using ImageMagick to convert {} to {}",
        input_path.display(),
        output_path.display()
    );

    // Execute ImageMagick convert command with input and output paths
    let output = Command::new("convert")
        .arg(input_path.to_str().unwrap())
        .arg(output_path.to_str().unwrap())
        .output()
        .context("Failed to execute ImageMagick convert command. Make sure ImageMagick is installed: 'brew install imagemagick'")?;

    // Check if the conversion was successful
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Provide specific error messages for common ImageMagick issues
        if stderr.contains("no decode delegate") || stderr.contains("HEIC") {
            return Err(anyhow!(
                "ImageMagick HEIC support is not available.\n\
                 Install HEIC support with: brew install imagemagick --with-heif\n\
                 Or try: brew install libheif && brew reinstall imagemagick\n\
                 Original error: {}", stderr
            ));
        } else if stderr.contains("command not found") || stderr.contains("No such file") {
            return Err(anyhow!(
                "ImageMagick is not installed or not found in PATH.\n\
                 Install it with: brew install imagemagick\n\
                 Original error: {}", stderr
            ));
        } else {
            return Err(anyhow!("ImageMagick conversion failed: {}", stderr));
        }
    }

    println!("Successfully converted to {}", output_path.display());
    Ok(())
}

// Convert HEIC file using FFmpeg
fn convert_with_ffmpeg(input_path: &Path, output_path: &Path) -> Result<()> {
    println!(
        "Using FFmpeg to convert {} to {}",
        input_path.display(),
        output_path.display()
    );

    // Execute FFmpeg command with input file, overwrite flag, and output file
    let output = Command::new("ffmpeg")
        .arg("-i")                              // Input flag
        .arg(input_path.to_str().unwrap())
        .arg("-y")                              // Overwrite output file without asking
        .arg(output_path.to_str().unwrap())
        .output()
        .context("Failed to execute FFmpeg command. Make sure FFmpeg is installed: 'brew install ffmpeg'")?;

    // Check if the conversion was successful
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Provide specific error messages for common FFmpeg issues
        if stderr.contains("No such file or directory") && stderr.contains("ffmpeg") {
            return Err(anyhow!(
                "FFmpeg is not installed or not found in PATH.\n\
                 Install it with: brew install ffmpeg\n\
                 Original error: {}", stderr
            ));
        } else if stderr.contains("Invalid data found") || stderr.contains("could not find codec") {
            return Err(anyhow!(
                "FFmpeg cannot decode this HEIC file. The file may be corrupted or use an unsupported HEIC variant.\n\
                 Try installing FFmpeg with additional codec support: brew install ffmpeg --with-libheif\n\
                 Original error: {}", stderr
            ));
        } else if stderr.contains("Permission denied") {
            return Err(anyhow!(
                "Permission denied when trying to write output file: {}\n\
                 Check file permissions and disk space.\n\
                 Original error: {}", 
                output_path.display(), stderr
            ));
        } else {
            return Err(anyhow!("FFmpeg conversion failed: {}", stderr));
        }
    }

    println!("Successfully converted to {}", output_path.display());
    Ok(())
}

// Main conversion function that orchestrates the HEIC to image conversion process
fn convert_heic_to_image(
    input_path: &Path,
    output_path: &Path,
    format: &OutputFormat,
) -> Result<()> {
    // Validate that the input file has a HEIC/HEIF extension
    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Warn if extension doesn't look like HEIC, but continue anyway
    if !["heic", "heif"].contains(&extension.as_str()) {
        println!("⚠️  Warning: File extension '{}' is not typical for HEIC files.", extension);
        println!("    Expected: .heic or .heif");
        println!("    Attempting conversion anyway...");
        println!();
    }

    // Strategy 1: Try to use the Rust image crate's built-in support first (fastest)
    match image::open(input_path) {
        Ok(img) => {
            println!(
                "Converting {} to {}",
                input_path.display(),
                output_path.display()
            );
            save_image(&img, output_path, format)?;
            return Ok(());
        }
        Err(img_error) => {
            // Image crate doesn't support HEIC, fall back to external tools
            println!("Rust image crate cannot handle this file, trying external tools...");
            println!("Image crate error: {}", img_error);
        }
    }

    // Strategy 2: Try ImageMagick (most common and reliable)
    if check_imagemagick_available() {
        return convert_with_imagemagick(input_path, output_path);
    }

    // Strategy 3: Try FFmpeg (alternative option)
    if check_ffmpeg_available() {
        return convert_with_ffmpeg(input_path, output_path);
    }

    // No conversion methods available - provide helpful error message
    Err(anyhow!(
        "HEIC format support is not available.\n\
         \n\
         To enable HEIC conversion, install one of these tools:\n\
         \n\
         1. ImageMagick:\n\
            brew install imagemagick\n\
         \n\
         2. FFmpeg:\n\
            brew install ffmpeg\n\
         \n\
         3. System libheif library:\n\
            brew install libheif\n\
         \n\
         Alternative solutions:\n\
         - Use online converters like convertio.co or cloudconvert.com\n\
         - Use the macOS Preview app: Open HEIC → Export as PNG/JPEG\n\
         - Use Photos app: Export as JPEG"
    ))
}

// Save a DynamicImage to disk in the specified format
fn save_image(img: &DynamicImage, output_path: &Path, format: &OutputFormat) -> Result<()> {
    // Save the image using the specified format and provide detailed error context
    img.save_with_format(output_path, format.to_image_format())
        .with_context(|| {
            format!(
                "Failed to save image to: {}\n\
                 Possible causes:\n\
                 - Insufficient disk space\n\
                 - No write permission to directory\n\
                 - Invalid output path\n\
                 - Output directory doesn't exist", 
                output_path.display()
            )
        })?;

    println!("Successfully converted to {}", output_path.display());
    Ok(())
}

// Check system requirements and provide early feedback about available conversion methods
fn check_system_requirements() -> Result<()> {
    let imagemagick_available = check_imagemagick_available();
    let ffmpeg_available = check_ffmpeg_available();
    
    // If no external tools are available, warn the user early
    if !imagemagick_available && !ffmpeg_available {
        println!("⚠️  Warning: No HEIC conversion tools detected!");
        println!();
        println!("The Rust image crate has limited HEIC support. For best results, install:");
        println!("  • ImageMagick: brew install imagemagick");
        println!("  • FFmpeg: brew install ffmpeg");
        println!();
        println!("Attempting conversion anyway...");
        println!();
    } else {
        let mut available_tools = Vec::new();
        if imagemagick_available {
            available_tools.push("ImageMagick");
        }
        if ffmpeg_available {
            available_tools.push("FFmpeg");
        }
        println!("✅ Conversion tools available: {}", available_tools.join(", "));
    }
    
    Ok(())
}

// Display an ASCII art banner for the application
fn show_banner() {
    // ASCII art banner displaying "HEIC CONVERT"
    let banner = String::from(
        "\n
\t 
\t ██╗  ██╗   ███████╗   ██╗    ██████╗   
\t ██║  ██║   ██╔════╝   ██║   ██╔════╝   
\t ███████║   █████╗     ██║   ██║        
\t ██╔══██║   ██╔══╝     ██║   ██║        
\t ██║  ██║   ███████╗   ██║   ╚██████╗   
\t ╚═╝  ╚═╝   ╚══════╝   ╚═╝    ╚═════╝
\t 
\t  ██████╗    ██████╗    ███╗   ██╗    ██╗   ██╗   ███████╗   ██████╗    ████████╗   
\t ██╔════╝   ██╔═══██╗   ████╗  ██║    ██║   ██║   ██╔════╝   ██╔══██╗   ╚══██╔══╝   
\t ██║        ██║   ██║   ██╔██╗ ██║    ██║   ██║   █████╗     ██████╔╝      ██║      
\t ██║        ██║   ██║   ██║╚██╗██║    ╚██╗ ██╔╝   ██╔══╝     ██╔══██╗      ██║      
\t ╚██████╗   ╚██████╔╝   ██║ ╚████║     ╚████╔╝    ███████╗   ██║  ██║      ██║      
\t  ╚═════╝    ╚═════╝    ╚═╝  ╚═══╝      ╚═══╝     ╚══════╝   ╚═╝  ╚═╝      ╚═╝
\t 

",
    );

    // Print the banner in cyan color using the toml_extract module's color function
    toml_extract::colour_print(&banner, "cyan");
}

// Main application entry point
fn main() -> Result<()> {
    // Initialize the application by displaying version information and banner
    toml_extract::main();  // Display version information from Cargo.toml
    show_banner();         // Display ASCII art banner

    // Parse command-line arguments
    let cli = Cli::parse();

    // If user requested detailed help, show it and exit
    if cli.bighelp {
        print_bighelp();
        return Ok(());
    }

    // Check system requirements and available conversion tools
    check_system_requirements()?;

    // Validate that input file was provided
    let input_path = cli.input.ok_or_else(|| {
        anyhow!(
            "❌ Input file is required!\n\
             \n\
             Usage: heic_convert -i <input.heic> [-o <output.png>] [-f <format>]\n\
             \n\
             Examples:\n\
             • heic_convert -i photo.heic\n\
             • heic_convert -i photo.heic -f jpg\n\
             • heic_convert -i photo.heic -o converted.png\n\
             \n\
             Use --bighelp for detailed examples and options."
        )
    })?;

    // Verify that the input file exists on the filesystem
    if !input_path.exists() {
        return Err(anyhow!(
            "❌ Input file does not exist: {}\n\
             \n\
             Please check:\n\
             • File path is correct\n\
             • File exists and is accessible\n\
             • You have read permissions for the file",
            input_path.display()
        ));
    }

    // Check if input file is readable
    match std::fs::metadata(&input_path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(anyhow!(
                    "❌ Input path is not a file: {}\n\
                     Please provide a path to a HEIC file, not a directory.",
                    input_path.display()
                ));
            }
            if metadata.len() == 0 {
                return Err(anyhow!(
                    "❌ Input file is empty: {}\n\
                     The HEIC file appears to be corrupted or empty.",
                    input_path.display()
                ));
            }
        }
        Err(e) => {
            return Err(anyhow!(
                "❌ Cannot access input file: {}\n\
                 Error: {}\n\
                 Please check file permissions and path.",
                input_path.display(),
                e
            ));
        }
    }

    // Determine output path: use provided path or auto-generate based on input filename
    let output_path = cli
        .output
        .unwrap_or_else(|| generate_output_path(&input_path, &cli.format));

    // Validate output path and check for potential issues
    if let Some(parent) = output_path.parent() {
        // Check if parent directory exists, if not try to create it
        if !parent.exists() {
            println!("Creating output directory: {}", parent.display());
            fs::create_dir_all(parent)
                .with_context(|| {
                    format!(
                        "❌ Failed to create output directory: {}\n\
                         Possible causes:\n\
                         • No write permission to parent directory\n\
                         • Invalid characters in path\n\
                         • Disk full\n\
                         • Path too long", 
                        parent.display()
                    )
                })?;
        }
        
        // Check if we can write to the output directory
        if parent.exists() && !parent.metadata()
            .map(|m| !m.permissions().readonly())
            .unwrap_or(false) 
        {
            return Err(anyhow!(
                "❌ No write permission to output directory: {}\n\
                 Please check directory permissions or choose a different output location.",
                parent.display()
            ));
        }
    }

    // Check if output file already exists and warn user
    if output_path.exists() {
        println!("⚠️  Output file already exists and will be overwritten: {}", output_path.display());
    }

    // Perform the actual HEIC to image conversion with comprehensive error handling
    match convert_heic_to_image(&input_path, &output_path, &cli.format) {
        Ok(()) => {
            println!("✅ Conversion completed successfully!");
            Ok(())
        }
        Err(e) => {
            // Provide user-friendly error messages with solutions
            if e.to_string().contains("HEIC format support is not available") {
                eprintln!("❌ HEIC Conversion Failed - Missing Dependencies");
                eprintln!();
                eprintln!("🔧 Quick Fix Options:");
                eprintln!("   1. Install ImageMagick: brew install imagemagick");
                eprintln!("   2. Install FFmpeg: brew install ffmpeg");
                eprintln!("   3. Use online converter: https://convertio.co/heic-png/");
                eprintln!();
                eprintln!("📱 macOS Users can also:");
                eprintln!("   • Open HEIC in Preview → Export as PNG/JPEG");
                eprintln!("   • Use Photos app → Export → JPEG");
                eprintln!();
                eprintln!("Original error: {}", e);
            } else if e.to_string().contains("does not appear to be a HEIC file") {
                eprintln!("❌ Invalid File Format");
                eprintln!();
                eprintln!("The input file doesn't appear to be a HEIC/HEIF file.");
                eprintln!("Supported extensions: .heic, .heif");
                eprintln!();
                eprintln!("Current file: {}", input_path.display());
                eprintln!("File extension: {:?}", input_path.extension());
            } else if e.to_string().contains("Failed to save image") {
                eprintln!("❌ Failed to Save Output File");
                eprintln!();
                eprintln!("Could not write to: {}", output_path.display());
                eprintln!("Please check:");
                eprintln!("   • Disk space availability");
                eprintln!("   • Write permissions");
                eprintln!("   • Output directory exists");
            } else {
                eprintln!("❌ Conversion Error: {}", e);
                eprintln!();
                eprintln!("💡 Try these solutions:");
                eprintln!("   1. Check if input file is corrupted");
                eprintln!("   2. Try a different output location");
                eprintln!("   3. Install conversion tools: brew install imagemagick ffmpeg");
                eprintln!("   4. Use --bighelp for more options");
            }
            Err(e)
        }
    }
}
