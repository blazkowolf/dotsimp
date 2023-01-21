use glob::glob;
use glob::GlobError;
use glob::MatchOptions;
use glob::Paths;
use glob::Pattern;
use regex::Captures;
use regex::Regex;
use serde::Deserialize;
use serde::Deserializer;
use std::collections::HashMap;
use std::env::{self, Args};
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use toml::Value;

use crate::prelude::*;

mod error;
mod prelude;

// models {{{
#[derive(Debug)]
struct DotsimpArgs {
    config_file: PathBuf,
}

// impl fmt::Display for DotsimpArgs {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_fmt(format_args!("Config file: {:?}", self.config_file))
//     }
// }

impl DotsimpArgs {
    fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        DotsimpArgs {
            config_file: path.to_path_buf(),
        }
    }
}

impl TryFrom<Args> for DotsimpArgs {
    type Error = DotsimpError;

    fn try_from(value: Args) -> core::result::Result<Self, Self::Error> {
        let value = value.collect::<Vec<_>>();

        if value.len() < 2 {
            return Err(DotsimpError::MissingReqArg("input_file"));
        }

        Ok(DotsimpArgs::new(&value[1]))
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    apps: HashMap<String, App>,
}

#[derive(Debug, Deserialize)]
struct App {
    #[serde(default)]
    links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
struct Link {
    path: String,
    target: String,
}

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
    let args = DotsimpArgs::try_from(env::args())?;

    let config = load_config(args.config_file)?;

    dbg!(config);

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

    // let path = Path::new(path_str.as_ref());

    // dbg!(expanded_paths);

    // symlink::symlink_file(sym.src, sym.dest)?;

    Ok(())
}
