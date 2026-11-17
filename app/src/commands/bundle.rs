use std::path::Path;

use crate::{
    commands::install_or_remove,
    config::{
        doty::DotyConfig,
        fs_interact::RunMode::{self, Install},
    },
    error::DotyError,
};

pub fn list_bundles() -> Result<(), DotyError> {
    let config = DotyConfig::load_from_cwd()?;
    let bundles = config.get_bundles();

    match bundles {
        Some(e) => {
            println!("Bundles:");
            for f in e {
                println!("\t{}", f)
            }
        }

        None => println!("No bundles are configured"),
    }

    Ok(())
}
pub fn install_or_remove_bundles(name: &str, mode: &RunMode) -> Result<(), DotyError> {
    let config = DotyConfig::load_from_cwd()?;
    let bundle = config.get_bundle(name);

    match bundle {
        Some(e) => {
            for package in e {
                install_or_remove(Path::new(package), mode)?;
            }
        }

        None => println!("No bundle with given name"),
    }

    Ok(())
}
