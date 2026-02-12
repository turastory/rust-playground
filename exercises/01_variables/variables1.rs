// variables1.rs
//
// Rust에서는 기본적으로 모든 변수가 불변(immutable)입니다.
//
// JavaScript:
//   const x = 5;  // 불변
//   let y = 10;   // 가변
//
// Rust:
//   let x = 5;      // 불변
//   let mut y = 10; // 가변 (mut 키워드 필요!)
//
// 아래 코드를 수정하여 컴파일되도록 만드세요.
//
// 실행: cargo run --bin variables1

fn main() {
    let x = 5;
    println!("x = {}", x);
    
    x = 6;  // ❌ 에러가 발생합니다!
    
    println!("x = {}", x);
}

// 힌트: x를 가변으로 만들려면 어떻게 해야 할까요?
