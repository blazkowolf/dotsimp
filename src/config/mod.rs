use serde::Deserialize;
use std::collections::HashMap;

pub mod app;
pub mod error;
pub mod link;

pub use self::app::App;
pub use self::error::ConfigError;
pub use self::link::Link;

#[derive(Debug, Default, Deserialize)]
pub struct Config<'config> {
    /// Applications belonging to this configuration
    #[serde(borrow)]
    pub apps: HashMap<&'config str, App<'config>>,
}

impl<'config> TryFrom<&'config str> for Config<'config> {
    type Error = ConfigError;

    fn try_from(contents: &'config str) -> Result<Self, Self::Error> {
        let config = toml::from_str(contents)?;
        Ok(config)
    }
}

impl Config<'_> {
    pub fn from_str(contents: &str) -> Result<Config<'_>, ConfigError> {
        Config::try_from(contents)
    }
}
