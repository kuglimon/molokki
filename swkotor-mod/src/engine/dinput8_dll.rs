use log::trace;
use std::ffi::c_void;
use windows::core::{GUID, HRESULT};
use windows::Win32::Foundation::HINSTANCE;

use crate::SW_KOTOR_MOD_ENGINE;

use super::SWKotorModEngine;

pub type REFIID = *const GUID;
pub type LPUNKNOWN = *mut core::ffi::c_void;

pub type DirectInput8CreateFn = unsafe extern "system" fn(
    HINSTANCE,
    u32,
    *const GUID,
    *mut *mut c_void,
    *mut c_void,
) -> HRESULT;

// "system" calling convention is correct for most Windows API functions.
#[no_mangle]
pub extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dw_version: u32,
    riidltf: REFIID,
    ppv_out: *mut *mut c_void,
    punk_outer: LPUNKNOWN,
) -> HRESULT {
    trace!("Calling original DirectInput8Create from wrapper");
    SW_KOTOR_MOD_ENGINE
        .lock()
        .unwrap()
        .direct_input8_create(hinst, dw_version, riidltf, ppv_out, punk_outer)
}

impl SWKotorModEngine {
    pub fn direct_input8_create(
        &self,
        hinst: HINSTANCE,
        dw_version: u32,
        riidltf: REFIID,
        ppv_out: *mut *mut c_void,
        punk_outer: LPUNKNOWN,
    ) -> HRESULT {
        trace!("Calling original DirectInput8Create from wrapper");
        unsafe { (&self.direct_input8_create_fn)(hinst, dw_version, riidltf, ppv_out, punk_outer) }
    }
}
