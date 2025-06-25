*/Write a program to implement the following
a. Implicit type declaration
b. Explicit type declaration*/
fn main() {
    // a. Implicit type declaration
    let x = 42;            // Rust infers x as i32
    let y = 3.14;          // Rust infers y as f64
    let name = "Alice";    // Rust infers name as &str (string slice)

    println!("Implicit types -> x: {}, y: {}, name: {}", x, y, name);

    // b. Explicit type declaration
    let a: i64 = 1000;     // explicitly declared as 64-bit signed integer
    let b: f32 = 2.718;    // explicitly declared as 32-bit float
    let greeting: &str = "Hello";  // explicitly declared as string slice

    println!("Explicit types -> a: {}, b: {}, greeting: {}", a, b, greeting);
}
