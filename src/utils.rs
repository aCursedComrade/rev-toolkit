use std::io::Write;

/// Parse memory address intereactively.
/// Just for testing through CLI.
pub fn get_addr(expect_type: &str) -> u64 {
    let mut t_addr = String::new();
    print!("Target {} address: ", expect_type);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut t_addr).unwrap();
    parse_addr(&t_addr)
}

/// Parse memory address from string slice.
pub fn parse_addr(addr_str: &str) -> u64 {
    u64::from_str_radix(&addr_str.trim().trim_start_matches("0x"), 16).unwrap()
}
