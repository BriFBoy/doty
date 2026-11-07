use std::{
    error::Error,
    fmt::Display,
    io::{self},
};

#[derive(Debug)]
pub enum DotyError {
    SymlinkError(SymlinkError),
    ClapError(clap::Error),
}
#[derive(Debug)]
pub enum SymlinkError {
    FileExits,
    PathError,
    IoError(io::Error),
}

impl Display for DotyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotyError::SymlinkError(symlink_error) => write!(f, "{}", symlink_error),
            DotyError::ClapError(error) => write!(f, "{}", error),
        }
    }
}
impl Display for SymlinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymlinkError::FileExits => write!(f, "File already exits"),
            SymlinkError::IoError(error) => write!(f, "{}", error),
            SymlinkError::PathError => write!(f, "Unable to resolve path"),
        }
    }
}

impl From<io::Error> for SymlinkError {
    fn from(value: io::Error) -> Self {
        SymlinkError::IoError(value)
    }
}
impl From<SymlinkError> for DotyError {
    fn from(value: SymlinkError) -> Self {
        DotyError::SymlinkError(value)
    }
}
impl Error for SymlinkError {}
impl Error for DotyError {}
