use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dot {
    pub fedora: Option<DotConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DotConfig {
    pub install: PathBuf,
    pub uninstall: PathBuf,
}
