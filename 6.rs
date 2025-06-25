/*Write Program to Declare an array, arr, of size 6 that has numbers divisible by 2 ranging
from 0 to 10 and Print the value of arr. */
fn main() {
    // Declare array with numbers divisible by 2 from 0 to 10 (0,2,4,6,8,10)
    let arr: [i32; 6] = [0, 2, 4, 6, 8, 10];

    // Print the entire array using debug format specifier
    println!("arr = {:?}", arr);
}
