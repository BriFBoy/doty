use std::{env::current_dir, fs, path::Path};

use which_os::{get_os_type, ostypes::Os};

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

    let os_type = get_os_type().ok_or(DotyError::SharedError(SharedError::PathError))?;

    if let Some(os) = config.fedora
        && os_type == Os::Fedora
    {
        println!("running {}", os.script);
    }

    Ok(())
}
