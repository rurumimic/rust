use app::AppConfig;
use fruits::{FruitConfig, OrangeConfig};

fn main() {
    println!("=== Orange Config Demo ===\n");

    let Ok(config) = AppConfig::load("config/orange") else {
        eprintln!("Failed to load config");
        return;
    };

    let FruitConfig::Orange(orange) = &config.fruit else {
        eprintln!("Expected Orange config");
        return;
    };

    print_orange(orange);
}

fn print_orange(orange: &OrangeConfig) {
    let color = &orange.color;
    let segments = orange.segments;
    println!("=== Orange Config (Validated) ===");
    println!("  Color: {color}");
    println!("  Segments: {segments}");
    println!("  Options:");
    if let Some(seedless) = orange.options.seedless {
        println!("    Seedless: {seedless}");
    }
}
