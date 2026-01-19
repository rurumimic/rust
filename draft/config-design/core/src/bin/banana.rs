use core::AppConfig;
use fruits::{BananaConfig, FruitConfig};

fn main() {
    println!("=== Banana Config Demo ===\n");

    match AppConfig::load("config/banana") {
        Ok(config) => {
            match config.fruit {
                FruitConfig::Banana(banana) => {
                    print_banana(&banana);
                }
                other => {
                    println!("Expected Banana, got: {}", other.kind());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}

fn print_banana(banana: &BananaConfig) {
    println!("=== Banana Config ===");
    println!("  Color: {}", banana.color);
    println!("  Curvature: {:?}", banana.curvature);
    println!("  Options:");
    if let Some(ripeness) = banana.options.ripeness {
        println!("    Ripeness: {:.0}%", ripeness * 100.0);
    }
}
