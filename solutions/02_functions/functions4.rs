// functions4.rs - 정답

fn main() {
    let result = calculate();
    println!("결과: {}", result);
}

fn calculate() -> i32 {
    let x = 5;
    let y = 10;
    
    x + y  // 세미콜론 제거 - 표현식!
}
