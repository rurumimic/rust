use app::AppConfig;
use fruits::{AppleConfig, FruitConfig};

fn main() {
    println!("=== Apple Config Demo ===\n");

    match AppConfig::load("config/apple") {
        Ok(config) => {
            println!("App: {}", config.app);
            println!("Version: {}", config.version);
            println!();

            match &config.fruit {
                FruitConfig::Apple(apple) => {
                    print_apple(apple);
                }
                _ => {
                    println!("Expected Apple config");
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
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
