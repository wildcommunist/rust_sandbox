pub fn run() {
    let age = 21;
    let check_id = true;
    let vip_person_of_age = true;

    // If/else
    if age >= 21 && check_id || vip_person_of_age {
        println!("Bartender: You are of age (if you live in the USA :p). What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Please wait few more years before trying that one again!");
    } else {
        println!("Bartender: I need to see your ID");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age)
}