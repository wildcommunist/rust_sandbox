pub fn run() {

    // Print to console
    println!("Hello from the print.rs file!");

    // BAsic formatting
    println!("We are formatting stuff: {} > 0 = {}", 1, true);

    // Positional arguments
    println!("{0} is an awesome {1} because {0} likes rust!", "Alex", "guy");

    // Named args
    println!("{name} enjoys writing {lang} code.", name = "Alex", lang = "rust");

    // Placeholder traits
    println!("Binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    // Debug trait
    println!("{:?}", (1337, true, "amazing"));

    // Basic operations
    println!("{} * {} = {}", 10, 10, 10 * 10)
}