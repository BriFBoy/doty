use std::{env::current_dir, fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

use crate::{
    config::dot::Dot,
    error::{
        DotyError::{self, ParsError},
        SharedError::{self, PermissionError},
    },
};

pub enum RunMode {
    Install,
    Uninstall,
}

pub fn load_config(package: &Path) -> Result<Dot, DotyError> {
    let mut dot_file = Path::join(
        &current_dir().map_err(|_| DotyError::SharedError(SharedError::PathError))?,
        package,
    );
    dot_file.push("dot.toml");
    let string = fs::read_to_string(dot_file).map_err(|_| ParsError)?;
    let config: Dot = toml::from_str(&string).map_err(|_| ParsError)?;
    Ok(config)
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

pub fn run_script(script: &Path, mode: RunMode, package: &Path) -> Result<(), DotyError> {
    let script = &Path::join(
        &current_dir().map_err(|_| DotyError::SharedError(SharedError::PathError))?,
        package,
    )
    .join(script);

    if !script.exists() {
        return Err(DotyError::ConfigError);
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
    child
        .wait()
        .map_err(|_| DotyError::SharedError(SharedError::HomeDirError))?;
    Ok(())
}
