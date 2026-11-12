use std::path::Path;

use which_os::get_os_id;

use crate::{
    config::fs_interact::{RunMode, load_config, run_script},
    error::{DotyError, SharedError},
};

pub mod link;
pub mod unlink;

pub fn install_or_remove(package: &Path, mode: RunMode) -> Result<(), DotyError> {
    let os_type = get_os_id().ok_or(DotyError::SharedError(SharedError::PathError))?;
    let config = load_config(package)?;

    if let Some(os) = config.target.get(&os_type) {
        run_script(os.get_script(), mode, package)?;
        Ok(())
    } else {
        Err(DotyError::NoConfigForOs)
    }
}
