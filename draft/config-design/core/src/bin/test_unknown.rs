use config_schema::SettingsRaw;
use core::AppConfig;
fn main() {
    println!("=== Unknown Key Policy Test ===\n");

    println!("--- Test: WARN policy ---");
    match AppConfig::load("config/apple_with_unknown") {
        Ok(config) => {
            println!("Success! Fruit: {}", config.fruit.kind());
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
    match AppConfig::try_from_raw(raw) {
        Ok(config) => {
            println!("Unexpected success: {}", config.fruit.kind());
        }
        Err(e) => {
            println!("Expected error: {}", e);
        }
    }

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
    match AppConfig::try_from_raw(raw) {
        Ok(config) => {
            println!("Success (unknown key allowed): {}", config.fruit.kind());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
