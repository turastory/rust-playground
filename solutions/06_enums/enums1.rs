// enums1.rs - 정답

enum Message {
    Quit,
    Echo,
    Move,
}

fn main() {
    let quit = Message::Quit;
    let echo = Message::Echo;
    let move_msg = Message::Move;
    
    println!("열거형 정의 완료!");
}
