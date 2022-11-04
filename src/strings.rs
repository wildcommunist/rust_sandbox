/*
str = immutavle, fixed (stack))
String = growable (STORED ON TEH HEAP!)
 */
pub fn run() {
    let hello = "Hello";
    let mut greeting = String::from(hello);
    println!("Capacity: {}", greeting.capacity());
    println!("primitive: {}, growable: {} ({:?})", hello, greeting, (hello.len(), greeting.len()));

    greeting.push(',');
    greeting.push_str(" Goodbye! ");
    println!("primitive: {}, growable: {} ({:?})", hello, greeting, (hello.len(), greeting.len()));

    // Capacity in bytes changes because our string grew!
    println!("Capacity: {}", greeting.capacity());

    // Is empty
    println!("Is empty: {}", greeting.is_empty());

    // Substrings
    println!("Contains bye: {}", greeting.contains("bye"));

    // Replace
    println!("Contains bye: {}", greeting.replace("bye", "day"));
    greeting.push_str("This is the greatest show!");

    // Loop by whitespaces
    for word in greeting.split_whitespace() {
        println!("{}", word)
    }

    // String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertions
    assert_eq!(s.len(), 2);
    assert_eq!(s.capacity(), 10);


    println!("{:?}", s);
}