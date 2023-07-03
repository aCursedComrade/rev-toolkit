use winapi::{
    um::processthreadsapi::OpenProcess,
    um::memoryapi::{ReadProcessMemory,WriteProcessMemory},
    um::handleapi::CloseHandle,
    shared::minwindef::{LPVOID,LPCVOID,DWORD},
    shared::ntdef::HANDLE,
};

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: DWORD) -> HANDLE {
    let handle: HANDLE = unsafe { OpenProcess(access, 0, pid) };

    if handle.is_null() {
        println!("[!] Failed to open process handle: {:?}", std::io::Error::last_os_error());
    }

    handle
}

/// Closes a given process handle.
pub fn close_handle(proc_handle: HANDLE) -> bool {
    unsafe { CloseHandle(proc_handle) != 0 }
}

// generic parameters
// https://doc.rust-lang.org/reference/items/functions.html#generic-functions
// https://doc.rust-lang.org/reference/items/generics.html

// https://github.com/rmccrystal/memlib-rs

/// Read data from target memory of given type and size.
/// Size is calculated automatically by passing `None` or `Some(usize)` when needed.
/// 
/// **NOTE:** Some data types, such as `String`, has to be read/written as `u8` bytes.
/// Seems to be the stable and *correct* way for any read/write operation.
/// 
/// TODO:
/// - read more on the topic
/// - follow pointers?
pub fn read_mem<T: Default>(proc_handle: HANDLE, address: u64, nsize: Option<usize>) -> Result<T, String> {
    let mut buffer: T = Default::default();
    let read_size = {
        match nsize {
            Some(i) => i,
            None => std::mem::size_of::<T>()
        }
    };

    let status = unsafe { ReadProcessMemory(
        proc_handle, 
        address as *const u64 as LPCVOID,
        &mut buffer as *mut T as LPVOID,
        read_size,
        std::ptr::null_mut() as *mut usize
    ) };

    if status == 0 {
        println!("[!] Failed to read memory: {:?}", std::io::Error::last_os_error());
        Err(String::from("Failed to read memory"))
    } else {
        Ok(buffer)
    }
}

/// A variation of `read_mem` for reading `Strings`.
/// Iterates over the memory starting from `address` until a null terminator is found.
/// 
/// Returns the `String` and its size
pub fn read_mem_str(proc_handle: HANDLE, address: u64) -> (String, usize) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut itr_addr = address.clone();
    let mut size: usize = 0;
    
    loop {
        let byte = read_mem::<u8>(proc_handle, itr_addr, Some(1)).expect("[!] Failed to read byte");
        if byte == 0u8 {
            break;
        } else {
            buffer.push(byte);
            itr_addr += 0x1;
            size += 1;
        }
    }

    println!("buffer: {:?}", buffer);
    (String::from_utf8(buffer).unwrap(), size)
}

/// Writes data to target memory of given type.
/// 
/// **NOTE:** Some data types, such as `String`, has to be read/written as `u8` bytes.
/// Seems to be the stable and *correct* way for any read/write operation.
/// 
/// TODO:
/// - read more on the topic
pub fn write_mem<T: Default>(proc_handle: HANDLE, address: u64, data: &T) -> bool {
    // std::slice::from_raw_parts(data as *const T as _, std::mem::size_of::<T>())
    let status = unsafe { WriteProcessMemory(
        proc_handle,
        address as *mut u64 as LPVOID,
        data as *const T as LPCVOID,
        std::mem::size_of_val(&data),
        std::ptr::null_mut() as *mut usize
    ) };

    if status == 0 {
        println!("[!] Failed to write memory: {:?}", std::io::Error::last_os_error());
    } else {
        println!("[+] Overwritten successfully")
    }

    status != 0
}