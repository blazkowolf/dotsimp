use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DotsimpError {
    MissingReqArg(&'static str),
    InvalidConfigPath(io::Error),
}

impl error::Error for DotsimpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DotsimpError::MissingReqArg(_arg) => None,
            DotsimpError::InvalidConfigPath(ref err) => Some(err),
        }
    }
}

impl fmt::Display for DotsimpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DotsimpError::MissingReqArg(arg) => f.write_fmt(format_args!(
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
