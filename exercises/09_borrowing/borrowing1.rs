// borrowing1.rs
// 실행: cargo run --bin borrowing1

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    
    // TODO: calculate_length에 s1의 참조를 전달하세요
    // let len = calculate_length(???);
    
    println!("'{}' 의 길이는 {}입니다.", s1, len);
}

// 힌트: &s1
