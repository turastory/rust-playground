// functions3.rs
//
// 값을 반환하는 함수를 작성하는 연습입니다.
//
// JavaScript:
//   function square(num) { return num * num; }
//   const square = (num) => num * num;
//
// Rust:
//   fn square(num: i32) -> i32 { num * num }
//
// 실행: cargo run --bin functions3

fn main() {
    let result = square(5);
    println!("5의 제곱은 {}", result);
}

// TODO: 반환 타입을 추가하고 값을 반환하세요
fn square(num: i32) {
    num * num
}

// 힌트: fn square(num: i32) -> i32 { ... }
