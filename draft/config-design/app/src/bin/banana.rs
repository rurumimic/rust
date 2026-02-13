use app::AppConfig;
use app::schema::{BananaSettingsRaw, FruitSettingsRaw};

fn main() {
    println!("=== Banana Config Demo ===\n");

    match AppConfig::load("config/banana") {
        Ok(config) => {
            match &config.fruit {
                FruitSettingsRaw::Banana(banana) => {
                    print_banana(banana);
                }
                _ => {
                    println!("Expected Banana config");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}

fn print_banana(banana: &BananaSettingsRaw) {
    println!("=== Banana Config (Raw) ===");
    println!("  Color: {}", banana.color);
    println!("  Curvature: {:?}", banana.curvature);
    println!("  Options:");
    if let Some(ripeness) = banana.options.ripeness {
        println!("    Ripeness: {:.0}%", ripeness * 100.0);
    }
}
