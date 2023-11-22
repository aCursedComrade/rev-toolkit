use crate::errors::InjectError;
use rev_toolkit::{memory::close_handle, Process};
use std::ffi::c_void;
use windows_sys::s;
use windows_sys::Win32::System::{
    Diagnostics::Debug::WriteProcessMemory,
    LibraryLoader::{GetModuleHandleA, GetProcAddress},
    Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
    Threading::{
        CreateRemoteThread, GetExitCodeThread, Sleep, WaitForSingleObject, PROCESS_CREATE_THREAD,
        PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

fn resolve_dll(dll: &str) -> Option<std::path::PathBuf> {
    let path = std::path::Path::new(dll);
    match path.canonicalize() {
        Ok(ab_path) => {
            if ab_path.is_file() {
                Some(ab_path)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

pub unsafe fn inject_dll(target_name: &str, dll: &str) -> Result<(), InjectError> {
    // dll-syringe - https://github.com/OpenByteDev/dll-syringe/

    // TODO dealing with cross-bitness injection
    // come up with a logic that is going to:
    // - look at the current host architecture
    // - look at the arch of our injector and target
    // - pick the best way to get the module handle and function address:
    //      - use `GetModuleHandleA` and `GetProcAddress` if both are on the same arch
    //      - do some PE dissection magic? to deal with 32 bit target from a 64 bit context
    // - continue the injection as below
    let kernel32handle = GetModuleHandleA(s!("kernel32.dll"));
    let loadlibrary = GetProcAddress(kernel32handle, s!("LoadLibraryA"));

    // resolve the path to DLL
    let check_path = resolve_dll(dll);
    if check_path.is_none() {
        return Err(InjectError::InvalidDLLPath);
    }
    let resolved_dll = check_path.unwrap();

    // construct a Process object
    let target = Process::new(
        target_name,
        PROCESS_CREATE_THREAD
            | PROCESS_QUERY_INFORMATION
            | PROCESS_VM_OPERATION
            | PROCESS_VM_READ
            | PROCESS_VM_WRITE,
    );
    if !target.is_valid() {
        return Err(InjectError::InvalidProcess);
    }

    // allocate memory for path string
    let alloc_address = VirtualAllocEx(
        target.handle,
        std::ptr::null(),
        std::mem::size_of_val(resolved_dll.as_os_str()),
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );
    if alloc_address.is_null() {
        return Err(InjectError::MemoryAllocError);
    }

    // write the path string
    let write_status = WriteProcessMemory(
        target.handle,
        alloc_address,
        resolved_dll.as_os_str() as *const _ as *const c_void,
        std::mem::size_of_val(resolved_dll.as_os_str()),
        std::ptr::null_mut(),
    );
    if write_status == 0 {
        let _ = VirtualFreeEx(target.handle, alloc_address, 0, MEM_RELEASE);
        return Err(InjectError::MemoryWriteError);
    }

    // spawn thread with LoadLibraryA to load the DLL
    let thread = CreateRemoteThread(
        target.handle,
        std::ptr::null(),
        0,
        std::mem::transmute(loadlibrary),
        alloc_address as *const c_void,
        0,
        std::ptr::null_mut(),
    );
    match thread {
        -1 => {
            let _ = VirtualFreeEx(target.handle, alloc_address, 0, MEM_RELEASE);
            return Err(InjectError::SpawnThreadError);
        }
        _ => {
            let mut exitcode = 0u32;
            WaitForSingleObject(thread, u32::MAX);
            let _ = GetExitCodeThread(thread, &mut exitcode);
            close_handle(thread);
            Sleep(200);

            let status: Result<(), InjectError>;
            // we look at the module list again to see if the DLL exists
            if target.query_module(resolved_dll.file_name().unwrap().to_str().unwrap()) {
                status = Ok(());
            } else {
                status = Err(InjectError::InjectionFail);
            }

            let _ = VirtualFreeEx(target.handle, alloc_address, 0, MEM_RELEASE);
            return status;
        }
    }
}
