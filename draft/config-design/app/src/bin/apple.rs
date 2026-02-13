use app::AppConfig;
use fruits::{AppleConfig, FruitConfig};

fn main() {
    println!("=== Apple Config Demo ===\n");

    let Ok(config) = AppConfig::load("config/apple") else {
        eprintln!("Failed to load config");
        return;
    };

    let app = &config.app;
    let version = &config.version;
    println!("App: {app}");
    println!("Version: {version}");
    println!();

    let FruitConfig::Apple(apple) = &config.fruit else {
        eprintln!("Expected Apple config");
        return;
    };

    print_apple(apple);
}

fn print_apple(apple: &AppleConfig) {
    let color = &apple.color;
    let sweetness = apple.sweetness;
    println!("=== Apple Config (Validated) ===");
    println!("  Color: {color}");
    println!("  Sweetness: {sweetness}/10");
    println!("  Options:");
    if let Some(price) = apple.options.max_price {
        println!("    Max Price: ${price}");
    }
    if let Some(season) = apple.options.season_only {
        println!("    Season Only: {season}");
    }
}
