// functions4.rs
//
// 표현식 vs 구문을 이해하는 연습입니다.
// 세미콜론의 유무가 중요합니다!
//
// 실행: cargo run --bin functions4

fn main() {
    let result = calculate();
    println!("결과: {}", result);
}

fn calculate() -> i32 {
    let x = 5;
    let y = 10;
    
    // TODO: x + y의 결과를 반환하세요
    // 힌트: 세미콜론을 제거하면 표현식이 됩니다!
    x + y;
}

// 힌트: 마지막 줄의 세미콜론을 제거하세요
