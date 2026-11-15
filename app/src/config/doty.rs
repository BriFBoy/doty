use std::{env::current_dir, fs, path::Path};

use serde::Deserialize;

use crate::error::{
    DotyError::{self},
    SharedError,
};

#[derive(Deserialize)]
pub struct DotyConfig {
    packages: Packages,
}

impl DotyConfig {
    pub fn load_from_cwd() -> Result<DotyConfig, DotyError> {
        let config_path = Path::join(
            &current_dir().map_err(|_| SharedError::HomeDirError)?,
            "doty.toml",
        );
        if !config_path.exists() {
            return Err(DotyError::ConfigError(
                "Did not found the doty config".to_string(),
            ));
        }
        let config: DotyConfig = toml::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();

        Ok(config)
    }

    pub fn get_fallback(&self) -> &str {
        if let Some(fallback) = &self.packages.fallback {
            fallback
        } else {
            "once"
        }
    }
    pub fn get_conflict(&self) -> &str {
        if let Some(conflict) = &self.packages.conflict {
            conflict
        } else {
            "error"
        }
    }
}

#[derive(Deserialize)]
struct Packages {
    fallback: Option<String>,
    conflict: Option<String>,
}
