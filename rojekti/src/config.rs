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
        let home_path = env::var("HOME").expect("HOME is not set on env, cannot continue");
        // TODO(tatu): Doesn't support .config in another directory, but I never change this, meh.main
        let xdg_config_home = Path::new(&home_path).join(".config");
        let layout_home = xdg_config_home.join("tmuxinator");

        Ok(Config {
            layout_path: layout_home,
        })
    }
}
