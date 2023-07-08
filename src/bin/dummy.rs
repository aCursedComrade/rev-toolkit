use std::io;
use windows::Win32::System::Threading::GetCurrentProcessId;

// A dummy program to test against.

fn main() {
    unsafe {
        let var_int: i32 = 123456;
        let var_string = String::from("DefaultString");
        let ptr2int = &var_int;
        let ptr2ptr = &ptr2int;
        let ptr2ptr2 = &ptr2ptr;

        loop {
            println!("Process ID: {}", GetCurrentProcessId());
            println!();
            println!("var_int ({:p}) = {}", &var_int, var_int); // {:p} prints address of the reference
            println!("var_string ({:p}) = {}", var_string.as_ptr(), var_string);
            println!();
            println!("ptr2int ({:p}) = {:p}", &ptr2int, ptr2int); // {:p} also prints the address value held by a pointer
            println!("ptr2ptr ({:p}) = {:p}", &ptr2ptr, ptr2ptr);
            println!("ptr2ptr2 ({:p}) = {:p}", &ptr2ptr2, ptr2ptr2);
            println!();
            println!("Press Enter to continue...");
            println!("----------------------------");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
}
