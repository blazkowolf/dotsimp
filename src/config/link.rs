use serde::Deserialize;
use std::fmt;
use std::path::Path;

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
