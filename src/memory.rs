use std::ffi::c_void;
use std::mem::MaybeUninit;
use windows::Win32::{
    Foundation::{CloseHandle, BOOL, HANDLE},
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

/// Opens a handle to the target with given access permission.
pub fn open_handle(pid: u32, access: PROCESS_ACCESS_RIGHTS) -> HANDLE {
    let status = unsafe { OpenProcess(access, BOOL(0), pid) };

    match status {
        Ok(handle) => handle,
        Err(_) => HANDLE(0),
    }
}

/// Closes a given process handle.
pub fn close_handle(handle: HANDLE) -> bool {
    unsafe { CloseHandle(handle).is_ok() }
}

/// Returns the porcess ID based on a name.
pub fn get_pid(proc_name: String) -> u32 {
    let mut pid: u32 = 0;

    let h_snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    match h_snapshot {
        Err(_) => pid,
        Ok(handle) => {
            let mut init_proc_entry = MaybeUninit::<PROCESSENTRY32>::uninit();

            unsafe {
                let proc_entry = init_proc_entry.assume_init_mut();
                proc_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
                proc_entry.szExeFile = [0; 260];

                if !Process32First(handle, proc_entry).is_err() {
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
                        if Process32Next(handle, proc_entry).is_err() {
                            break;
                        }
                    }
                }
            }

            close_handle(handle);
            pid
        }
    }
}

/// Returns the base address of the given module name. Usually, module name is same as process name.
pub fn get_module_base(mod_name: String, pid: u32) -> usize {
    let mut base_addr: usize = 0;

    let h_snapshot =
        unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid) };

    match h_snapshot {
        Err(_) => base_addr,
        Ok(handle) => {
            let mut init_module_entry = MaybeUninit::<MODULEENTRY32>::uninit();

            unsafe {
                let module_entry = init_module_entry.assume_init_mut();
                module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;
                module_entry.szModule = [0; 256];

                if !Module32First(handle, module_entry).is_err() {
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
                        if Module32Next(handle, module_entry).is_err() {
                            break;
                        }
                    }
                }
            }

            close_handle(handle);
            base_addr
        }
    }
}

/// Read object from target memory of given type.
pub fn read_mem<T: Default>(handle: HANDLE, address: usize) -> Option<T> {
    let mut buffer: T = Default::default();

    let status = unsafe {
        ReadProcessMemory(
            handle,
            address as *const c_void,
            &mut buffer as *mut T as *mut c_void,
            std::mem::size_of::<T>(),
            Some(std::ptr::null_mut()),
        )
    };

    match status {
        Ok(()) => Some(buffer),
        Err(_) => None,
    }
}

/// Read raw bytes upto a given size and returns a `Vec<u8>`.
/// The bytes will be in **little endian** order.
///
/// Useful when reading objects of indefinite size like `String`.
pub fn read_mem_raw(
    handle: HANDLE,
    address: usize,
    size: usize,
) -> Option<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();

    for idx in 0..size {
        let read_byte = read_mem::<u8>(handle, address + idx);
        match read_byte {
            None => return None,
            Some(byte) => buffer.push(byte),
        }
    }

    Some(buffer)
}

/// Writes data to target memory of given type.
/// Works with objects of indefinite size like `String`.
pub fn write_mem<T: Default>(handle: HANDLE, address: usize, data_ptr: *const T) -> bool {
    let status = unsafe {
        WriteProcessMemory(
            handle,
            address as *const c_void,
            data_ptr as *const c_void,
            std::mem::size_of_val(&data_ptr),
            Some(std::ptr::null_mut()),
        )
    };

    status.is_ok()
}
