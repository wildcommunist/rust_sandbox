// Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![0, 1, 2, 3, 4];

    println!("{:?}", numbers);
    println!("Single value from array[4]: {}", numbers[4]);

    numbers.push(9);
    numbers.push(10);

    numbers[4] = 55;
    println!("Single value from vector[4]: {}", numbers[4]);
    println!("Length of the vector: {}, stack allocation (in bytes): {}", numbers.len(), mem::size_of_val(&numbers));

    // Get a slice from an array
    let mut slice: &[i32] = &numbers[2..4];
    println!("Slice: {:?}", slice);

    println!("Slice: {:?}", numbers);

    // Pop an element
    numbers.pop();
    println!("Vector: {:?}", numbers);

    // Loop though vec values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    println!("Vector: {:?}", numbers);
    // Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
        println!("Number: {}", x);
    }
    println!("Vector: {:?}", numbers);
}