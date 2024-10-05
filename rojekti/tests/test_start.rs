use std::path::PathBuf;

use workdir::Workdir;
mod workdir;

#[test]
fn starts_new_project_without_config() {
    let wk = Workdir::new("start-new");
    let mut cmd = wk.command("start");
    wk.create_dir_all_in_dir(PathBuf::from("rojekti"));

    cmd.env("XDG_CONFIG_HOME", &wk.dir)
        .env("PATH", format!("{}:$PATH", wk.support_script_dir()))
        .env("EDITOR", wk.support_script_path("autosavevim"))
        .arg("test-proj");

    wk.assert_ok(&mut cmd)
}
