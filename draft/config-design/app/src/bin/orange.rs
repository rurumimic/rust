use app::AppConfig;
use app::schema::{FruitSettingsRaw, OrangeSettingsRaw};

fn main() {
    println!("=== Orange Config Demo ===\n");

    match AppConfig::load("config/orange") {
        Ok(config) => {
            match &config.fruit {
                FruitSettingsRaw::Orange(orange) => {
                    print_orange(orange);
                }
                _ => {
                    println!("Expected Orange config");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}

fn print_orange(orange: &OrangeSettingsRaw) {
    println!("=== Orange Config (Raw) ===");
    println!("  Color: {}", orange.color);
    println!("  Segments: {}", orange.segments);
    println!("  Options:");
    if let Some(seedless) = orange.options.seedless {
        println!("    Seedless: {}", seedless);
    }
}
