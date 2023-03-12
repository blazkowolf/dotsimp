use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct Config<'config> {
    /// Applications belonging to this configuration
    #[serde(borrow)]
    pub apps: HashMap<&'config str, App<'config>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct App<'app> {
    /// Display name for this application
    pub name: &'app str,

    /// Symlinks for this application
    #[serde(borrow)]
    #[serde(default)]
    pub links: Vec<Link<'app>>,
}

#[derive(Debug, Deserialize)]
pub struct Link<'link> {
    /// Path to the symlink
    #[serde(borrow)]
    pub path: &'link Path,

    /// File the symlink points to
    #[serde(borrow)]
    pub target: &'link Path,
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.path.display(), self.target.display())
    }
}

impl<'config> Config<'config> {
    pub fn from_toml(contents: &'config str) -> Self {
        toml::from_str(contents).expect("Config file should be a valid TOML document")
    }
}
