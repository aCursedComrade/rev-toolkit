use rev_toolkit::dll_main;

mod structs;

unsafe fn init() {
    loop {}
}

dll_main!(init);
