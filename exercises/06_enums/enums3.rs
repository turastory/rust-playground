// enums3.rs
//
// match 표현식을 사용하는 연습입니다.
//
// 실행: cargo run --bin enums3

enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn process_message(msg: Message) {
    // TODO: match를 사용하여 각 메시지를 처리하세요
    // - Quit: "Quitting" 출력
    // - Echo(s): "Echo: {s}" 출력
    // - Move { x, y }: "Moving to ({x}, {y})" 출력
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Echo(String::from("Hello"));
    let msg3 = Message::Move { x: 10, y: 20 };
    
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
}

// 힌트: match msg { Message::Quit => ..., ... }
