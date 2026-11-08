use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dot {
    fedora: Option<DotConfig>,
}

#[derive(Deserialize, Debug)]
struct DotConfig {
    script: String,
}
