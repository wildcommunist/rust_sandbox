// Enums are type which have few definite values

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
    Jump,
}

fn move_avatar(m: Movement) {
    // Perform action based on movement type
    match m {
        Movement::Up => { println!("Moving UP") }
        Movement::Down => { println!("Moving DOWN") }
        Movement::Left => { println!("Moving LEFT") }
        Movement::Right => { println!("Moving RIGHT") }
        _ => { println!("UNHANDLED MOVEMENT") }
    }
}

pub fn run() {
    let a1 = Movement::Left;
    let a2 = Movement::Right;
    let a3 = Movement::Up;
    let a4 = Movement::Down;
    let a5 = Movement::Jump;
    move_avatar(a1);
    move_avatar(a2);
    move_avatar(a3);
    move_avatar(a4);
    move_avatar(a5);
}