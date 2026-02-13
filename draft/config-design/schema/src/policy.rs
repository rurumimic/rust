use serde::Deserialize;

/// Unknown key handling policy.
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UnknownKeyPolicy {
    /// Reject unknown keys (default).
    #[default]
    Deny,
    /// Log a warning and ignore.
    Warn,
    /// Ignore silently.
    Allow,
}

impl UnknownKeyPolicy {
    /// Handle unknown keys per policy.
    ///
    /// - `Deny`: return an error
    /// - `Warn`: log and continue
    /// - `Allow`: ignore and continue
    pub fn handle_unknown(
        &self,
        unknown_keys: &[String],
        context: &str,
    ) -> Result<(), crate::SchemaError> {
        if unknown_keys.is_empty() {
            return Ok(());
        }

        match self {
            UnknownKeyPolicy::Deny => {
                Err(crate::SchemaError::UnknownKeys(unknown_keys.to_vec()))
            }
            UnknownKeyPolicy::Warn => {
                log::warn!(
                    "[{}] unknown keys will be ignored: {:?}",
                    context,
                    unknown_keys
                );
                Ok(())
            }
            UnknownKeyPolicy::Allow => Ok(()),
        }
    }
}
