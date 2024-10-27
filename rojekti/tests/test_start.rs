use std::{path::PathBuf, process};

use workdir::Workdir;
mod workdir;

#[test]
fn starts_new_project_without_config() {
    let wk = Workdir::new("start-new");
    let mut cmd = wk.command("start");
    wk.create_dir_all_in_dir(PathBuf::from("rojekti"));

    wk.assert_no_file("rojekti/test-proj.yml");

    cmd.env("XDG_CONFIG_HOME", &wk.dir)
        .env("EDITOR", "autosavevim")
        .arg("test-proj");

    wk.assert_ok(&mut cmd);
    wk.assert_has_file("rojekti/test-proj.yml");
}

// FIXME(tatu): tmux support script calls tmux leading to infinite loop. Maybe this should be done
// at the nix flake level?
// #[test]
// fn starts_project_existing_rojekti_config() {
//     let wk = Workdir::new("start-new");
//     let mut cmd = wk.command("start");
//     wk.create_dir_all_in_dir(PathBuf::from("rojekti"));
//
//     wk.assert_no_file("rojekti/test-proj.yml");
//
//     // should create a new configuration
//     cmd.env("XDG_CONFIG_HOME", &wk.dir)
//         .env("EDITOR", "autosavevim")
//         .arg("test-proj");
//
//     wk.assert_ok(&mut cmd);
//     wk.assert_has_file("rojekti/test-proj.yml");
//
//     println!("here here here");
//
//     // should start a tmux session
//     let mut cmd = wk.command("start");
//     cmd.env("XDG_CONFIG_HOME", &wk.dir)
//         .env("EDITOR", "autosavevim")
//         .arg("test-proj");
//     wk.assert_error(&mut cmd);
// }
