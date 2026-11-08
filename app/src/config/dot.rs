use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dot {
    pub fedora: Option<DotConfig>,
}

#[derive(Deserialize, Debug)]
pub struct DotConfig {
    pub script: String,
}
