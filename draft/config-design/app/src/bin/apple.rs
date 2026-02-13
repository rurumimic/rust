use app::AppConfig;
use fruits::{AppleConfig, FruitConfig};

fn main() {
    println!("=== Apple Config Demo ===\n");

    let config = match AppConfig::load("config/apple") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err.user_message());
            return;
        }
    };

    let app = &config.app;
    let version = &config.version;
    println!("App: {app}");
    println!("Version: {version}");
    println!();

    let FruitConfig::Apple(apple) = &config.fruit else {
        eprintln!("Expected apple config, got {}", config.fruit.kind());
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
