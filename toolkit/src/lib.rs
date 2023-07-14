pub mod memory;
pub mod process;
pub mod utils;

// TODO: re-export certain winapi variables so examples dont have to import the windows crate

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(1, 1);
    }
}
