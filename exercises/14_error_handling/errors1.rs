// errors1.rs
// 실행: cargo run --bin errors1

fn divide(a: i32, b: i32) -> Result<i32, String> {
    // TODO: b가 0이면 Err, 아니면 Ok(a / b) 반환
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

// 힌트: if b == 0 { Err(...) } else { Ok(...) }
