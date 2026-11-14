use std::path::Path;

use which_os::{get_os_id, get_os_id_like};

use crate::{
    config::{
        doty::DotyConfig,
        fs_interact::{RunMode, load_config, run_script},
    },
    error::{
        DotyError::{self, ConfigError},
        SharedError,
    },
};

pub mod link;
pub mod unlink;

/// Runs the install or remove command.
pub fn install_or_remove(package: &Path, mode: RunMode) -> Result<(), DotyError> {
    let doty_config = DotyConfig::load_from_cwd()?;
    let os_type = get_os_id().ok_or(DotyError::SharedError(SharedError::PathError))?;
    let config = load_config(package)?;

    if let Some(os) = config.target.get(&os_type) {
        run_script(os.get_script(), &mode, package)?;
        return Ok(());
    }

    let canditates = get_os_id_like().ok_or(ConfigError(
        "You os does not have any fallbacks".to_string(),
    ))?;

    if doty_config.get_fallback() == "once" {
        if let Some(os) = config.target.get(&canditates[0]) {
            println!("Using fallback: {}", canditates[0]);

            if run_script(os.get_script(), &mode, package).is_ok() {
                return Ok(());
            };
        }
    } else if doty_config.get_fallback() == "full" {
        for cantidate in canditates {
            if let Some(os) = config.target.get(&cantidate) {
                println!("Using fallback: {}", cantidate);

                if run_script(os.get_script(), &mode, package).is_ok() {
                    return Ok(());
                };
            }
        }
    }

    Err(DotyError::NoConfigForOs)
}
