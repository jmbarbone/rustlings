// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move(i32),
    ChangeColor(String),
}

// I mean, this doesn't do anything though, right?

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Narcissus")));
    println!("{:?}", Message::Move(0));
    println!("{:?}", Message::ChangeColor(String::from("blue")));
}
