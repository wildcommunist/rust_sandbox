/*
Primitive types
integers: u8,i8,u16,i16,u32,i32mu64,i64,u128,i128
floats: f32,f64
booleans
characters
tuples
arrays
 */

pub fn run() {

    // Default is i32
    let x = 1;

    // Default float64
    let y = 2.75;

    // Explicit
    let z: i64 = 1337;

    // find max size
    println!("Max size of i32 is: {}", std::i32::MAX);
    println!("Max size of i64 is: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;
    println!("The user is active: {:?}", (x, y, z, is_active));

    // Boolean from expression
    let is_greater = 3 > 2;
    println!("3 > 2 = {}", is_greater);

    //Characters (unicode chars)
    let a1 = '\u{1F600}';
    println!("{:?}", a1);
}