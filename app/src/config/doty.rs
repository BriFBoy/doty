use std::{collections::BTreeMap, env::current_dir, fs, path::Path};

use serde::Deserialize;

use crate::error::{
    DotyError::{self},
    SharedError,
};

#[derive(Deserialize)]
pub struct DotyConfig {
    packages: Packages,
    bundles: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Deserialize)]
struct Packages {
    fallback: Option<String>,
    conflict: Option<String>,
}

impl DotyConfig {
    pub fn load_from_cwd() -> Result<DotyConfig, DotyError> {
        let config_path = Path::join(
            &current_dir().map_err(|_| SharedError::HomeDirError)?,
            "doty.toml",
        );

        if !config_path.exists() {
            return Ok(DotyConfig::default());
        }

        let config: DotyConfig = toml::from_str(
            &fs::read_to_string(&config_path)
                .map_err(|e| SharedError::ReadError(config_path.clone(), e))?,
        )
        .map_err(|_| DotyError::ConfigError(format!("Unable to pars {}", config_path.display())))?;

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
    pub fn get_bundle(&self, key: &str) -> Option<&Vec<String>> {
        match &self.bundles {
            Some(btreemap) => btreemap.get(key),
            None => None,
        }
    }
    pub fn get_bundles(&self) -> Option<Vec<&String>> {
        self.bundles
            .as_ref()
            .map(|btreemap| btreemap.keys().collect())
    }
}
impl Default for DotyConfig {
    fn default() -> Self {
        Self {
            packages: Packages {
                fallback: Some("once".to_string()),
                conflict: Some("error".to_string()),
            },
            bundles: Some(BTreeMap::<String, Vec<String>>::new()),
        }
    }
}
