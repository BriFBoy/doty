use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dot {
    #[serde(flatten)]
    pub target: HashMap<String, DotConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DotConfig {
    script: PathBuf,
}

impl DotConfig {
    pub fn get_script(&self) -> &Path {
        &self.script
    }
}
