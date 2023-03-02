use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub apps: HashMap<String, App>,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    #[serde(default)]
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub path: PathBuf,
    pub target: PathBuf,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.path.display(), self.target.display())
    }
}
