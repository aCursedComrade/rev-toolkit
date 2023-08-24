use rev_toolkit::{memory::close_handle, process::Process};
use std::ffi::c_void;
use windows::core::s;
use windows::Win32::System::{
    Diagnostics::Debug::WriteProcessMemory,
    LibraryLoader::{GetModuleHandleA, GetProcAddress},
    Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE},
    Threading::{
        CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, PROCESS_CREATE_THREAD,
        PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
    },
};

pub unsafe fn inject_dll(target_name: String, dll_path: String) -> Result<(), crate::InjectError> {
    let kernel32handle = GetModuleHandleA(s!("kernel32.dll")).unwrap();
    let loadlibrary = GetProcAddress(kernel32handle, s!("LoadLibraryA"));
    let mut exitcode = 0u32;

    let process = Process::new(
        target_name,
        PROCESS_CREATE_THREAD
            | PROCESS_QUERY_INFORMATION
            | PROCESS_VM_OPERATION
            | PROCESS_VM_READ
            | PROCESS_VM_WRITE,
    );

    if !process.is_valid() {
        return Err(crate::InjectError::InvalidProcess);
    }

    // allocate memory for path string
    let alloc_address = VirtualAllocEx(
        process.handle,
        Some(std::ptr::null()),
        std::mem::size_of_val(dll_path.as_str()),
        MEM_RESERVE | MEM_COMMIT,
        PAGE_READWRITE,
    );

    if alloc_address.is_null() {
        return Err(crate::InjectError::MemoryAllocError);
    }

    // write the path string
    let write_status = WriteProcessMemory(
        process.handle,
        alloc_address,
        dll_path.as_ptr() as *const c_void,
        std::mem::size_of_val(dll_path.as_str()),
        Some(std::ptr::null_mut()),
    );

    if write_status.is_err() {
        let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
        return Err(crate::InjectError::MemoryWriteError);
    }

    // spawn thread with LoadLibraryA to load the DLL
    let thread_status = CreateRemoteThread(
        process.handle,
        Some(std::ptr::null()),
        0,
        std::mem::transmute(loadlibrary),
        Some(alloc_address as *const c_void),
        0,
        Some(std::ptr::null_mut()),
    );

    // sync and cleanup
    match thread_status {
        Ok(thread) => {
            WaitForSingleObject(thread, u32::MAX);
            let _ = GetExitCodeThread(thread, &mut exitcode);
            close_handle(thread);

            let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
            return Ok(());
        }
        Err(_) => {
            let _ = VirtualFreeEx(process.handle, alloc_address, 0, MEM_RELEASE);
            return Err(crate::InjectError::SpawnThreadError);
        }
    }
}
