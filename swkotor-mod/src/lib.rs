pub mod engine;
pub mod liveqa;
pub mod system;

use engine::SW_KOTOR_MOD_ENGINE;
use log::trace;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::*;

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    {
        // Touch the engine to trigger initialize. This is safe to do multiple times. We'll want to
        // do it before anything else to initialize logging. DllMain can be called with JUST
        // deattach in error cases.
        //
        // TODO(tatu): Maybe we should still tell the engine if we're starting or already shutting
        // down? Right now it'll load libraries in case of detach and might fail again.
        let _unused = SW_KOTOR_MOD_ENGINE.lock().unwrap();
    }

    match call_reason {
        DLL_PROCESS_ATTACH => {
            trace!("Attaching dll");
        }
        DLL_PROCESS_DETACH => {
            trace!("Detaching dll or dll loading failed early");
        }
        // We can ignore these safely
        DLL_THREAD_ATTACH | DLL_THREAD_DETACH => (),
        _ => {
            trace!("Unknown dll call reason {call_reason:?}");
            panic!("Unknown dll call reason {call_reason:?}");
        }
    };

    true
}
