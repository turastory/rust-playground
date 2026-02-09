// enums2.rs
//
// 데이터를 포함하는 열거형을 다루는 연습입니다.
//
// 실행: cargo run --bin enums2

enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
}

fn main() {
    // TODO: Message::Echo에 문자열을 전달하여 인스턴스를 만드세요
    // let msg = Message::Echo(???);
    
    println!("메시지 생성 완료!");
}

// 힌트: Message::Echo(String::from("hello"))
