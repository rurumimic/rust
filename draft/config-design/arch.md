# Config Design Architecture

## 목표
- 순환 의존성 없이 설정 관리
- 컴파일 타임 안정성 (enum + match, 정적 타이핑)
- 모듈별 확장 옵션 지원
- unknown 키 처리 정책 (deny/allow/warn)

## 크레이트 레이아웃

```
config-design/
├── config-schema/     # 설정 스키마 정의 (입력 경계)
├── fruits/            # 도메인 로직 + 도메인 Config 타입
└── core/              # 앱 진입점, config 로딩 및 변환
```

### 의존성 방향 (순환 없음)

```
config-schema  ←──  fruits  ←──  core
     │                │           │
     └── serde        └── serde   └── config-rs
         serde_json       config-     fruits
         thiserror        schema      config-schema
```

## 핵심 패턴

### 1. 경계에서만 Value 사용 (tagged enum + unknown keys)

```rust
// config-schema/src/fruit.rs
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum FruitSettingsRaw {
    Apple(AppleSettingsRaw),
    Banana(BananaSettingsRaw),
    Orange(OrangeSettingsRaw),
}
```

### 2. 도메인 내부는 완전 타입화

```rust
// fruits/src/config.rs
pub enum FruitConfig {
    Apple(AppleConfig),
    Banana(BananaConfig),
    Orange(OrangeConfig),
}

// fruits/src/apple.rs
pub struct AppleConfig {
    pub color: String,
    pub sweetness: i32,       // 타입 안전
    pub options: AppleOptions, // 구조체로 확장 옵션
}
```

### 3. try_from_raw 패턴으로 검증 + 변환

```rust
impl AppleConfig {
    pub fn try_from_raw<F>(raw: &AppleSettingsRaw, warn_fn: F) -> Result<Self, FruitError>
    where F: Fn(&str)
    {
        // 1. 유효성 검증
        if !(0..=10).contains(&raw.sweetness) {
            return Err(FruitError::InvalidSweetness(raw.sweetness));
        }

        // 2. unknown 키 처리
        let unknown = raw.unknown_keys();
        raw.unknown_key_policy.handle_unknown(&unknown, "apple", warn_fn)?;

        Ok(AppleConfig { color: raw.color.clone(), sweetness: raw.sweetness, options: raw.options.clone() })
    }
}
```

### 4. Unknown 키 처리 전략

```rust
pub enum UnknownKeyPolicy {
    Deny,   // 에러 발생 (기본값)
    Warn,   // 경고 로그 후 무시
    Allow,  // 무시
}
```

YAML에서 정책 지정:
```yaml
fruit:
  kind: apple
  color: red
  sweetness: 7
  unknown_field: value        # unknown 필드
  unknown_key_policy: warn    # warn/deny/allow
```

## 파일 구조

```
config-schema/src/
├── lib.rs
├── error.rs      # SchemaError
├── policy.rs     # UnknownKeyPolicy
├── fruit.rs      # FruitSettingsRaw
└── settings.rs   # SettingsRaw

fruits/src/
├── lib.rs
├── error.rs      # FruitError
├── config.rs     # FruitConfig enum
├── apple.rs      # AppleConfig, AppleOptions
├── banana.rs     # BananaConfig, Curvature enum
└── orange.rs     # OrangeConfig, OrangeOptions

core/src/
├── lib.rs        # AppConfig, load()
└── bin/
    ├── apple.rs
    ├── banana.rs
    └── orange.rs
```

## 실행 예시

```bash
cargo run --bin apple       # settings.yaml 로드
cargo run --bin banana      # banana.yaml 로드
cargo run --bin test_unknown # unknown 키 정책 테스트
```
