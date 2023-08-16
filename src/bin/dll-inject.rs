use clap::Parser;
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

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    /// Target program (ex: explorer.exe)
    target: String,

    #[arg(short, long)]
    /// Absolute path to the DLL
    path: String,
}

// TODO cross-bitness injection, dealing with 32 bit processes from a 64 bit context
// current state of the injector requires it be in the same mode (32 or 64 bit) as the target

// https://stackoverflow.com/questions/8776437/c-injecting-32-bit-targets-from-64-bit-process
// dll-syringe - https://github.com/OpenByteDev/dll-syringe/

fn main() {
    unsafe {
        let args = Cli::parse();
        let kernel32handle = GetModuleHandleA(s!("kernel32.dll")).unwrap();
        let loadlibrary = GetProcAddress(kernel32handle, s!("LoadLibraryA"));
        let mut exitcode = 0u32;

        println!("[*] Target: {}", &args.target);
        println!("[*] DLL path: {}", &args.path);

        let target = Process::new(
            args.target.clone(),
            PROCESS_CREATE_THREAD
                | PROCESS_QUERY_INFORMATION
                | PROCESS_VM_OPERATION
                | PROCESS_VM_READ
                | PROCESS_VM_WRITE,
        );

        // allocate memory for path string
        let alloc_address = VirtualAllocEx(
            target.handle,
            Some(std::ptr::null()),
            std::mem::size_of_val(args.path.as_str()),
            MEM_RESERVE | MEM_COMMIT,
            PAGE_READWRITE,
        );

        // write the path string
        let write_status = WriteProcessMemory(
            target.handle,
            alloc_address,
            args.path.as_ptr() as *const c_void,
            std::mem::size_of_val(args.path.as_str()),
            Some(std::ptr::null_mut()),
        );

        if write_status.is_ok() {
            // spawn thread with LoadLibraryA to load the DLL
            let thread_status = CreateRemoteThread(
                target.handle,
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
                    println!("[+] Thread spawned");
                    WaitForSingleObject(thread, u32::MAX);
                    let _ = GetExitCodeThread(thread, &mut exitcode);
                    println!("[+] Thread exited with code: {}", exitcode);
                    close_handle(thread);
                }
                Err(_) => {
                    println!("[!] Falied to spawn thread");
                }
            }

            let _ = VirtualFreeEx(target.handle, alloc_address, 0, MEM_RELEASE);
        } else {
            println!("[!] Failed to allocate/write memory on target");
        }

        println!("[*] End");
    }
}
