use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config<'config> {
    #[serde(borrow)]
    pub apps: HashMap<&'config str, App<'config>>,
}

#[derive(Debug, Deserialize)]
pub struct App<'app> {
    pub name: &'app str,
    #[serde(borrow)]
    #[serde(default)]
    pub links: Vec<Link<'app>>,
}

#[derive(Debug, Deserialize)]
pub struct Link<'link> {
    #[serde(borrow)]
    pub path: &'link Path,
    #[serde(borrow)]
    pub target: &'link Path,
}

impl fmt::Display for Link<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.path.display(), self.target.display())
    }
}
