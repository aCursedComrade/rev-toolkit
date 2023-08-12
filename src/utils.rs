use std::io::Write;

/// Parse memory address intereactively.
/// Just for testing through CLI.
pub fn get_addr(expect_type: &str) -> usize {
    let mut t_addr = String::new();
    print!("Target {} address: ", expect_type);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut t_addr).unwrap();
    usize::from_str_radix(&t_addr.trim().trim_start_matches("0x"), 16).unwrap()
}
