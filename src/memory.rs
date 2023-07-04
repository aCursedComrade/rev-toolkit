use std::mem::MaybeUninit;

// love this import style :)
use winapi::{
    shared::{
        minwindef::{DWORD, LPCVOID, LPVOID},
        ntdef::HANDLE,
    },
    um::{
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        processthreadsapi::OpenProcess,
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
            MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
            TH32CS_SNAPPROCESS,
        },
    },
};

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: DWORD) -> HANDLE {
    let handle: HANDLE = unsafe { OpenProcess(access, 0, pid) };

    if handle.is_null() {
        println!(
            "[!] Failed to open process handle: {:?}",
            std::io::Error::last_os_error()
        );
    }

    handle
}

/// Closes a given process handle.
pub fn close_handle(proc_handle: HANDLE) -> bool {
    unsafe { CloseHandle(proc_handle) != 0 }
}

/// Returns the porcess ID based on a name.
///
/// https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes
pub fn get_pid(proc_name: String) -> u32 {
    let mut pid: u32 = 0;

    let h_snapshot: HANDLE = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    if h_snapshot != INVALID_HANDLE_VALUE {
        let mut init_proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

        unsafe {
            let proc_entry = init_proc_entry.assume_init_mut();
            proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
            proc_entry.szExeFile = [0; 260];

            if Process32First(h_snapshot, proc_entry) != 0 {
                loop {
                    let mut path = String::new();
                    for c in proc_entry.szExeFile {
                        path.push(char::from_u32(c as u32).unwrap())
                    }
                    // println!("{}", path);
                    if path.contains(&proc_name) {
                        pid = proc_entry.th32ProcessID;
                        break;
                    }
                    if Process32Next(h_snapshot, proc_entry) == 0 {
                        break;
                    }
                }
            }
        }
    }

    close_handle(h_snapshot);
    pid
}

/// Returns the base address of the given module name. Usually, module name is same as process name.
pub fn get_module_base(mod_name: String, proc_id: u32) -> u64 {
    let mut base_addr: u64 = 0;

    let h_snapshot: HANDLE =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, proc_id) };
    if h_snapshot != INVALID_HANDLE_VALUE {
        let mut init_module_entry = MaybeUninit::<MODULEENTRY32>::uninit();

        unsafe {
            let module_entry = init_module_entry.assume_init_mut();
            module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
            module_entry.szModule = [0; 256];

            if Module32First(h_snapshot, module_entry) != 0 {
                loop {
                    let mut path = String::new();
                    for c in module_entry.szModule {
                        path.push(char::from_u32(c as u32).unwrap())
                    }
                    // println!("{}", path);
                    if path.contains(&mod_name) {
                        base_addr = module_entry.modBaseAddr as u64;
                        break;
                    }
                    if Module32Next(h_snapshot, module_entry) == 0 {
                        break;
                    }
                }
            }
        }
    }

    close_handle(h_snapshot);
    base_addr
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
pub fn read_mem<T: Default>(
    proc_handle: HANDLE,
    address: u64,
    nsize: Option<usize>,
) -> Result<T, String> {
    let mut buffer: T = Default::default();
    let read_size = {
        match nsize {
            Some(i) => i,
            None => std::mem::size_of::<T>(),
        }
    };

    let status = unsafe {
        ReadProcessMemory(
            proc_handle,
            address as *const u64 as LPCVOID,
            &mut buffer as *mut T as LPVOID,
            read_size,
            std::ptr::null_mut() as *mut usize,
        )
    };

    if status == 0 {
        println!(
            "[!] Failed to read memory: {:?}",
            std::io::Error::last_os_error()
        );
        Err(String::from("Failed to read memory"))
    } else {
        Ok(buffer)
    }
}

/// A variation of [`read_mem`] for reading `Strings`.
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

    (String::from_utf8(buffer).unwrap(), size)
}

/// Writes data to target memory of given type.
///
/// **NOTE:** Some data types, such as `String`, has to be read/written as `u8` bytes.
/// Seems to be the stable and *correct* way for any read/write operation.
///
/// TODO:
/// - read more on the topic
/// - write as bytes instead?
pub fn write_mem<T: Default>(proc_handle: HANDLE, address: u64, data: &T) -> bool {
    // std::slice::from_raw_parts(data as *const T as _, std::mem::size_of::<T>())
    let status = unsafe {
        WriteProcessMemory(
            proc_handle,
            address as *mut u64 as LPVOID,
            data as *const T as LPCVOID,
            std::mem::size_of::<T>(),
            std::ptr::null_mut() as *mut usize,
        )
    };

    if status == 0 {
        println!(
            "[!] Failed to write memory: {:?}",
            std::io::Error::last_os_error()
        );
    } else {
        println!("[+] Overwritten successfully")
    }

    status != 0
}
