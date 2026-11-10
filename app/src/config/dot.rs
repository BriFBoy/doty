use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dot {
    pub target: HashMap<String, DotConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DotConfig {
    pub install: PathBuf,
    pub uninstall: PathBuf,
}
