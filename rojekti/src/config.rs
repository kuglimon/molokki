use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

pub struct Config {
    pub layout_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let config_home = env::var("XDG_CONFIG_HOME").map_or_else(
            |_| {
                Path::new(&env::var("HOME").expect("HOME is not set, no config directory to use"))
                    .join(".config")
            },
            |xdg_config_home| Path::new(&xdg_config_home).to_path_buf(),
        );

        let layout_path = config_home.join("rojekti");

        Ok(Config { layout_path })
    }
}
