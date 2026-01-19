use core::AppConfig;
use fruits::{FruitConfig, OrangeConfig};

fn main() {
    println!("=== Orange Config Demo ===\n");

    match AppConfig::load("config/orange") {
        Ok(config) => {
            match config.fruit {
                FruitConfig::Orange(orange) => {
                    print_orange(&orange);
                }
                other => {
                    println!("Expected Orange, got: {}", other.kind());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}

fn print_orange(orange: &OrangeConfig) {
    println!("=== Orange Config ===");
    println!("  Color: {}", orange.color);
    println!("  Segments: {}", orange.segments);
    println!("  Options:");
    if let Some(seedless) = orange.options.seedless {
        println!("    Seedless: {}", seedless);
    }
}
