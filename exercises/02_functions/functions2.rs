// functions2.rs
//
// 매개변수를 받는 함수를 작성하는 연습입니다.
// Rust에서는 매개변수의 타입을 반드시 명시해야 합니다.
//
// 실행: cargo run --bin functions2

fn main() {
    call_me(3);
}

// TODO: 매개변수 타입을 추가하세요
fn call_me(num) {
    for i in 0..num {
        println!("Ring! 호출 번호 {}", i + 1);
    }
}

// 힌트: fn call_me(num: i32) { ... }
