use serde::Deserialize;

/// Unknown 키 처리 정책
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UnknownKeyPolicy {
    /// 에러 발생 (기본값)
    #[default]
    Deny,
    /// 경고 로그 후 무시
    Warn,
    /// 무시
    Allow,
}

impl UnknownKeyPolicy {
    /// 알려지지 않은 키들을 정책에 따라 처리
    ///
    /// - `Deny`: Err 반환
    /// - `Warn`: 경고 출력 후 Ok
    /// - `Allow`: 무시하고 Ok
    pub fn handle_unknown<F>(
        &self,
        unknown_keys: &[String],
        context: &str,
        warn_fn: F,
    ) -> Result<(), crate::SchemaError>
    where
        F: Fn(&str),
    {
        if unknown_keys.is_empty() {
            return Ok(());
        }

        match self {
            UnknownKeyPolicy::Deny => {
                Err(crate::SchemaError::UnknownKeys(unknown_keys.to_vec()))
            }
            UnknownKeyPolicy::Warn => {
                warn_fn(&format!(
                    "[{}] unknown keys will be ignored: {:?}",
                    context, unknown_keys
                ));
                Ok(())
            }
            UnknownKeyPolicy::Allow => Ok(()),
        }
    }
}
