use app::AppConfig;
use app::schema::{FruitSettingsRaw, SettingsRaw};

fn main() {
    println!("=== Unknown Key Policy Test ===\n");

    println!("--- Test: WARN policy ---");
    match AppConfig::load("config/apple_with_unknown") {
        Ok(config) => {
            println!("Success! Fruit: {}", fruit_kind(&config.fruit));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    println!();

    println!("--- Test: DENY policy (default) ---");
    let yaml = r#"
app: TestApp
version: 1.0.0
fruit:
  kind: apple
  color: green
  sweetness: 5
  unknown_extra: bad_field
  options:
    max_price: 10
"#;

    let raw: SettingsRaw = serde_yaml::from_str(yaml).unwrap();
    println!("Parsed fruit: {}", fruit_kind(&raw.fruit));

    println!();

    println!("--- Test: ALLOW policy ---");
    let yaml = r#"
app: TestApp
version: 1.0.0
fruit:
  kind: apple
  color: green
  sweetness: 5
  unknown_extra: ignored_field
  unknown_key_policy: allow
  options:
    max_price: 10
"#;

    let raw: SettingsRaw = serde_yaml::from_str(yaml).unwrap();
    println!("Success (unknown key allowed): {}", fruit_kind(&raw.fruit));
}

fn fruit_kind(raw: &FruitSettingsRaw) -> &'static str {
    match raw {
        FruitSettingsRaw::Apple(_) => "apple",
        FruitSettingsRaw::Banana(_) => "banana",
        FruitSettingsRaw::Orange(_) => "orange",
    }
}
