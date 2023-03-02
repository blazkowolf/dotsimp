use crate::args::DotsimpArgs;
use crate::model::App;
use crate::prelude::*;
use model::Config;
use std::env;
use std::fs;
use std::io::{self, Stdout, Write};
use std::path::Path;

mod args;
mod error;
mod model;
mod prelude;

/// Main program state
pub struct Dotsimp {
    /// Command-line arguments/options
    pub args: DotsimpArgs,
    /// Deserialized `.dotsimprc` toml configuration
    pub config: Config,
    /// Handle to `stdout`
    pub writer: Stdout,
}

impl Dotsimp {
    fn run(&self) -> Result<()> {
        for App { name, links } in self.config.apps.values() {
            writeln!(&self.writer, "[{name}] Creating links")?;
            for link in links {
                use std::io::ErrorKind::*;

                let target = &link.target.canonicalize()?;

                match symlink::symlink_file(target, &link.path) {
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

        // let var_re = Regex::new(r"\$(?P<var>\w+)").expect("This regex messed up somehow");
        // let var_re = Regex::new(r"(?P<var>\$(?P<name>\w+))").expect("This regex messed up somehow");

        //let expanded_paths = DIRS
        //    .iter()
        //    .map(|str| var_re.replace_all(str, |caps: &Captures| env::var(&caps["name"]).unwrap()))
        //    //.map(|val| )
        //    .collect::<Vec<_>>();

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

fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref().canonicalize()?;
    let config_str = fs::read_to_string(path)?;

    let config: Config = toml::from_str(&config_str).unwrap();

    Ok(config)
}

fn main() -> Result<()> {
    let args = DotsimpArgs::try_from(env::args_os())?;
    let config = load_config(&args.config_file)?;
    let writer = io::stdout();
    let dotsimp = Dotsimp {
        args,
        config,
        writer,
    };

    dotsimp.run()?;

    Ok(())
}
