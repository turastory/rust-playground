// enums2.rs - 정답

enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Echo(String::from("hello"));
    
    println!("메시지 생성 완료!");
}
