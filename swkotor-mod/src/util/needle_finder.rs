/// Utility to find a needle in a haystack, or a string in memory.
///
use std::{ffi::c_void, slice};
use windows::Win32::System::Memory::{
    VirtualProtect, VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_GUARD, PAGE_NOACCESS,
    PAGE_NOCACHE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE, PAGE_WRITECOMBINE,
};

#[allow(dead_code)]
unsafe fn remove_guard_page(address: *const c_void, size: usize) -> bool {
    let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS::default();
    let res = VirtualProtect(address, size, PAGE_READWRITE, &mut old_protect);
    res.is_ok()
}

// A small utility for searching for `needle` in `haystack`.
fn naive_mem_search(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    // Slide over haystack windows of length equal to needle
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Checks if the page is accessible by default
fn skip_memory(mbi: &MEMORY_BASIC_INFORMATION) -> bool {
    mbi.State != MEM_COMMIT
        || (mbi.Protect == PAGE_NOACCESS
            || (mbi.Protect & PAGE_GUARD) == PAGE_GUARD
            || (mbi.Protect & PAGE_NOCACHE) == PAGE_NOCACHE
            || (mbi.Protect & PAGE_WRITECOMBINE) == PAGE_WRITECOMBINE)
}
/// Search the current process memory for the given `needle`
/// Returns the first address where `needle` is found, or None if not found.
///
/// CAUTION: This is purely a naive PoC. It may crash if pages are partially accessible.
///
/// - `needle`: the bytes to look for (e.g. b"dialog.tlk\0" or wide-encoded bytes).
pub fn find_string_in_memory(needle: &[u8]) -> Option<*mut u8> {
    let mut address = 0usize; // Start from address 0 and walk upward

    loop {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        // VirtualQuery will fill `mbi` with info about the region containing `address`.
        let result = unsafe {
            VirtualQuery(
                Some(address as *const _),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };

        if result == 0 {
            // No more regions to query
            break;
        }

        // We only scan committed, readable pages. (Skipping free/reserved/noaccess.)
        // A more complete approach also checks mbi.Protect, e.g., skipping PAGE_GUARD, etc.
        if !skip_memory(&mbi) {
            let base = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;

            // Construct a slice to read that memory. This is UNSAFE and can crash
            // if the region is partially accessible or there's a guard page.
            let slice = unsafe { slice::from_raw_parts(base as *const u8, region_size) };

            // Search for the needle.
            if let Some(pos) = naive_mem_search(slice, needle) {
                let found_addr = base + pos;
                return Some(found_addr as *mut u8);
            }
        }

        // Move to the next region: base + region_size
        address = mbi.BaseAddress as usize + mbi.RegionSize;
    }
    None
}

/// Returns all starting indices in `haystack` where `needle` is found.
fn naive_mem_search_all(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    let mut results = Vec::new();

    if needle.is_empty() {
        // If needle is empty, every index is a match (ambiguous).
        return results;
    }

    let mut start = 0;

    while let Some(pos) = haystack[start..]
        .windows(needle.len())
        .position(|window| window == needle)
    {
        let absolute_pos = start + pos;
        results.push(absolute_pos);

        // Move `start` forward to continue searching *after* this match.
        // You could do `+ 1` or `+ needle.len()` depending on whether you want overlapping matches.
        start = absolute_pos + 1;
    }

    results
}

use std::mem;

pub unsafe fn find_all_matches_in_memory(needle: &[u8]) -> Vec<*mut u8> {
    let mut results = Vec::new();
    let mut address = 0usize;

    loop {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();

        let result = VirtualQuery(
            Some(address as *const _),
            &mut mbi,
            mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        );

        if result == 0 {
            // No more regions
            break;
        }

        // Check if this region is committed and readable
        if !skip_memory(&mbi) {
            let base = mbi.BaseAddress as usize;

            let region_size = mbi.RegionSize;
            // Construct a slice for the region
            let slice = slice::from_raw_parts(base as *const u8, region_size);
            // Search for all occurrences in this region
            let offsets = naive_mem_search_all(slice, needle);
            for offset in offsets {
                let found_addr = base + offset;
                results.push(found_addr as *mut u8);
            }
        } else {
            log::trace!(
                "Skipping entry at {:p} with protect {:?}",
                mbi.BaseAddress,
                mbi.Protect
            );
        }

        // Move to the next region
        address = mbi.BaseAddress as usize + mbi.RegionSize;
    }

    results
}
