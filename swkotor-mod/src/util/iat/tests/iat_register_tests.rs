/// Tests that we get some response from IAT hooking
///
/// To know if IAT is working (somewhat), try to hook into
/// a known function. And to be sure that we know a function
/// exists in OUR IAT table, declare it ourselves.
use crate::live_test;
use crate::test_assert;

use mktemp::Temp;
use std::error::Error;
use std::ffi::c_void;
use std::ffi::CString;
use std::path::Path;
use std::ptr;
use std::ptr::null;
use std::str::FromStr;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;

use super::install_plt_hook;

/// The type for CreateFileA
type CreateFileAFn = unsafe extern "system" fn(
    lpFileName: *const i8,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: *mut core::ffi::c_void,
    dwCreationDisposition: u32,
    dwFlagsAndAttributes: u32,
    hTemplateFile: HANDLE,
) -> HANDLE;

// The function that resides in the kernel32.dll, the one
// that old C programs (including swkotor) call
#[link(name = "kernel32")]
extern "system" {
    pub fn CreateFileA(
        lpFileName: *const i8,
        dwDesiredAccess: u32,
        dwShareMode: u32,
        lpSecurityAttributes: *const c_void,
        dwCreationDisposition: u32,
        dwFlagsAndAttributes: u32,
        hTemplateFile: HANDLE,
    ) -> HANDLE;
}

/// Wraps the winapi CloseHandle
fn close_handle(handle: HANDLE) {
    if handle.0 == INVALID_HANDLE_VALUE.0 {
        return;
    }

    let _ = unsafe { CloseHandle(handle).or(Err("Failed to close handle")) };
}

/// Struct to store the handle and call CloseHandle if needed. Th
struct HandleDropper {
    handle: HANDLE,
}

impl Drop for HandleDropper {
    fn drop(&mut self) {
        if self.handle.0 != INVALID_HANDLE_VALUE.0 {
            close_handle(self.handle);
        }
    }
}

// Windows constants we might need:
const GENERIC_READ: u32 = 0x80000000;
const FILE_SHARE_READ: u32 = 0x00000001;
const OPEN_EXISTING: u32 = 3;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x00000080;

/// Calls the CreateFileA as our DLL sees it.
fn call_default_creatfilea(file_path: &Path) -> Result<HandleDropper, Box<dyn Error>> {
    // Convert a Rust string path to a null-terminated C string
    log::trace!("Calling the default CreateFileA");
    let binding = file_path.to_string_lossy().to_string();
    let path_string = binding.as_str();
    let filename = CString::from_str(path_string)?;

    // Perform the call
    let handle = unsafe {
        CreateFileA(
            filename.as_ptr(),
            GENERIC_READ, // or some other desiredAccess
            FILE_SHARE_READ,
            ptr::null(),   // no security attributes
            OPEN_EXISTING, // or CREATE_ALWAYS, etc.
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(null::<*mut c_void>() as *mut c_void),
        )
    };
    Ok(HandleDropper { handle: handle })
}

fn print_function_head(addr: *const u8, len: usize) {
    if !log::log_enabled!(log::Level::Debug) {
        return;
    }

    let mut str = "".to_string();
    for i in 0..len {
        let val = unsafe { *addr.add(i) };
        str = format!("{str} {:02X}", val);
    }
    log::debug!("Function bytes: {str}");
}

/// Call the pointer value of CreateFileA
fn call_createfilea_ptr(
    file_path: &Path,
    function: *const CreateFileAFn,
) -> Result<HandleDropper, Box<dyn Error>> {
    log::trace!("Calling CreateFileA via ptr");

    let binding = file_path.to_string_lossy().to_string();
    let path_string = binding.as_str();
    let filename = CString::from_str(path_string)?;

    let createfilea_ptr = function;
    log::debug!(
        "Calling the CreateFileA ptr at address: {:p} with content:",
        createfilea_ptr
    );
    print_function_head(createfilea_ptr as *const u8, 16);

    // Convert the pointer to our type. There is no proper way to dereference the *const T
    let createfilea_ptr =
        unsafe { std::mem::transmute::<usize, CreateFileAFn>(createfilea_ptr as usize) };

    let handle = unsafe {
        createfilea_ptr(
            filename.as_ptr(),
            GENERIC_READ, // or some other desiredAccess
            FILE_SHARE_READ,
            null::<*mut c_void>() as *mut c_void, // no security attributes
            OPEN_EXISTING,                        // or CREATE_ALWAYS, etc.
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(null::<*mut c_void>() as *mut c_void),
        )
    };

    Ok(HandleDropper { handle: handle })
}

// Just call the default createfilea. This is a bad test
// as it is a test to test tests.
live_test! {
    fn test_call_createfilea() {
        let temp_file = Temp::new_file()?;
        let handle = call_default_creatfilea(temp_file.as_path())?;
        test_assert!(handle.handle.0 != INVALID_HANDLE_VALUE.0, "Default CreateFileA call failed");
    }
}

// Call the default createfilea via a pointer. Only gives
// an example for developers to handle pointer calling
live_test! {
    fn test_call_createfilea_ptr() {
        let temp_file = Temp::new_file()?;
        let handle = call_createfilea_ptr(temp_file.as_path(), CreateFileA as *const CreateFileAFn)?;
        test_assert!(handle.handle.0 != INVALID_HANDLE_VALUE.0, "Pointer CreateFileA call failed");
    }
}

// Check that the default function works after dropping.
// conceptually relies on the above tests to make sense.
// If this test fails, essentially any drop in the actual
// code will cause the program to crash.
live_test! {
    fn test_call_createfilea_after_drop() {
        let temp_file = Temp::new_file()?;
        let module_name = crate::DLL_MOCK_SELF;
        // Our hooked CreateFileA implementation. Closures
        // cannot be used with `unsafe extern`
        unsafe extern "system" fn test_createfilea(
            _lp_file_name: *const i8,
            _dw_desired_access: u32,
            _dw_share_mode: u32,
            _lp_security_attrs: *mut core::ffi::c_void,
            _dw_creation_disposition: u32,
            _dw_flags_and_attrs: u32,
            _h_template_file: HANDLE
        ) -> HANDLE {
            HANDLE(null::<*mut c_void>() as *mut c_void)
        }

        let store = install_plt_hook::<CreateFileAFn>(&module_name.to_string(), "CreateFileA", &(
            test_createfilea as CreateFileAFn
        ))?;

        drop(store);

        // We will crash here if we fail
        log::trace!("Calling call_default_creatfilea for testing");
        let handle = call_default_creatfilea(temp_file.as_path())?;
        log::trace!("Called call_default_creatfilea successfully");

        // The mock will just return a null, so by all accounts, this
        // should return something valid
        test_assert!(handle.handle.0 != INVALID_HANDLE_VALUE.0, "After-drop CreateFileA call failed");
    }
}

// Finally, replace and call the new function
live_test! {
    fn test_call_after_reading() {
        let temp_file = Temp::new_file()?;
        let module_name = crate::DLL_MOCK_SELF;
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

        // The tests are not meant to be run multiple times but it's a good
        // practice to know the state of everything at the start of a test.
        CALL_COUNT.store(0, atomic::Ordering::Relaxed);

        // Our hooked CreateFileA implementation
        unsafe extern "system" fn test_createfilea(
            _lp_file_name: *const i8,
            _dw_desired_access: u32,
            _dw_share_mode: u32,
            _lp_security_attrs: *mut core::ffi::c_void,
            _dw_creation_disposition: u32,
            _dw_flags_and_attrs: u32,
            _h_template_file: HANDLE
        ) -> HANDLE {
            CALL_COUNT.store(1, atomic::Ordering::Relaxed);
            INVALID_HANDLE_VALUE
        }

        let _store = install_plt_hook::<CreateFileAFn>(&module_name.to_string(), "CreateFileA", &(
            test_createfilea as CreateFileAFn
        ))?;

        log::trace!("Calling call_default_creatfilea for testing");
        // The default createfilea should now be replaced. Might crash if a bug.
        let handle = call_default_creatfilea(temp_file.as_path())?;
        log::trace!("Called call_default_creatfilea successfully");

        test_assert!(CALL_COUNT.load(atomic::Ordering::Relaxed) == 1);
        test_assert!(handle.handle.0 == INVALID_HANDLE_VALUE.0, "Replacing CreateFileA call failed");
    }
}

// Test that we can call the stored pointer of the original function
live_test! {
    fn test_call_real_fn() {
        let temp_file = Temp::new_file()?;
        let module_name = crate::DLL_MOCK_SELF;
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

        // The tests are not meant to be run multiple times but it's a good
        // practice to know the state of everything at the start of a test.
        CALL_COUNT.store(0, atomic::Ordering::Relaxed);

        // Our hooked CreateFileA implementation. This one should
        // return a failure, while the real one should succeed
        unsafe extern "system" fn test_createfilea(
            _lp_file_name: *const i8,
            _dw_desired_access: u32,
            _dw_share_mode: u32,
            _lp_security_attrs: *mut core::ffi::c_void,
            _dw_creation_disposition: u32,
            _dw_flags_and_attrs: u32,
            _h_template_file: HANDLE
        ) -> HANDLE {
            CALL_COUNT.store(1, atomic::Ordering::Relaxed);
            INVALID_HANDLE_VALUE
        }

        let store = install_plt_hook::<CreateFileAFn>(&module_name.to_string(), "CreateFileA", &(
            test_createfilea as CreateFileAFn
        ))?;

        log::trace!("Calling call_createfilea_ptr for testing");
        // The default createfilea should now be replaced. Might crash if a bug.
        let handle = call_createfilea_ptr(temp_file.as_path(), store.get_fn() as *const CreateFileAFn)?;
        log::trace!("Called call_createfilea_ptr successfully");

        // Our hook was not called in this test, so there should not pass.
        test_assert!(CALL_COUNT.load(atomic::Ordering::Relaxed) != 1);
        test_assert!(handle.handle.0 != INVALID_HANDLE_VALUE.0, "Replacing CreateFileA call failed");
    }
}
