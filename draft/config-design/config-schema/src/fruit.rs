use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use crate::UnknownKeyPolicy;

/// Fruit 설정의 Raw 스키마 (입력 경계)
///
/// YAML/JSON에서 역직렬화되는 구조체입니다.
/// `options` 필드는 flatten으로 확장 옵션을 받습니다.
#[derive(Debug, Clone, Deserialize)]
pub struct FruitSettingsRaw {
    /// 과일 종류 (apple, banana, orange 등)
    pub kind: String,

    /// 색상 (공통 필드)
    pub color: String,

    /// 확장 옵션 (과일 종류별로 다른 필드)
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,

    /// unknown 키 처리 정책 (옵션)
    #[serde(default)]
    pub unknown_key_policy: UnknownKeyPolicy,
}

impl FruitSettingsRaw {
    /// extra에서 특정 키 추출 (타입 변환 포함)
    pub fn extract<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, crate::SchemaError> {
        match self.extra.get(key) {
            Some(value) => serde_json::from_value(value.clone()).map(Some).map_err(|_| {
                crate::SchemaError::InvalidFieldType {
                    field: key.to_string(),
                    expected: std::any::type_name::<T>().to_string(),
                    actual: format!("{:?}", value),
                }
            }),
            None => Ok(None),
        }
    }

    /// extra에서 필수 키 추출
    pub fn extract_required<T: serde::de::DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<T, crate::SchemaError> {
        self.extract(key)?
            .ok_or_else(|| crate::SchemaError::MissingField(key.to_string()))
    }

    /// 알려진 키 목록과 비교하여 unknown 키 반환
    pub fn unknown_keys(&self, known_keys: &[&str]) -> Vec<String> {
        self.extra
            .keys()
            .filter(|k| !known_keys.contains(&k.as_str()))
            .cloned()
            .collect()
    }
}
