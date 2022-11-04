// Variables are immutable by default!!
pub fn run() {
    let name = "Alex";
    let mut language = "golang";

    println!("My name is {} and I like writing code in {}.", name, language);
    language = "rust";
    println!("{} has changed his preference to {}.", name, language);

    // Constants - usually all in upper
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multiple assignments at once
    let (person, age, country) = ("Alex", 35, "Kazakhstan");
    println!("Name: {}\nAge: {}\nCountry: {}", person, age, country);
}