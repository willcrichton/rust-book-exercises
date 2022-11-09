//! P2: Terminal user interface
//!
//! This problem explores the differences between designing systems with
//! classes and traits. The adjacent file `tui.cpp` provides a C++ implementation
//! of a terminal user inteface (TUI), i.e. a simple set of graphical elements that
//! can be drawn into the terminal. The C++ code uses classes, inheritance, and
//! virtual methods.
//!
//! To see the C++ code in action, you can build and execute the code by running:
//!
//! ```bash
//! ./run.cpp-sh
//! ```
//!
//! Your task is to redesign the C++ TUI API into Rust. Your API should similarly
//! contain data structures that represent Text, Heading, and Container. You should
//! replicate the behavior of `main` in `tui.cpp` into the `container_test` function.
//!
//! Note: Cargo's test harness silences printing by default. You can prevent that
//! behavior by running:
//!
//! ```bash
//! cargo test container -- --nocapture
//! ```


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn container_test() {
        // Your unit test goes here!
    }
}
