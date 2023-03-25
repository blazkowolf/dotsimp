#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused)]
#![warn(clippy::all, clippy::pedantic)]

use crate::args::DotsimpArgs;
use crate::config::{App, Config};
use crate::prelude::*;
use regex::Captures;
use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, Stdout, Write};
use std::path::PathBuf;
use std::process::exit;

mod args;
mod config;
mod error;
mod prelude;

const ENV_VAR_REGEX_STR: &str = r"(?P<var>\$(?P<name>\w+))";

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
                let var_re = Regex::new(ENV_VAR_REGEX_STR).expect("This regex messed up somehow");

                let expanded_path = {
                    let path_str_lossy = &link.path.to_string_lossy();
                    let path_str = var_re.replace_all(path_str_lossy, |caps: &Captures<'_>| {
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

fn main() {
    let args =
        DotsimpArgs::try_from(env::args_os()).expect("dotsimp program arguments should be valid");

    let config_path = args
        .config_file
        .canonicalize()
        .expect("configuration file path should be valid");
    let config_contents =
        &fs::read_to_string(config_path).expect("configuration file should exist");

    let config =
        Config::from_str(config_contents).expect("configuration file contents should be valid");
    let writer = io::stdout();
    let dotsimp = Dotsimp {
        args,
        config,
        writer,
    };

    match dotsimp.run() {
        Ok(_) => exit(0),
        Err(error) => {
            eprintln!("{error}");
            exit(1)
        }
    }
}
