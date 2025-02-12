use std::{ffi::c_void, io};

use log::trace;
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};

/// Ergonomic RAII wrapper for changing [page protection flags](https://learn.microsoft.com/en-us/windows/win32/memory/memory-protection-constants).
///
/// Use helper function `with_virtual_protect` to wrap a function that needs to change page
/// protection:
///
/// ```rust
/// with_virtual_protect(0x1239845,PAGE_EXECUTE_READWRITE, 5, || {
///     // Do something with with memory
///     Ok(())
/// });
/// ```
///
/// Panics in teardown if flags cannot be reset back to original state.
struct VirtualProtectionGuard {
    /// Start address to which the given bytes should apply
    target_address: usize,
    protection_size: usize,
    original_memory_protection_flag: PAGE_PROTECTION_FLAGS,
}

impl VirtualProtectionGuard {
    unsafe fn new(
        target_address: usize,
        new_page_protection_flags: PAGE_PROTECTION_FLAGS,
        protection_size: usize,
    ) -> io::Result<Self> {
        let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();

        VirtualProtect(
            target_address as *mut c_void,
            protection_size,
            new_page_protection_flags,
            &mut old_protect,
        )
        .map_err(|_| {
            return io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!(
                    "Could not change virtual memory protection to {:?}",
                    new_page_protection_flags
                ),
            );
        })?;

        Ok(VirtualProtectionGuard {
            protection_size,
            target_address,
            original_memory_protection_flag: old_protect,
        })
    }

    unsafe fn teardown(&self) {
        let mut old_protect: PAGE_PROTECTION_FLAGS = Default::default();

        VirtualProtect(
            self.target_address as *mut c_void,
            self.protection_size,
            self.original_memory_protection_flag,
            &mut old_protect,
        )
        .expect("Could reset page protection");
    }
}

impl Drop for VirtualProtectionGuard {
    fn drop(&mut self) {
        unsafe { self.teardown() };
    }
}

unsafe fn with_virtual_protect<F, T>(
    target_address: usize,
    new_page_protection_flags: PAGE_PROTECTION_FLAGS,
    protection_size: usize,
    work: F,
) -> io::Result<T>
where
    F: FnOnce() -> io::Result<T>,
{
    trace!("removing protection");
    let _guard =
        VirtualProtectionGuard::new(target_address, new_page_protection_flags, protection_size)?;
    let res = Ok(work()?);
    trace!("removed protection");
    res
}

pub struct Patch<const COUNT: usize> {
    /// This is for debugging means only
    name: String,

    /// The bytes this patch should apply
    bytes: [u8; COUNT],

    /// Bytes we expect memory to contain before applying. Used to detect if it's safe to apply
    /// patch.
    original_bytes: [u8; COUNT],

    /// Start address to which the given bytes should apply
    target_address: usize,

    required_memory_protection_level: PAGE_PROTECTION_FLAGS,
}

impl Patch<5> {
    pub fn call_instruction_to_function(
        name: String,
        original_bytes: [u8; 5],
        target_address: usize,
        replacement_function: extern "system" fn(i32, i32) -> bool,
    ) -> Patch<5> {
        let replacement_fn_ptr = replacement_function as *const () as usize;
        assert!(usize::BITS == 32, "We should have 32bit pointers");

        let relative_offset = replacement_fn_ptr.wrapping_sub(target_address as usize + 5);

        assert!(
            relative_offset <= u32::MAX as usize,
            "Relative offset overflowed 32-bit limit"
        );

        // The CALL instruction is `E8 <offset>` (5 bytes total)
        let mut patch: [u8; 5] = [0xE8, 0, 0, 0, 0];
        patch[1..].copy_from_slice(&(relative_offset as u32).to_le_bytes());

        Patch {
            name,
            target_address,
            original_bytes,
            bytes: patch,
            required_memory_protection_level: PAGE_EXECUTE_READWRITE,
        }
    }
}

impl<const COUNT: usize> Patch<COUNT> {
    pub fn bytes(
        name: String,
        target_address: usize,
        original_bytes: [u8; COUNT],
        new_bytes: [u8; COUNT],
    ) -> Patch<COUNT> {
        // The CALL instruction is `E8 <offset>` (5 bytes total)
        Patch {
            name,
            target_address,
            original_bytes,
            bytes: new_bytes,
            required_memory_protection_level: PAGE_READWRITE,
        }
    }

    pub unsafe fn can_apply(&self) -> bool {
        let current = std::slice::from_raw_parts_mut(
            self.target_address as *mut u8,
            self.original_bytes.len(),
        );
        current == self.original_bytes
    }

    // FIXME(tatu): should be 'self' not '&self'
    pub unsafe fn apply(&self) -> io::Result<AppliedPatch<COUNT>> {
        with_virtual_protect(
            self.target_address,
            self.required_memory_protection_level,
            self.bytes.len(),
            || {
                trace!("patch start");
                // Save old memory in-case we want to revert
                let old_memory =
                    std::slice::from_raw_parts(self.target_address as *mut u8, self.bytes.len());
                // std::slice::from_raw_parts_mut(self.target_address as *mut u8, self.bytes.len());

                trace!("patch compare {} == {}", old_memory.len(), self.bytes.len());

                assert!(
                    old_memory.len() == self.bytes.len(),
                    "Should have read same amount of old bytes"
                );

                let mut sized_old_memory: [u8; COUNT] = [0; COUNT];
                // sized_old_memory.copy_from_slice(old_memory);

                trace!("copied le slice");

                trace!(
                    "applying patch {} to {:x?}",
                    self.name,
                    std::slice::from_raw_parts_mut(0x006e09a8 as *mut u8, self.bytes.len())
                );

                (self.target_address as *mut u8).copy_from(self.bytes.as_ptr(), self.bytes.len());

                trace!(
                    "applied patch {} with {:x?}",
                    self.name,
                    std::slice::from_raw_parts_mut(0x006e09a8 as *mut u8, self.bytes.len())
                );

                Ok(AppliedPatch {
                    name: self.name.clone(),
                    original_bytes: sized_old_memory,
                    applied_bytes: self.bytes,
                    target_address: self.target_address,
                    required_memory_protection_level: self.required_memory_protection_level,
                })
            },
        )
    }
}

pub struct AppliedPatch<const COUNT: usize> {
    name: String,
    original_bytes: [u8; COUNT],
    applied_bytes: [u8; COUNT],
    target_address: usize,
    required_memory_protection_level: PAGE_PROTECTION_FLAGS,
}

impl<const COUNT: usize> AppliedPatch<COUNT> {
    #[allow(dead_code)]
    pub unsafe fn revert(self) -> io::Result<()> {
        with_virtual_protect(
            self.target_address,
            self.required_memory_protection_level,
            self.applied_bytes.len(),
            || {
                trace!(
                    "reverting patch {} from {:x?} to {:x?}",
                    self.name,
                    self.applied_bytes,
                    self.original_bytes
                );

                (self.target_address as *mut u8)
                    .copy_from(self.original_bytes.as_ptr(), self.original_bytes.len());

                trace!(
                    "reverted patch {} from {:x?} to {:x?}",
                    self.name,
                    self.applied_bytes,
                    self.original_bytes
                );

                Ok(())
            },
        )
    }
}
