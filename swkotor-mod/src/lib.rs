use windows::{Win32::Foundation::*, Win32::System::SystemServices::*};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => println!("dll attach"),
        DLL_PROCESS_DETACH => println!("dll deattach"),
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
