use std::path::Path;

use crate::{
    commands::{install_or_remove, link::link, unlink::unlink},
    config::{
        doty::DotyConfig,
        fs_interact::RunMode::{self, Install, Uninstall},
    },
    error::DotyError,
};

pub fn list_bundles() -> Result<(), DotyError> {
    let config = DotyConfig::load_from_cwd()?;
    let bundles = config.get_bundles();

    match bundles {
        Some(packages) => {
            println!("Bundles:");
            for package in packages {
                println!("\t{package}",)
            }
        }

        None => println!("No bundles are configured"),
    }

    Ok(())
}
pub fn install_or_remove_bundles(name: &str, mode: RunMode) -> Result<(), DotyError> {
    let config = DotyConfig::load_from_cwd()?;
    let bundle = config.get_bundle(name);

    match bundle {
        Some(packages) => {
            for package in packages {
                install_or_remove(Path::new(package), mode)?;
                match mode {
                    Install => link(Path::new(package))?,
                    Uninstall => unlink(Path::new(package))?,
                }
            }
            Ok(())
        }

        None => Err(DotyError::ConfigError(
            "No bundle found with given name".into(),
        )),
    }
}
