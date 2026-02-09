// enums1.rs
//
// 열거형을 정의하는 연습입니다.
//
// 실행: cargo run --bin enums1

// TODO: Message 열거형을 정의하세요
// Variants: Quit, Echo, Move

fn main() {
    let quit = Message::Quit;
    let echo = Message::Echo;
    let move_msg = Message::Move;
    
    println!("열거형 정의 완료!");
}

// 힌트: enum Message { Quit, Echo, Move, }
