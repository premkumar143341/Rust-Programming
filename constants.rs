/*Write a program to create different types of constants print it in the output*/
fn main() {
    // Define constants using `const` keyword with explicit types

    const MAX_POINTS: u32 = 100_000;           // unsigned 32-bit integer
    const PI: f64 = 3.14159;                    // 64-bit floating point
    const IS_ACTIVE: bool = true;               // boolean
    const GREETING: &str = "Hello, Rust!";     // string slice

    println!("MAX_POINTS (u32): {}", MAX_POINTS);
    println!("PI (f64): {}", PI);
    println!("IS_ACTIVE (bool): {}", IS_ACTIVE);
    println!("GREETING (&str): {}", GREETING);
}
