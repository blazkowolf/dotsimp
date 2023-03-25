use crate::config::link::Link;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct App<'app> {
    /// Display name for this application
    pub name: &'app str,

    /// Symlinks for this application
    #[serde(borrow)]
    #[serde(default)]
    pub links: Vec<Link<'app>>,
}
