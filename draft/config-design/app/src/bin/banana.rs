use app::AppConfig;
use fruits::{BananaConfig, FruitConfig};

fn main() {
    println!("=== Banana Config Demo ===\n");

    let Ok(config) = AppConfig::load("config/banana") else {
        eprintln!("Failed to load config");
        return;
    };

    let FruitConfig::Banana(banana) = &config.fruit else {
        eprintln!("Expected Banana config");
        return;
    };

    print_banana(banana);
}

fn print_banana(banana: &BananaConfig) {
    println!("=== Banana Config (Validated) ===");
    println!("  Color: {}", banana.color);
    println!("  Curvature: {:?}", banana.curvature);
    println!("  Options:");
    if let Some(ripeness) = banana.options.ripeness {
        println!("    Ripeness: {:.0}%", ripeness * 100.0);
    }
}
