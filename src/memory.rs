use std::ffi::c_void;
use std::mem::MaybeUninit;
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::{
            Debug::{ReadProcessMemory, WriteProcessMemory},
            ToolHelp::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First,
                Process32Next, MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE,
                TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
            },
        },
        Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS},
    },
};

// https://doc.rust-lang.org/reference/items/functions.html#generic-functions
// https://doc.rust-lang.org/reference/items/generics.html

// https://github.com/rmccrystal/memlib-rs

// TODO String are a pain, find better ways to handle them
// read_mem_str, write_mem_str

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> HANDLE {
    let handle: HANDLE = unsafe { OpenProcess(access, 0, pid) };

    if handle == INVALID_HANDLE_VALUE {
        println!(
            "[!] Failed to open process handle: {:?}",
            std::io::Error::last_os_error()
        );
    }

    handle
}

/// Closes a given process handle.
pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle) != 0 }
}

/// Returns the porcess ID based on a name.
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
pub fn get_module_base(mod_name: String, pid: u32) -> usize {
    let mut base_addr: usize = 0;

    let h_snapshot: HANDLE =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };

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
                        base_addr = module_entry.modBaseAddr as usize;
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

/// Read data from target memory of given type and size.
/// Size is calculated automatically by passing `None` or `Some(usize)` when needed.
///
/// **NOTE:** Some data types, mostly `String`, has to be read/written as `u8` bytes.
/// Seems to be the stable and *correct* way for any read/write operation.
pub fn read_mem<T: Default>(handle: HANDLE, address: usize) -> T {
    let mut buffer: T = Default::default();

    let status = unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            &mut buffer as *mut T as *mut c_void,
            std::mem::size_of::<T>(),
            std::ptr::null_mut(),
        )
    };

    if status == 0 {
        println!(
            "[!] Failed to read memory: {:?}",
            std::io::Error::last_os_error()
        );
    };

    buffer
}

/// A variation of [`read_mem`] for reading `String`.
/// Iterates over the memory starting from `address` until a null terminator is found.
pub fn read_mem_str(handle: HANDLE, address: usize) -> String {
    let mut buffer: Vec<u8> = Vec::new();
    let mut itr_addr = address.clone();

    loop {
        let byte = read_mem::<u8>(handle, itr_addr);
        if byte == 0u8 {
            break;
        } else {
            buffer.push(byte);
            itr_addr += 0x1;
        }
    }

    String::from_utf8(buffer).unwrap()
}

/// Writes data to target memory of given type.
///
/// **NOTE:** Some data types, such as `String`, has to be read/written as `u8` bytes.
/// Seems to be the stable and *correct* way for any read/write operation.
pub fn write_mem<T: Default>(handle: HANDLE, address: usize, data_ptr: *const T) -> bool {
    let status = unsafe {
        WriteProcessMemory(
            handle,
            address as *const c_void,
            data_ptr as *const c_void,
            std::mem::size_of_val(&data_ptr),
            std::ptr::null_mut(),
        )
    };

    if status == 0 {
        println!(
            "[!] Failed to write memory: {:?}",
            std::io::Error::last_os_error()
        );
    }

    status != 0
}

/// Variation of [`write_mem`] but for `String`.
///
/// **NOTE:** Due to possible side-effects, we should stay within the boundary
/// of the original string length. Need to read more on that.
pub fn write_mem_str(handle: HANDLE, address: usize, data: String) -> bool {
    // get the current String
    let buffer = read_mem_str(handle, address);

    // nullify the existing data
    let mut itr_addr = address.clone();
    for _ in 0..buffer.len() {
        let status = write_mem::<u8>(handle, itr_addr, &0u8);
        if status == false {
            println!("[!] Failed to clear out memory");
            break;
        }
        itr_addr += 0x1;
    }

    // slice the data up to the length of the original buffer
    let ow_buffer: String = {
        if data.len() > buffer.len() {
            println!("[*] String input was sliced to boundary");
            String::from_utf8(data.clone().as_bytes()[..buffer.len()].to_vec()).unwrap()
        } else {
            data.clone()
        }
    };

    // overwrite the string
    let mut itr_addr = address.clone();
    for ch in ow_buffer.chars() {
        let byte = ch as u8;
        let status = write_mem::<u8>(handle, itr_addr, &byte);
        if status == false {
            println!("[!] Failed to overwrite String");
            break;
        }
        itr_addr += 0x1;
    }

    // check to see if we have overwritten successfully
    let buffer = read_mem_str(handle, address);
    buffer == ow_buffer
}
