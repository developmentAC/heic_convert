use serde::Deserialize;
use std::fs;
use toml::de::from_str;

use colored::Colorize;
use std::io;
use std::io::Write; // For flushing output

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

// Print colored text to the console
pub fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_green().bold());
        }
        "green_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_purple().bold());
        }
        "purple_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
        "yellow_noLineFeed" => {
            print!("\x1b[2K\r");
            print!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r");
            println!("{}", text.bright_yellow().bold());
        }
    }
}

fn parse_cargo_toml(file_path: &str) {
    // Check if the file exists
    if !std::path::Path::new(file_path).exists() {
        eprintln!("\t Cargo.toml file not found;\n\t Cannot display version information.\n");
        return;
    }

    // Read the content of the Cargo.toml file
    let content = fs::read_to_string(file_path).expect("Failed to read Cargo.toml file");

    // Parse the TOML content into the CargoToml struct
    let cargo_toml: CargoToml = from_str(&content).expect("Failed to parse Cargo.toml");

    // Print the extracted package information
    let out_message_0 = format!("\t Package name: '{}'.", cargo_toml.package.name);
    colour_print(&out_message_0, "purple");

    let out_message_1 = format!("\t Package version: '{}'.", cargo_toml.package.version);
    colour_print(&out_message_1, "purple");

    let out_message_2 = format!("\t Package edition: '{}'.\n", cargo_toml.package.edition);
    colour_print(&out_message_2, "purple");
}

pub fn main() {
    let file_path = "Cargo.toml"; // Path to your Cargo.toml file
    parse_cargo_toml(file_path);
}
