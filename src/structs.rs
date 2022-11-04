// Custom data types on steroids!

// Traditional struct
#[derive(Debug)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
#[derive(Debug)]
struct FancyColour(u8, u8, u8);

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{}, {}", self.last_name.to_uppercase(), self.first_name)
    }

    // Set last name
    fn set_last_name(&mut self, new_last: &str) {
        self.last_name = new_last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.last_name, self.first_name)
    }
}

pub fn run() {
    let mut c: Colour = Colour {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: R:{}, G:{}, B:{}\n\t{:?}", c.red, c.green, c.blue, c);

    c.red = 128;

    println!("Color: R:{}, G:{}, B:{}\n\t{:?}", c.red, c.green, c.blue, c);

    let mut c1 = FancyColour(128, 128, 0);
    println!("Fancy colour: {} {} {}", c1.0, c1.1, c1.2);

    c1.2 = 255;

    println!("Fancy colour: {} {} {}\n\t{:?}", c1.0, c1.1, c1.2, c1);

    let mut p = Person::new("Mary", "Doe");
    println!("Person: {}, {}", p.last_name.to_uppercase(), p.first_name);
    p.first_name = "Katie".to_string();
    println!("Person: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());
    println!("Person: {:?}", p.to_tuple());
}