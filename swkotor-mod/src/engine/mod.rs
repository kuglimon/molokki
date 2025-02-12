mod dinput8_dll;
mod kotor;
use std::{
    sync::{LazyLock, Mutex},
    thread,
    time::Duration,
};

use dinput8_dll::DirectInput8CreateFn;
use env_logger::Env;
use kotor::filter_resolutions;
use log::trace;

use crate::liveqa;
use crate::{
    mem::Patch,
    system::dll_loader::{get_proc_address, load_system_library_a, DllLibrary},
};

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

        unsafe {
            let patches = vec![Patch::call_instruction_to_function(
                "filter_resolutions - 0x006e09a8".to_string(),
                [0xe8, 0x03, 0xd9, 0xf0, 0xff],
                0x006e09a8,
                filter_resolutions,
            )];

            // SteamWorks DRM encrypts the executable, postpone patching until it's done. I haven't
            // found a better way than to just poll in a quick loop.
            //
            // If you look at the disassembly of the Steam executable main function, you'll find it
            // obfuscated. During boot it'll unwind this obfuscation. Polling in a loop means we
            // miss the whole initialization and might cause bugs due to timing issues.
            //
            // TODO(tatu): Maybe we could hook to bink dll as videos are played first?
            let _handle = thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));

                if patches.iter().all(|p| p.can_apply()) {
                    trace!("Safe to apply patches, applying");
                    patches.iter().for_each(|p| {
                        trace!("Applying patch");
                        p.apply().expect("patch should have applied");
                    });
                    break;
                } else {
                    trace!("Patches don't match, are you on steam?");
                }
            });
        }

        SWKotorModEngine {
            direct_input8_create_fn,
        }
    }
}

fn setup_logging() {
    // Dump all logs to a file. For that, we'll need a pipe to pass to env_logger.
    let file = std::fs::File::create("swkotor-mod.log")
        .expect("Failed to initialize logging file for piping.");
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("trace"));
    builder.target(env_logger::Target::Pipe(Box::new(file)));
    builder.init();
}

// TODO(tatu): Provide a more ergonomic function for this?
pub static SW_KOTOR_MOD_ENGINE: LazyLock<Mutex<SWKotorModEngine>> = LazyLock::new(|| {
    // Is this safe to do here?
    setup_logging();

    liveqa::runner::run_live_qa_tests();

    Mutex::new(SWKotorModEngine::new())
});
