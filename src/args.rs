use crate::error::DotsimpError;
use std::env::ArgsOs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct DotsimpArgs {
    pub config_file: PathBuf,
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

impl TryFrom<ArgsOs> for DotsimpArgs {
    type Error = DotsimpError;

    fn try_from(value: ArgsOs) -> core::result::Result<Self, Self::Error> {
        let value = value.collect::<Vec<_>>();

        if value.len() < 2 {
            return Err(DotsimpError::MissingReqArg("input_file"));
        }

        Ok(DotsimpArgs::new(&value[1]))
    }
}
