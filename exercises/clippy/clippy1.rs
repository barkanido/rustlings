// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// Execute `rustlings hint clippy1` for hints :)

use core::f64::EPSILON;

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if EPSILON < (y - x).abs() {
        println!("Success!");
    }
}
