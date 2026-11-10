use std::{
    error::Error,
    fmt::Display,
    io::{self},
    path::PathBuf,
};

#[derive(Debug)]
pub enum DotyError {
    LinkError(LinkError),
    ClapError(clap::Error),
    UnLinkError(UnLinkError),
    SharedError(SharedError),
    ParsError,
    NoConfigForOs,
}

#[derive(Debug)]
pub enum SharedError {
    HomeDirError,
    ReadError(PathBuf, io::Error),
    PathError,
    PermissionError(String, io::Error),
}
#[derive(Debug)]
pub enum LinkError {
    SharedError(SharedError),
    FileExits(PathBuf),
    SymlinkError(PathBuf, io::Error),
    IoError(io::Error),
}
#[derive(Debug)]
pub enum UnLinkError {
    RemoveError(PathBuf, io::Error),
    IoError(io::Error),
    SharedError(SharedError),
}

impl From<PathBuf> for LinkError {
    fn from(v: PathBuf) -> Self {
        Self::FileExits(v)
    }
}

impl Display for DotyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DotyError::LinkError(symlink_error) => write!(f, "{}", symlink_error),
            DotyError::ClapError(error) => write!(f, "{}", error),
            DotyError::UnLinkError(un_link_error) => write!(f, "{}", un_link_error),
            DotyError::SharedError(shared_error) => write!(f, "{}", shared_error),
            DotyError::ParsError => write!(f, "Unable to pars config"),
            DotyError::NoConfigForOs => write!(f, "No config for this OS"),
        }
    }
}
impl Display for UnLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnLinkError::RemoveError(path_buf, e) => {
                write!(
                    f,
                    "Failed to remove file or directory {}, with error: {}",
                    path_buf.display(),
                    e
                )
            }
            UnLinkError::IoError(error) => {
                write!(f, "Something whent wrong, error: {}", error)
            }
            UnLinkError::SharedError(shared_error) => {
                write!(f, "{}", shared_error)
            }
        }
    }
}
impl Display for SharedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SharedError::PathError => write!(f, "Unable to resolve path"),
            SharedError::HomeDirError => write!(f, "Unable to resolve the home directory"),
            SharedError::ReadError(path_buf, error) => {
                write!(
                    f,
                    "Unable to read directory {}, error: {}",
                    path_buf.display(),
                    error
                )
            }
            SharedError::PermissionError(message, error) => {
                write!(f, "{}, error: {}", message, error)
            }
        }
    }
}
impl Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkError::FileExits(path) => write!(f, "File ({})already exits", path.display()),
            LinkError::IoError(error) => {
                write!(f, "Something whent wrong, error: {}", error)
            }
            LinkError::SharedError(shared_error) => {
                write!(f, "{}", shared_error)
            }
            LinkError::SymlinkError(path_buf, error) => {
                write!(
                    f,
                    "Unable to symlink {}, error: {}",
                    path_buf.display(),
                    error
                )
            }
        }
    }
}

impl From<io::Error> for LinkError {
    fn from(value: io::Error) -> Self {
        LinkError::IoError(value)
    }
}
impl From<SharedError> for LinkError {
    fn from(value: SharedError) -> Self {
        LinkError::SharedError(value)
    }
}
impl From<SharedError> for UnLinkError {
    fn from(value: SharedError) -> Self {
        UnLinkError::SharedError(value)
    }
}
impl From<io::Error> for UnLinkError {
    fn from(value: io::Error) -> Self {
        UnLinkError::IoError(value)
    }
}
impl From<LinkError> for DotyError {
    fn from(value: LinkError) -> Self {
        DotyError::LinkError(value)
    }
}
impl From<UnLinkError> for DotyError {
    fn from(value: UnLinkError) -> Self {
        DotyError::UnLinkError(value)
    }
}
impl From<SharedError> for DotyError {
    fn from(value: SharedError) -> Self {
        DotyError::SharedError(value)
    }
}
impl Error for LinkError {}
impl Error for DotyError {}
impl Error for UnLinkError {}
impl Error for SharedError {}
