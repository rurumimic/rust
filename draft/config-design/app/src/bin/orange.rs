use app::AppConfig;
use fruits::{FruitConfig, OrangeConfig};

fn main() {
    println!("=== Orange Config Demo ===\n");

    let config = match AppConfig::load("config/orange") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err.user_message());
            return;
        }
    };

    let FruitConfig::Orange(orange) = &config.fruit else {
        eprintln!("Expected orange config, got {}", config.fruit.kind());
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
