pub mod entry;

use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::*;
use env_logger::{self, Env};
use log::trace;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn setup_logging() {
    // Dump all logs to a file. For that, we'll need a pipe to pass to env_logger.
    let file = std::fs::File::create("swkotor-mod.log").expect("Failed to initialize logging file for piping.");
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("trace"));
    builder.target(env_logger::Target::Pipe(Box::new(file)));
    builder.init();
}

fn attach() {
    setup_logging();
    trace!("Hello from the DLL!");
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            attach();
        },
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
