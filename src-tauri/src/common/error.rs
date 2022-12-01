use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MoletError {
    EnvError(String),
    SystemError(String),
    AppError(String),
    IOError(String),
    Warning(String),
    Info(String),
}

impl Display for MoletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoletError::EnvError(s) => write!(f, "{}", s),
            MoletError::SystemError(s) => write!(f, "{}", s),
            MoletError::AppError(s) => write!(f, "{}", s),
            MoletError::IOError(s) => write!(f, "{}", s),
            MoletError::Warning(s) => write!(f, "{}", s),
            MoletError::Info(s) => write!(f, "{}", s),
        }
    }
}
