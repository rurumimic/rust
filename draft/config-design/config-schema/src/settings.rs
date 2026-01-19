use serde::Deserialize;

use crate::FruitSettingsRaw;

/// 전체 설정 파일의 Raw 스키마
#[derive(Debug, Clone, Deserialize)]
pub struct SettingsRaw {
    /// 앱 이름
    pub app: String,

    /// 버전
    pub version: String,

    /// Fruit 설정
    pub fruit: FruitSettingsRaw,
}
