/*Write a program to create and access a tuple.*/
fn main() {
    // Create a tuple with different types
    let my_tuple = (42, 3.14, "Rust");

    // Access tuple elements using dot notation
    let int_value = my_tuple.0;
    let float_value = my_tuple.1;
    let string_value = my_tuple.2;

    println!("Integer value: {}", int_value);
    println!("Float value: {}", float_value);
    println!("String value: {}", string_value);
}
