use core::AppConfig;
use fruits::FruitConfig;

fn main() {
    println!("=== Unknown Key Policy Test ===\n");

    // 1. Warn 정책 테스트
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

    // 2. Deny 정책 테스트 (기본값)
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

    // YAML을 직접 파싱해서 테스트
    let raw: config_schema::SettingsRaw = serde_yaml::from_str(yaml).unwrap();
    match FruitConfig::try_from_raw(&raw.fruit) {
        Ok(config) => {
            println!("Unexpected success: {}", config.kind());
        }
        Err(e) => {
            println!("Expected error: {}", e);
        }
    }

    println!();

    // 3. Allow 정책 테스트
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

    let raw: config_schema::SettingsRaw = serde_yaml::from_str(yaml).unwrap();
    match FruitConfig::try_from_raw(&raw.fruit) {
        Ok(config) => {
            println!("Success (unknown key allowed): {}", config.kind());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
