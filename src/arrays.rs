// Fixed list of same-type elements (lok at vectors for growable arrays)
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [0, 1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("Single value from array[4]: {}", numbers[4]);

    numbers[4] = 55;
    println!("Single value from array[4]: {}", numbers[4]);
    println!("Length of the array: {}, stack allocation (in bytes): {}. Funny that! an array of 5 elements that take 4 bytes each. Magic!", numbers.len(), mem::size_of_val(&numbers));

    // Get a slice from an array
    let slice: &[i32] = &numbers[2..4];

    println!("Slice: {:?}", slice);
}