use crate::config::ConfigError;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DotsimpError {
    MissingReqArg(&'static str),
    InvalidConfigPath(io::Error),
    Config(ConfigError),
}

impl error::Error for DotsimpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use DotsimpError::*;
        match *self {
            MissingReqArg(_arg) => None,
            InvalidConfigPath(ref err) => Some(err),
            Config(ref err) => Some(err),
        }
    }
}

impl fmt::Display for DotsimpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DotsimpError::*;
        match self {
            MissingReqArg(arg) => write!(f, "Missing required command line argument: {arg}"),
            InvalidConfigPath(ref _err) => write!(f, "Invalid config file path provided."),
            Config(ref err) => write!(f, "{err}"),
        }
    }
}

impl From<io::Error> for DotsimpError {
    fn from(value: io::Error) -> Self {
        Self::InvalidConfigPath(value)
    }
}

impl From<ConfigError> for DotsimpError {
    fn from(value: ConfigError) -> Self {
        Self::Config(value)
    }
}
