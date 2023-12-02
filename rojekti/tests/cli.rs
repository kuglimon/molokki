use assert_cmd::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

struct Setup {
    pub test_dir: PathBuf,
}

impl Setup {
    fn init() -> Setup {
        let temp = env::temp_dir();
        let directory = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        let temp = temp.join(directory);

        fs::create_dir_all(&temp).expect(&format!(
            "could not create test dir {:?}",
            temp.to_string_lossy()
        ));

        Setup {
            test_dir: Path::new(&temp).to_path_buf(),
        }
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.test_dir).expect("could not remove test dir");
    }
}

#[test]
fn list_files_fails_on_missing_config_directory() -> Result<(), Box<dyn std::error::Error>> {
    let setup = Setup::init();

    let mut cmd = Command::cargo_bin("rojekti")?;

    cmd.env("XDG_CONFIG_HOME", &setup.test_dir)
        .env("HOME", &setup.test_dir)
        .arg("list");

    cmd.assert().failure();

    Ok(())
}

#[test]
fn lists_files_in_without_custom_xdg_home() -> Result<(), Box<dyn std::error::Error>> {
    let setup = Setup::init();

    let temp = &setup.test_dir.join(".config");

    println!("vaasdfasdf {:?}", temp.join("rojekti"));

    fs::create_dir_all(&temp.join("rojekti")).expect(&format!(
        "could not create config dir {:?}",
        temp.to_string_lossy()
    ));

    let mut cmd = Command::cargo_bin("rojekti")?;

    cmd.env("HOME", &setup.test_dir).arg("list");

    cmd.assert().success();

    Ok(())
}

#[test]
fn lists_files_in_with_custom_xdg_home() -> Result<(), Box<dyn std::error::Error>> {
    let setup = Setup::init();

    let temp = &setup.test_dir.join(".what");

    fs::create_dir_all(&temp.join("rojekti")).expect(&format!(
        "could not create config dir {:?}",
        temp.to_string_lossy()
    ));

    let mut cmd = Command::cargo_bin("rojekti")?;

    cmd.env("XDG_CONFIG_HOME", &temp)
        .env("HOME", &setup.test_dir)
        .arg("list");

    cmd.assert().success();

    Ok(())
}
