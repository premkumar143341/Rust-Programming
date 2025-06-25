/*Write a program to implement the Scope and Shadowing*/
fn main() {
    let x = 5; // Outer scope x
    println!("Outer x = {}", x);

    {
        // Start inner scope
        let x = 10; // Shadowing outer x in this inner scope
        println!("Inner scope shadowed x = {}", x);
    } // Inner scope ends here

    println!("Outer x after inner scope = {}", x);

    // Shadowing again in the same scope
    let x = x + 5; 
    println!("Shadowed x in outer scope = {}", x);
}
