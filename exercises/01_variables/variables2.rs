// variables2.rs
//
// 변수에 값을 할당하지 않으면 사용할 수 없습니다.
//
// JavaScript와 달리, Rust는 초기화되지 않은 변수 사용을 컴파일 타임에 방지합니다.
//
// 실행: cargo run --bin variables2

fn main() {
    let x = 0;
    
    println!("x = {}", x);  // ❌ 에러! x가 초기화되지 않았습니다
}

// 힌트: x에 값을 할당하세요
