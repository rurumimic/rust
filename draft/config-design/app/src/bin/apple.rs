use app::AppConfig;
use fruits::{AppleConfig, FruitConfig};

fn main() {
    println!("=== Apple Config Demo ===\n");

    let Ok(config) = AppConfig::load("config/apple") else {
        eprintln!("Failed to load config");
        return;
    };

    println!("App: {}", config.app);
    println!("Version: {}", config.version);
    println!();

    let FruitConfig::Apple(apple) = &config.fruit else {
        eprintln!("Expected Apple config");
        return;
    };

    print_apple(apple);
}

fn print_apple(apple: &AppleConfig) {
    println!("=== Apple Config (Validated) ===");
    println!("  Color: {}", apple.color);
    println!("  Sweetness: {}/10", apple.sweetness);
    println!("  Options:");
    if let Some(price) = apple.options.max_price {
        println!("    Max Price: ${}", price);
    }
    if let Some(season) = apple.options.season_only {
        println!("    Season Only: {}", season);
    }
}
