use lazy_static::lazy_static;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, path::PathBuf, process, sync::atomic};

static NEXT_ID: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

lazy_static! {
    static ref UNIXTIME: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}

/// Wrapper for testing our binary in integration tests.
///
/// Handles finding the binary and setting clean test directories for us. All of the directories
/// are created under `target` and are cleaned through normal cargo mechanisms.
///
/// Cargo compiles our binary before tests are run and we can run the same binary for integration
/// testing.
///
/// Heavilly inspired and copied from:
/// https://github.com/BurntSushi/xsv/blob/0f58a988016327016378a21bf4e335a41b51b2e9/tests/workdir.rs
pub struct Workdir {
    // where our test executable is inside 'target'
    root: PathBuf,

    // where our tests will generate data and set as the CWD during execution
    pub dir: PathBuf,
}

impl Workdir {
    pub fn new(name: &str) -> Workdir {
        let id = NEXT_ID.fetch_add(1, atomic::Ordering::SeqCst);

        let mut root = env::current_exe()
            .unwrap()
            .parent()
            .expect("integration test executable")
            .to_path_buf();

        // `env::current_exe` sometimes points to deps? I haven't researched why or if it always
        // points.
        if root.ends_with("deps") {
            root.pop();
        }

        let dir = root
            .join(format!("rojekti-it-{}", *UNIXTIME))
            .join(name)
            .join(format!("test-{}", id));

        if let Err(err) = fs::create_dir_all(&dir) {
            panic!("Could not create '{:?}': {}", dir, err);
        }

        Workdir { root, dir }
    }

    pub fn bin(&self) -> PathBuf {
        self.root.join("rojekti")
    }

    pub fn command(&self, sub_command: &str) -> process::Command {
        let mut command = process::Command::new(&self.bin());
        command.current_dir(&self.dir).arg(sub_command);
        command
    }

    pub fn create_dir_all_in_dir(&self, path: PathBuf) -> PathBuf {
        let path = &self.dir.join(path);
        if let Err(err) = fs::create_dir_all(path) {
            panic!("Could not create '{:?}': {}", path, err);
        }
        path.to_path_buf()
    }

    pub fn assert_error(&self, cmd: &mut process::Command) {
        let o = cmd.output().expect("failed to get command output");

        if o.status.success() {
            panic!(
                "\n\n===== {:?} =====\n\
                    command succeeded but expected failure!\
                    \n\ncwd: {}\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }

    pub fn assert_ok(&self, cmd: &mut process::Command) {
        let o = cmd.output().expect("failed to get command output");

        if !o.status.success() {
            panic!(
                "\n\n===== {:?} =====\n\
                    command failed but expected ok!\
                    \n\ncwd: {}\
                    \n\nstatus: {}\
                    \n\nstdout: {}\n\nstderr: {}\
                    \n\n=====\n",
                cmd,
                self.dir.display(),
                o.status,
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
        }
    }

    pub(crate) fn assert_has_file(&self, filename: &str) {
        let path = &self.dir.join(filename);
        assert!(
            path.exists(),
            "expected file {} to exist",
            path.to_str().unwrap()
        )
    }

    pub(crate) fn assert_no_file(&self, filename: &str) {
        let path = &self.dir.join(filename);
        assert!(
            !path.exists(),
            "expected file {} to not exist",
            path.to_str().unwrap()
        )
    }
}
