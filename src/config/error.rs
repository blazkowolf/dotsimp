use std::fmt;

use toml::de;

#[derive(Debug)]
pub enum ConfigError {
    InvalidTOML(String),
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ConfigError::InvalidTOML(_) => None,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTOML(error) => write!(f, "Configuration file was either malformed TOML or did not contain correct configuration values. {error}"),
        }
    }
}

impl From<de::Error> for ConfigError {
    fn from(error: de::Error) -> Self {
        Self::InvalidTOML(error.to_string())
    }
}
