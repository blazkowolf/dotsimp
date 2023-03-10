use crate::args::DotsimpArgs;
use crate::model::App;
use crate::prelude::*;
use model::Config;
use regex::Captures;
use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Stdout, Write};
use std::path::Path;
use std::path::PathBuf;

mod args;
mod error;
mod model;
mod prelude;

/// Main program state
pub struct Dotsimp<'dotsimp> {
    /// Command-line arguments/options
    pub args: DotsimpArgs,
    /// Deserialized `.dotsimprc` toml configuration
    pub config: Config<'dotsimp>,
    /// Handle to `stdout`
    pub writer: Stdout,
}

impl<'dotsimp> Dotsimp<'dotsimp> {
    fn run(&self) -> Result<()> {
        for App { name, links } in self.config.apps.values() {
            writeln!(&self.writer, "[{name}] Creating links")?;
            for link in links {
                use std::io::ErrorKind::*;

                // let var_re = Regex::new(r"\$(?P<var>\w+)").expect("This regex messed up somehow");
                let var_re =
                    Regex::new(r"(?P<var>\$(?P<name>\w+))").expect("This regex messed up somehow");

                let expanded_path = {
                    let path_str_lossy = &link.path.to_string_lossy();
                    let path_str = var_re.replace_all(path_str_lossy, |caps: &Captures| {
                        env::var(&caps["name"]).unwrap()
                    });
                    PathBuf::from(path_str.to_string())
                };

                let target = &link.target.canonicalize()?;

                match symlink::symlink_file(target, expanded_path) {
                    Ok(_) => writeln!(&self.writer, "[{name}] Symlink {link} created")?,
                    Err(e) if e.kind() == AlreadyExists => {
                        writeln!(&self.writer, "[{name}] Symlink {link} already exists")?
                    }
                    Err(e) => writeln!(io::stderr(), "[{name}] Symlink {link} failed: {e}")?,
                }
            }
        }

        // for path in glob(&config.apps["nvim"].links.as_ref().unwrap()[0].target)
        //     .unwrap()
        //     .filter_map(core::result::Result::ok)
        // {
        //     println!("{}", path.display());
        // }

        Ok(())
    }
}

// models {{{

// impl Paths {
//     fn new(
//         dir_patterns: Vec<Pattern>,
//         require_dir: bool,
//         options: MatchOptions,
//         todo: Vec<core::result::Result<(PathBuf, usize), GlobError>>,
//         scope: Option<PathBuf>,
//     ) -> Self {
//         Self {
//             dir_patterns,
//             require_dir,
//             options,
//             todo,
//             scope,
//         }
//     }
// }

// #[derive(Debug, Deserialize)]
// #[serde(remote = "Paths")]
// struct PathsDef {
//     dir_patterns: Vec<Pattern>,
//     require_dir: bool,
//     options: MatchOptions,
//     todo: Vec<core::result::Result<(PathBuf, usize), GlobError>>,
//     scope: Option<PathBuf>,
// }
// }}}

fn main() -> Result<()> {
    let args = DotsimpArgs::try_from(env::args_os())?;

    let config_file_path = args.config_file.canonicalize()?;
    let config_file_contents = &fs::read_to_string(config_file_path)?;

    let config = toml::from_str(config_file_contents).unwrap();
    let writer = io::stdout();
    let dotsimp = Dotsimp {
        args,
        config,
        writer,
    };

    dotsimp.run()?;

    Ok(())
}
