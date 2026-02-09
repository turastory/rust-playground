// ownership1.rs
// 실행: cargo run --bin ownership1

fn main() {
    let s = String::from("hello");
    
    // TODO: takes_ownership 함수를 정의하세요
    // 이 함수는 String을 받아서 출력합니다
    
    takes_ownership(s);
    
    // s는 더 이상 유효하지 않습니다
}

// 힌트: fn takes_ownership(s: String) { println!("{}", s); }
