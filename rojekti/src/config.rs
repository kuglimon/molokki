use std::{
    env,
    path::{Path, PathBuf},
};

use crate::error::Error;
use crate::error::ErrorKind;

#[derive(Debug)]
pub struct RuntimeEnvironment {
    pub layout_path: PathBuf,
    pub editor: String,
    pub pwd: PathBuf,
}

impl RuntimeEnvironment {
    /// Load runtime environment from environment variables.
    ///
    /// Following variables are used: `XDG_CONFIG_HOME` as the configuration directory. Fallback to
    /// using `HOME` if `XDG_CONFIG_HOME` is not found. PWD is fetched based on rust std.
    pub fn from_env() -> Result<Self, Error> {
        let config_home = env::var("XDG_CONFIG_HOME")
            .map(|xdg_config_home| Path::new(&xdg_config_home).to_path_buf())
            .or_else(|_| {
                env::var("HOME").map(|home| Path::new(&home).join(".config").to_path_buf())
            })
            .map_err(|_| {
                Error::new(ErrorKind::RuntimeError(
                    "Broken runtime environment: HOME or XDG_CONFIG_HOME set".to_string(),
                ))
            })?;

        let layout_path = config_home.join("rojekti");
        let editor = env::var("EDITOR").unwrap_or("vi".to_string());
        // TODO(tatu): this might not be required for all cases
        let pwd = env::current_dir().expect("PWD does not exist, cannot continue");

        Ok(RuntimeEnvironment {
            layout_path,
            editor,
            pwd,
        })
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy; // 1.4.0
    use std::env;
    use std::sync::Mutex;

    use super::RuntimeEnvironment;

    // FIXME(tatu): There's something wonky here as these tests will randomly fail.
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

        let runtime_environment = RuntimeEnvironment::from_env()
            .expect("should have been able to parse runtime environment");

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

        let runtime_environment = RuntimeEnvironment::from_env()
            .expect("should have been able to parse runtime environment");

        assert_eq!(runtime_environment.editor, "vim");
        assert_eq!(
            runtime_environment.layout_path.to_str(),
            Some("/what/.config/rojekti")
        );

        reset_env();

        Ok(())
    }

    #[test]
    fn it_returns_an_error_when_config_dir_missing() -> TestResult {
        let _shared = THE_RESOURCE.lock()?;

        env::set_var("EDITOR", "vim");

        let result = RuntimeEnvironment::from_env();

        // FIXME(tatu): This should test that the error type is correct as well
        assert!(result.is_err());

        reset_env();

        Ok(())
    }
}
