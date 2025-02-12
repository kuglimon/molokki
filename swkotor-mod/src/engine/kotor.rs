/// Overrides for Kotor functions.
use log::trace;
use std::{arch::asm, ffi::c_int};
/// Functions for overriding Kotor functionality.
///
/// Calling convention is __stdcall, thus we need to mark all with `extern "system"`.

// TODO(tatu): We could also have these in the engine if we pinned memory? Have to look that up.
//
// Within kotor there's a function that's hardcoded to check for resolutions. It's used in the
// menus and some other place. Modifying this seems to enable all the resolutions a system
// supports.
//
// Original function checks for width and height and returns 0 to reject a resolution and 1 to
// acceppt.
//
// Hence we just accept all resolutions.
#[inline(never)]
#[no_mangle]
pub extern "system" fn filter_resolutions(width: c_int, height: c_int) -> bool {
    // Marker to make it easier to find our function in the sea of instructions.
    unsafe {
        asm!("nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop", "nop",);
    }
    trace!("Asked to filter resolution {width:?}x{height:?}");
    return true;
}
