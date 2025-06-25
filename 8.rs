/*Write a program to create an array of 10 elements and implement the following
a. Create a of 2nd and 3rd element
b. Omit the start index of the slice
c. Omit the End Index of the Slice
d. Omit both Start and End Index of the Slice*/
fn main() {
    // Create an array of 10 elements
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // a. Create a slice of 2nd and 3rd elements (indices 1 and 2)
    let slice_2nd_3rd = &arr[1..3];  // inclusive start, exclusive end
    println!("Slice of 2nd and 3rd elements: {:?}", slice_2nd_3rd);

    // b. Omit the start index of the slice (from start to index 4)
    let slice_start_omitted = &arr[..5];  // from 0 to 4 (5 excluded)
    println!("Slice omitting start index (first 5 elements): {:?}", slice_start_omitted);

    // c. Omit the end index of the slice (from index 5 to end)
    let slice_end_omitted = &arr[5..];  // from 5 to end
    println!("Slice omitting end index (elements from index 5 to end): {:?}", slice_end_omitted);

    // d. Omit both start and end index of the slice (whole array)
    let slice_whole = &arr[..];  // entire array
    println!("Slice omitting both start and end indices (whole array): {:?}", slice_whole);
}
