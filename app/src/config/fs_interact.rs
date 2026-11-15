use std::{
    env::current_dir, fmt::Display, fs, os::unix::fs::PermissionsExt, path::Path, process::Command,
};

use crate::error::{
    DotyError::{self},
    SharedError::{self, PermissionError},
};

#[derive(Debug, Clone)]
pub enum RunMode {
    Install,
    Uninstall,
}
impl Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunMode::Install => write!(f, "Install"),
            RunMode::Uninstall => write!(f, "Uninstall"),
        }
    }
}

pub fn set_needed_permissions(script: &Path) -> Result<(), DotyError> {
    let metadata = fs::metadata(script).unwrap();
    let mut permisitons = metadata.permissions();

    if permisitons.mode() & 0o100 == 0 {
        permisitons.set_mode(permisitons.mode() | 0o100);
        fs::set_permissions(script, permisitons).map_err(|e| {
            DotyError::SharedError(PermissionError("Unable to set permissions".to_string(), e))
        })?;
    }

    Ok(())
}

pub fn run_script(script: &Path, mode: &RunMode, package: &Path) -> Result<(), DotyError> {
    let script = &Path::join(
        &current_dir().map_err(|_| DotyError::SharedError(SharedError::PathError))?,
        package,
    )
    .join(script);

    if !script.exists() {
        return Err(DotyError::ConfigError("Script does not exists".to_string()));
    }

    set_needed_permissions(script)?;
    let arg = match mode {
        RunMode::Install => "install",
        RunMode::Uninstall => "uninstall",
    };

    let mut child = Command::new(script)
        .arg(arg)
        .spawn()
        .map_err(|e| DotyError::SharedError(SharedError::PermissionError("".to_string(), e)))?;
    let status = child
        .wait()
        .map_err(|_| DotyError::FailedScript(package.to_path_buf(), mode.clone()))?;
    if !status.success() {
        return Err(DotyError::FailedScript(package.to_path_buf(), mode.clone()));
    }
    Ok(())
}
