use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CognoError {
    #[error("could not load spec modifier")]
    SpecModifierLoadError(#[from] io::Error),

    #[error("format error in spec modifier")]
    SpecModifierFormatError(#[from] toml::de::Error),

    #[error("conflicting modifiers {0}")]
    ConflictingModifiers(String),

    #[error("serialisation error {0}")]
    SerialisationError(#[from] serde_json::Error)
}
