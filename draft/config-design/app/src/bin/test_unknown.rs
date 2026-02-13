use app::AppConfig;
use schema::SettingsRaw;

fn main() {
    println!("=== Unknown Key Policy Test ===\n");

    println!("--- Test: WARN policy ---");
    match AppConfig::load("config/apple_with_unknown") {
        Ok(config) => {
            println!("Success! Fruit: {}", config.fruit.kind());
        }
        Err(e) => {
            eprintln!("{}", e.user_message());
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
    let result: Result<AppConfig, _> = raw.try_into();
    match result {
        Ok(config) => {
            println!("Success! Fruit: {}", config.fruit.kind());
        }
        Err(e) => {
            println!("Rejected (as expected): {}", e.user_message());
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
    let result: Result<AppConfig, _> = raw.try_into();
    match result {
        Ok(config) => {
            println!("Success (unknown key allowed): {}", config.fruit.kind());
        }
        Err(e) => {
            eprintln!("{}", e.user_message());
        }
    }

    println!();

    println!("--- Test: Invalid sweetness (validation) ---");
    let yaml = r#"
app: TestApp
version: 1.0.0
fruit:
  kind: apple
  color: red
  sweetness: 15
"#;

    let raw: SettingsRaw = serde_yaml::from_str(yaml).unwrap();
    let result: Result<AppConfig, _> = raw.try_into();
    match result {
        Ok(_) => {
            println!("Unexpected success");
        }
        Err(e) => {
            println!("Validation failed (as expected): {}", e.user_message());
        }
    }
}
