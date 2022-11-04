// Loves tuples :D - just a group of values (of any types!)
// Gotcha: max of 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("Alex", "Kazakhstan", 37);
    println!("{:?}", person);
    println!("{} is from {} and he is {} years of age", person.0, person.1, person.2);
}