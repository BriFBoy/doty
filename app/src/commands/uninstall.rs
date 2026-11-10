use std::path::Path;

use which_os::get_os_id;

use crate::{
    config::fs_interact::{load_config, run_script},
    error::{DotyError, SharedError},
};

pub fn uninstall(package: &Path) -> Result<(), DotyError> {
    let os_type = get_os_id().ok_or(DotyError::SharedError(SharedError::PathError))?;
    let config = load_config(package)?;

    if let Some(os) = config.target.get(&os_type) {
        run_script(&os.uninstall)?;
        Ok(())
    } else {
        Err(DotyError::NoConfigForOs)
    }
}
