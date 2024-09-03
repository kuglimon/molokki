use std::{
    env,
    path::{Path, PathBuf},
};

pub struct RuntimeEnvironment {
    pub layout_path: PathBuf,
    pub editor: String,
    pub pwd: PathBuf,
}

impl RuntimeEnvironment {
    pub fn from_env() -> Self {
        let config_home = env::var("XDG_CONFIG_HOME").map_or_else(
            |_| {
                Path::new(&env::var("HOME").expect("HOME is not set, no config directory to use"))
                    .join(".config")
            },
            |xdg_config_home| Path::new(&xdg_config_home).to_path_buf(),
        );

        let layout_path = config_home.join("rojekti");
        let editor = env::var("EDITOR").unwrap_or("vi".to_string());
        // TODO(tatu): this might not be required for all cases
        let pwd = env::current_dir().expect("PWD does not exist, cannot continue");

        RuntimeEnvironment {
            layout_path,
            editor,
            pwd,
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy; // 1.4.0
    use std::env;
    use std::sync::Mutex;

    use super::RuntimeEnvironment;

    static THE_RESOURCE: Lazy<Mutex<()>> = Lazy::new(Mutex::default);
    type TestResult<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

    fn reset_env() {
        env::remove_var("XDG_CONFIG_HOME");
        env::remove_var("EDITOR");
        env::remove_var("HOME");
    }

    #[test]
    fn it_uses_config_from_environment_variables() -> TestResult {
        let _shared = THE_RESOURCE.lock()?;

        env::set_var("EDITOR", "vim");
        env::set_var("HOME", "/tmp");

        let runtime_environment = RuntimeEnvironment::from_env();

        assert_eq!(runtime_environment.editor, "vim");
        assert_eq!(
            runtime_environment.layout_path.to_str(),
            Some("/tmp/.config/rojekti")
        );

        reset_env();

        Ok(())
    }

    #[test]
    fn it_prioritizes_xdg_config_home() -> TestResult {
        let _shared = THE_RESOURCE.lock()?;

        env::set_var("EDITOR", "vim");
        env::set_var("HOME", "/tmp");
        env::set_var("XDG_CONFIG_HOME", "/what/.config");

        let runtime_environment = RuntimeEnvironment::from_env();

        assert_eq!(runtime_environment.editor, "vim");
        assert_eq!(
            runtime_environment.layout_path.to_str(),
            Some("/what/.config/rojekti")
        );

        reset_env();

        Ok(())
    }
}
