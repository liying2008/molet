use std::fmt::{write, Display, Formatter};

#[derive(Debug)]
pub enum MoletError {
    EnvError(String),
    SystemError(String),
    IOError(String),
}

impl Display for MoletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoletError::EnvError(s) => f.write_str(s.as_str()),
            MoletError::SystemError(s) => f.write_str(s.as_str()),
            MoletError::IOError(s) => f.write_str(s.as_str()),
        }
    }
}
