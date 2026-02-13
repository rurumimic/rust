use app::AppConfig;
use fruits::{BananaConfig, FruitConfig};

fn main() {
    println!("=== Banana Config Demo ===\n");

    let config = match AppConfig::load("config/banana") {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err.user_message());
            return;
        }
    };

    let FruitConfig::Banana(banana) = &config.fruit else {
        eprintln!("Expected banana config, got {}", config.fruit.kind());
        return;
    };

    print_banana(banana);
}

fn print_banana(banana: &BananaConfig) {
    let color = &banana.color;
    let curvature = &banana.curvature;
    println!("=== Banana Config (Validated) ===");
    println!("  Color: {color}");
    println!("  Curvature: {curvature:?}");
    println!("  Options:");
    if let Some(ripeness) = banana.options.ripeness {
        let percent = ripeness * 100.0;
        println!("    Ripeness: {percent:.0}%");
    }
}
