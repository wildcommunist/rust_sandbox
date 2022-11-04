pub fn run() {
    greetings("Welcome", "Alex");
    let result = add(44, 978);
    println!("Sum is: {}", result);

    let n3: i32 = 1337;

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Adding via closure: {}", add_nums(3, n3));
}

fn greetings(greet: &str, name: &str) {
    println!("{}, {}, nice to meet you", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}