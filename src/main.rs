use glob::glob;
use regex::Captures;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::{self, Args};
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use toml::Value;

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

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let value = value.collect::<Vec<_>>();

        if value.len() < 2 {
            return Err(DotsimpError::MissingReqArg("input_file"));
        }

        Ok(DotsimpArgs::new(&value[1]))
    }
}

type DotsimpResult<TSuccess> = Result<TSuccess, DotsimpError>;

#[derive(Debug)]
enum DotsimpError {
    MissingReqArg(&'static str),
    InvalidConfigPath(io::Error),
}

impl error::Error for DotsimpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DotsimpError::MissingReqArg(ref _arg) => None,
            DotsimpError::InvalidConfigPath(ref err) => Some(err),
        }
    }
}

impl fmt::Display for DotsimpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DotsimpError::MissingReqArg(ref arg) => f.write_fmt(format_args!(
                "Missing required command line argument: {}",
                arg
            )),
            DotsimpError::InvalidConfigPath(ref _err) => {
                f.write_str("Invalid config file path provided.")
            }
        }
    }
}

impl From<io::Error> for DotsimpError {
    fn from(value: io::Error) -> Self {
        DotsimpError::InvalidConfigPath(value)
    }
}

#[derive(Debug, Deserialize)]
struct Config {
    apps: HashMap<String, App>,
}

#[derive(Debug, Deserialize)]
struct App {
    path: PathBuf,
    target: PathBuf,
}
// }}}

fn load_config(path: impl AsRef<Path>) -> DotsimpResult<Config> {
    let path = path.as_ref().canonicalize()?;
    let config_str = fs::read_to_string(path)?;

    let config: Config = toml::from_str(&config_str).unwrap();

    Ok(config)
}

fn main() -> DotsimpResult<()> {
    // let args = DotsimpArgs::try_from(env::args())?;

    // let config = load_config(args.config_file)?;

    // dbg!(config);

    for path in glob("../dotfiles/nvim/**/*.lua")
        .unwrap()
        .filter_map(Result::ok)
    {
        println!("{}", path.display());
    }

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
