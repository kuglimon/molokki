/// Module that contains some hopefully obsoleting functions that
/// are usable for debugging.
///

use crate::{graphics::{initialize_pending_setup_rendering, RendingStore}, util};

use super::{imgui::ImguiRendable, textdraw::TextdrawRendable};

#[allow(dead_code)]
pub fn replace_mouse_button_text() {
    unsafe {
        let matches = util::needle_finder::find_all_matches_in_memory(b"Reverse Mouse Buttons\0");
        for needle in &matches {
            let new_bytes = b"Reverse Mouse Tittons";
            log::trace!("Replacing at address {:?}", needle);
            util::memory_patcher::patch_memory(needle.clone(), new_bytes);
        }
        log::trace!("Replaced {} instances", matches.len());
        // Do a check of trying to re-find our set string
        let matches = util::needle_finder::find_all_matches_in_memory(b"Reverse Mouse Tittons\0");
        for needle in &matches {
            log::trace!("Found at address {:?}", needle);
        }
        log::trace!("Found {} instances", matches.len());
    }
}

#[allow(dead_code)]
pub fn replace_hz_text() {
    if let Some(needle) = util::needle_finder::find_string_in_memory(b"%d Hz\0") {
        log::trace!("Needle found at address: {:?}", needle);
        let new_bytes = b"%d Iz";
        unsafe {
            util::memory_patcher::patch_memory(needle, new_bytes);
        }
    } else {
        log::trace!("Needle not found");
    }
}

/// Tries to replace the resolution string in memory. Did not
/// seem to work.
#[allow(dead_code)]
pub fn replace_resolution() {
    unsafe {
        let matches = util::needle_finder::find_all_matches_in_memory(b"1280x1024\0");
        for needle in &matches {
            let new_bytes = b"3440x1440";
            log::trace!("Replacing at address {:?}", needle);
            util::memory_patcher::patch_memory(needle.clone(), new_bytes);
        }
        log::trace!("Replaced {} instances", matches.len());
    }
}

/// Wrap the current graphics renderer here
#[allow(dead_code)]
pub fn draw_boxes_on_screen() -> RendingStore {
    let rendable = Box::new(TextdrawRendable::new());

    initialize_pending_setup_rendering(rendable)
}
