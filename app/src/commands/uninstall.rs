use std::{path::Path, process::Command};

use which_os::{get_os_type, ostypes::Os};

use crate::{
    config::fs_interact::{load_config, set_needed_permissions},
    error::{DotyError, SharedError},
};

pub fn uninstall(package: &Path) -> Result<(), DotyError> {
    let os_type = get_os_type().ok_or(DotyError::SharedError(SharedError::PathError))?;
    let config = load_config(package)?;

    if let Some(os) = config.fedora
        && os_type == Os::Fedora
        && os.uninstall.exists()
    {
        set_needed_permissions(&os.uninstall)?;

        let mut child = Command::new(os.uninstall)
            .spawn()
            .map_err(|e| DotyError::SharedError(SharedError::PermissionError("".to_string(), e)))?;
        child
            .wait()
            .map_err(|_| DotyError::SharedError(SharedError::HomeDirError))?;
    }

    Ok(())
}
