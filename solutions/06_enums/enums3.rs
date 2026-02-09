// enums3.rs - 정답

enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Echo(s) => println!("Echo: {}", s),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
    }
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Echo(String::from("Hello"));
    let msg3 = Message::Move { x: 10, y: 20 };
    
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
}
