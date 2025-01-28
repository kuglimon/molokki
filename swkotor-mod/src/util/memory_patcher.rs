use std::ffi::c_void;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_PROTECTION_FLAGS, PAGE_READWRITE};

/// Overwrites `new_data` bytes at `address`, temporarily changing the page
/// to writable if needed, then restoring the original protection.
pub unsafe fn patch_memory(address: *mut u8, new_data: &[u8]) {
    let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();
    // Make memory writable
    let _ = VirtualProtect(
        address as *mut c_void,
        new_data.len(),
        PAGE_READWRITE,
        &mut old_protect,
    )
    .map_err(|e| {
        log::error!("Failed to break memory protection");
        e
    })
    .unwrap();

    // Write new data
    address.copy_from(new_data.as_ptr(), new_data.len());
    // Restore original protection
    let _ = VirtualProtect(
        address as *mut c_void,
        new_data.len(),
        old_protect,
        &mut old_protect,
    )
    .map_err(|e| {
        log::error!("Failed to break memory protection");
        e
    })
    .unwrap();
}
