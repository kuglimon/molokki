use std::ffi::c_void;
use std::{ffi::CStr, fmt, mem::ManuallyDrop, path::PathBuf};

use windows::Win32::Foundation::HMODULE;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::MAX_PATH,
        System::{
            LibraryLoader::{GetProcAddress, LoadLibraryA},
            SystemInformation::GetSystemDirectoryA,
        },
    },
};

/// TODO(tatu): Fix this damn loader fiasco. Right now the loader is procedural as fuck. You need
/// to first call LoadLibraryA, then check the return base addresses, then call GetProcAddress with
/// the base address and again check if it works. If you never called LoadLibraryA you'll end up
/// fucked. There's a heavy use of casting and transmuting, which allow you to shoot yourself in
/// the dick, robocop style.
///
/// I think a better design would be to have a DllLoader, which you initialize with the name of the
/// dll. That loader is typed to either return a valid loader or errored one. Through the valid
/// loader you can then load references to functions. That would create an api where it's always
/// clear from the code that you have an instance that is valid and can load functions.
///
/// Pseudo-code example:
///
/// ```
/// // Just don't make DllLoader fields public so users can't construct a broken instance
/// match DllLoader.for_dll(DllLibrary.Dinput8) {
///     Some(loader) => loader.load_function("DirectInput8Create")
///     None => panic!("Could not load library")
/// }
/// ```

type FnSystemPointer = unsafe extern "system" fn() -> isize;
type DllBaseAddress = usize;

// Lists some known libraries kotor loads during boot/gameplay
#[derive(Debug)]
pub enum DllLibrary {
    Dinput8,
}

impl fmt::Display for DllLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str = match self {
            DllLibrary::Dinput8 => "dinput8.dll",
        };
        write!(f, "{}", as_str)
    }
}

// TODO(tatu): Fix this API, why are we making shit like this in rust? Just make a loader that
// handles all this state.
//
// Loads funtion by name from given base address. DLL needs to have been loaded before using this.
//
// Panics if function cannot be loaded, gain the fuck we gonna do if we cannot load it? Crash at
// the callsite, not two days after. Since we are actually overriding default libraries we should
// not try to hide it and act as some valid library in between. Crash hard and fast because it's
// 100% our fault or a broken system in case of crashes. Prolonging error will just make it
// frustrating to debug.
//
// Note that this function is super unsafe and dangerous to use, I don't know if there's a better
// way to do this. But transmute in general is bad.
pub fn get_proc_address<T>(address: DllBaseAddress, function_name: &str) -> T {
    let function_name = PCSTR::from_raw(format!("{function_name}\0").as_mut_ptr());
    // FIXME(tatu): The fuck is this casting bullshit? We back to using c now? Wrap as new type.
    let maybe_address = unsafe { GetProcAddress(HMODULE(address as *mut c_void), function_name) };

    match maybe_address {
        Some(valid_address) => unsafe {
            std::mem::transmute_copy::<ManuallyDrop<FnSystemPointer>, T>(&ManuallyDrop::new(
                valid_address,
            ))
        },
        None => panic!("Cannot load function {function_name:?}"),
    }
}

// Wrapper around GetSystemDirectoryA without having to fiddle with Cstr.
//
// Panics if system directory cannot be found. Not much point in doing anything if we cannot even
// load the dlls we're overriding. We'd crash anyways.
fn get_system_directory() -> String {
    let mut path_buffer = [0u8; MAX_PATH as usize];

    unsafe {
        let len = GetSystemDirectoryA(Some(path_buffer.as_mut()));

        // Should not happen, broken system, just crash
        if len == 0 {
            panic!("Could not load system directory, your system is fucked!");
        }

        let system_dir = CStr::from_ptr(path_buffer.as_ptr() as *const i8);

        system_dir.to_string_lossy().into_owned()
    }
}

// Ties to load library in path.
//
// Panics if library is not found, there's nothing we can do if a DLL is not found.
fn load_library_a(dll_path: PathBuf) -> DllBaseAddress {
    let dll_path_str = dll_path
        .to_str()
        .expect("Could not convert path to string, broken dll path given");
    let dll_path = PCSTR::from_raw(format!("{dll_path_str}\0").as_mut_ptr());

    let base_address = match unsafe { LoadLibraryA(dll_path) } {
        Ok(value) => value,
        Err(_) => panic!("Could not load {dll_path_str:?}"),
    };

    base_address.0 as DllBaseAddress
}

// In Windows you'd load libraries using something like `LoadLibraryA`, which handles searching for
// the library as well. But in our case we don't want this as we're already relying on this logic
// to override the existing library. Instead we want to bypass the mechanism and directly load from
// system directories.
pub fn load_system_library_a(library: DllLibrary) -> DllBaseAddress {
    let system_path = PathBuf::from(get_system_directory());
    let dll_path = system_path.join(library.to_string());
    load_library_a(dll_path)
}
