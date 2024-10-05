use std::path::PathBuf;

use workdir::Workdir;
mod workdir;

#[test]
fn list_files_fails_on_missing_config_directory() {
    let wk = Workdir::new("list-missing-config-dir");
    let mut cmd = wk.command("list");
    cmd.env("XDG_CONFIG_HOME", &wk.dir).env("HOME", &wk.dir);

    wk.assert_error(&mut cmd)
}

#[test]
fn lists_files_in_home() {
    let wk = Workdir::new("list-empty-in-$HOME");
    wk.create_dir_all_in_dir(PathBuf::from(".config/rojekti"));

    let mut cmd = wk.command("list");
    cmd.env("HOME", &wk.dir).env_remove("XDG_CONFIG_HOME");

    wk.assert_ok(&mut cmd);
}

#[test]
fn lists_files_in_with_custom_xdg_home() {
    let wk = Workdir::new("list-in-$XDG_HOME");
    let mut path = wk.create_dir_all_in_dir(PathBuf::from(".what/rojekti"));
    let mut cmd = wk.command("list");
    path.pop();
    cmd.env("XDG_CONFIG_HOME", path);

    wk.assert_ok(&mut cmd);
}
