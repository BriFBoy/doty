use std::{
    collections::HashMap,
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;

use crate::error::{
    DotyError::{self, ParsError},
    SharedError,
};

#[derive(Deserialize, Debug)]
pub struct DotConfig {
    pub exclude: Vec<String>,
    pub depends_on: Vec<String>,
    #[serde(flatten)]
    pub target: HashMap<String, OsConfig>,
}

impl DotConfig {
    pub fn load_config(package: &Path) -> Result<DotConfig, DotyError> {
        let mut dot_file = Path::join(&current_dir().map_err(|_| SharedError::PathError)?, package);
        dot_file.push("dot.toml");
        let string = fs::read_to_string(dot_file).map_err(|_| ParsError)?;
        let config: DotConfig = toml::from_str(&string).map_err(|_| ParsError)?;
        Ok(config)
    }
}

#[derive(Deserialize, Debug)]
pub struct OsConfig {
    script: PathBuf,
}

impl OsConfig {
    pub fn get_script(&self) -> &Path {
        &self.script
    }
}
