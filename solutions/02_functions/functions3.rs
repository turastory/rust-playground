// functions3.rs - 정답

fn main() {
    let result = square(5);
    println!("5의 제곱은 {}", result);
}

fn square(num: i32) -> i32 {  // 반환 타입 추가
    num * num
}
