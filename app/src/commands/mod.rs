use std::path::Path;

use which_os::{get_os_id, get_os_id_like};

use crate::{
    config::{
        dot::DotConfig,
        doty::DotyConfig,
        fs_interact::{RunMode, run_script},
    },
    error::{
        DotyError::{self, ConfigError},
        SharedError,
    },
};

pub mod bundle;
pub mod link;
pub mod unlink;

/// Runs the install or remove command.
pub fn install_or_remove(package: &Path, mode: &RunMode) -> Result<(), DotyError> {
    let doty_config = DotyConfig::load_from_cwd()?;
    let os_type = get_os_id().ok_or(SharedError::PathError)?;
    let config = DotConfig::load_config(package)?;

    if let Some(os) = config.target.get(&os_type) {
        if let RunMode::Install = mode {
            install_dependent_packages(&config.depends_on)?;
        }
        run_script(os.get_script(), &mode, package)?;
        return Ok(());
    }

    let canditates = get_os_id_like().ok_or(ConfigError(
        "You os does not have any fallbacks".to_string(),
    ))?;

    if doty_config.get_fallback() == "once" {
        if let Some(os) = config.target.get(&canditates[0]) {
            println!("Using fallback: {}", canditates[0]);
            if let RunMode::Install = mode {
                install_dependent_packages(&config.depends_on)?;
            }

            run_script(os.get_script(), &mode, package)?;
            return Ok(());
        }
    } else if doty_config.get_fallback() == "full" {
        for cantidate in canditates {
            if let Some(os) = config.target.get(&cantidate) {
                println!("Using fallback: {}", cantidate);
                if let RunMode::Install = mode {
                    install_dependent_packages(&config.depends_on)?;
                }

                run_script(os.get_script(), &mode, package)?;
                return Ok(());
            }
        }
    } else {
        return Err(DotyError::ConfigError("Unknown fallback type".to_string()));
    }
    Err(DotyError::NoConfigForOs)
}

// TODO: Fix stack overflow when package has circular deps
fn install_dependent_packages(deps: &Vec<String>) -> Result<(), DotyError> {
    for dep in deps {
        println!("Installing dependencies");
        install_or_remove(Path::new(dep), &RunMode::Install)?;
    }
    Ok(())
}
