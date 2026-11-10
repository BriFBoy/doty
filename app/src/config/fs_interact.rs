use std::{env::current_dir, fs, os::unix::fs::PermissionsExt, path::Path};

use crate::{
    config::dot::Dot,
    error::{
        DotyError::{self, ParsError},
        SharedError::{self, PermissionError},
    },
};

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
