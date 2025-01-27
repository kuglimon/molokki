use super::common::{install_plt_hook, IatStore};

use std::error::Error;
use std::ffi::CStr;
use std::sync::LazyLock;
use std::sync::Mutex;
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};

type CreateFileAFn = unsafe extern "system" fn(
    lpFileName: *const i8,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: *mut core::ffi::c_void,
    dwCreationDisposition: u32,
    dwFlagsAndAttributes: u32,
    hTemplateFile: HANDLE,
) -> HANDLE;

/// Store the real function pointer
///
/// Encapsulate this in order to safely use it, as
/// "creating a shared reference to mutable static is discouraged". Note
/// that the mutex only secures the store, but should not block multi-thread
/// createfilea creation.
///
/// This might require a more generic wrapper around this concept to be usable
/// elsewhere. This is hard to grasp.
///
static REAL_CREATEFILEA: LazyLock<Mutex<Option<IatStore<CreateFileAFn>>>> =
    LazyLock::new(|| Mutex::new(None));

fn set_real_createfilea(store: IatStore<CreateFileAFn>) -> Result<(), Box<dyn Error>> {
    let mut guard = REAL_CREATEFILEA.lock()?;
    *guard = Some(store);
    Ok(())
}

fn get_real_createfilea() -> Result<IatStore<CreateFileAFn>, Box<dyn Error>> {
    let guard = REAL_CREATEFILEA.lock()?;
    match &*guard {
        None => Err("Bug. No CreateFileA hook stored".into()),
        Some(store) => Ok(store.clone()),
    }
}

// Our hooked CreateFileA implementation
unsafe extern "system" fn my_createfilea(
    lp_file_name: *const i8,
    dw_desired_access: u32,
    dw_share_mode: u32,
    lp_security_attrs: *mut core::ffi::c_void,
    dw_creation_disposition: u32,
    dw_flags_and_attrs: u32,
    h_template_file: HANDLE,
) -> HANDLE {
    log::trace!("CreateFileA called");

    let iat_store = get_real_createfilea();
    if let Err(e) = iat_store {
        log::error!("Cannot run CreateFileA. {e}");
        return INVALID_HANDLE_VALUE;
    }
    let iat_store = iat_store.unwrap();

    let real_fn: CreateFileAFn = iat_store.get_fn();

    if lp_file_name.is_null() {
        // No file name specified. Why would windows do this..?
        return real_fn(
            lp_file_name,
            dw_desired_access,
            dw_share_mode,
            lp_security_attrs,
            dw_creation_disposition,
            dw_flags_and_attrs,
            h_template_file,
        );
    }

    let orig_filename = CStr::from_ptr(lp_file_name)
        .to_string_lossy()
        .into_owned()
        .to_ascii_lowercase();
    log::trace!("CreateFileA called for file {orig_filename}");
    if !orig_filename.contains("dialog.tlk") {
        // Not our file, skip
        return real_fn(
            lp_file_name,
            dw_desired_access,
            dw_share_mode,
            lp_security_attrs,
            dw_creation_disposition,
            dw_flags_and_attrs,
            h_template_file,
        );
    }

    log::error!("dialog.tlk detected");
    // Override the path to one we want?
    // let new_path = r"C:\ModData\dialog_mod.tlk";
    // let new_cstr = CString::new(new_path).unwrap();

    return real_fn(
        lp_file_name,
        dw_desired_access,
        dw_share_mode,
        lp_security_attrs,
        dw_creation_disposition,
        dw_flags_and_attrs,
        h_template_file,
    );
}

/// Installs the above hook to catch the opening of dialog.tlk.
///
/// So far, only installs it in the main .exe memory space, so
/// any opens coming from a DLL will not be caught.
///
pub fn install_createfilea_hook() -> Result<(), Box<dyn Error>> {
    let store = install_plt_hook::<CreateFileAFn>(
        "swkotor.exe",
        "CreateFileA",
        &(my_createfilea as CreateFileAFn),
    )?;

    set_real_createfilea(store)?;

    Ok(())
}
