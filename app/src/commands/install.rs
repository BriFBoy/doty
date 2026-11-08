use std::{env::current_dir, fs, path::Path};

use crate::{
    config::dot::Dot,
    error::{DotyError, SharedError},
};

pub fn install(package: &Path) -> Result<(), DotyError> {
    let mut dot_file = Path::join(
        &current_dir().map_err(|_| DotyError::SharedError(SharedError::PathError))?,
        package,
    );
    dot_file.push("dot.toml");
    let string =
        fs::read_to_string(dot_file).map_err(|_| DotyError::SharedError(SharedError::PathError))?;
    let config: Dot =
        toml::from_str(&string).map_err(|_| DotyError::SharedError(SharedError::PathError))?;

    println!("config: {:?}", config);
    Ok(())
}
