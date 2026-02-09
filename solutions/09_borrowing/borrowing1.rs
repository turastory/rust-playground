// borrowing1.rs - 정답

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    
    println!("'{}' 의 길이는 {}입니다.", s1, len);
}
