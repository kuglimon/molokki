mod dinput8_dll;
use std::sync::{LazyLock, Mutex};

use dinput8_dll::DirectInput8CreateFn;
use env_logger::Env;
use log::trace;

use crate::liveqa;
use crate::system::dll_loader::{get_proc_address, load_system_library_a, DllLibrary};

// Holds the global state of our mod engine.
//
// Throughout the sources you'll find the plain windows functions in pascal case and snake case.
// The distinction here has been that pascal case functions are the raw windows-rs api functions or
// wrappers for Kotor functions while snake case ones offer a more rust like ergonomic API, for the
// same functionality. The snake case APIs should be safer and you should always use them.
//
// Engine overrides libraries and functions Kotor uses. Within this module you can find '_dll.rs'
// files, which define what parts of the dll are overriden. For now all the used functions need to
// be overriden, later on through IAT only a handful might need to be. Engine implementation for
// overriden functions is done on the corresponding dll files.
//
// XXX(tatu): Is there away to avoid global state?
#[derive(Debug)]
pub struct SWKotorModEngine {
    direct_input8_create_fn: DirectInput8CreateFn,
}

impl SWKotorModEngine {
    pub fn new() -> Self {
        // FIXME(tatu): There's now a disjoint in where modules are defined and how they are loaded
        // here. Implementation happens in the 'dll.rs' files but loading is still done here.
        trace!("Loading engine libraries");
        let dinput8_base_address = load_system_library_a(DllLibrary::Dinput8);
        let direct_input8_create_fn = get_proc_address(dinput8_base_address, "DirectInput8Create");
        trace!("Done loading engine libraries");

        SWKotorModEngine {
            direct_input8_create_fn,
        }
    }
}

fn setup_logging() {
    // fs::write("swkotor-mod.log", "creating logging").expect("Unable to write file");
    // Dump all logs to a file. For that, we'll need a pipe to pass to env_logger.
    let file = std::fs::File::create("swkotor-mod.log")
        .expect("Failed to initialize logging file for piping.");
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("trace"));
    builder.target(env_logger::Target::Pipe(Box::new(file)));
    builder.init();
    // fs::write("swkotor-mod.log", "logging ok").expect("Unable to write file");
}

// TODO(tatu): Provide a more ergonomic function for this?
pub static SW_KOTOR_MOD_ENGINE: LazyLock<Mutex<SWKotorModEngine>> = LazyLock::new(|| {
    // Is this safe to do here?
    setup_logging();

    liveqa::runner::run_live_qa_tests();

    Mutex::new(SWKotorModEngine::new())
});
