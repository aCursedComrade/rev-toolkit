//! Contain functions used to find patterns and compare data
use windows_sys::Win32::Foundation::HANDLE;

/// Compare a byte pattern with given mask.
pub fn eval_sig() -> bool {
    todo!();
}

/// Find a pattern within memory that
/// matches a given mask.
pub fn find_pattern() -> Option<usize> {
    todo!()
}

/// Find a pattern within memory that
/// matches a given set of bytes.
pub fn find_bytes(handle: HANDLE, bytes: &[u8], address: usize, size: u32) -> Option<usize> {
    for i in 0..size as usize {
        let cur_addr = address + i;
        if let Some(chunk) = crate::memory::read_mem::<[u8; 32]>(handle, cur_addr) {
            let mut buffer: &[u8] = &chunk;

            while !buffer.is_empty() {
                if buffer.starts_with(bytes) {
                    return Some(cur_addr);
                }
                buffer = &buffer[1..];
            }
        } else {
            continue;
        };
    }

    None
}
