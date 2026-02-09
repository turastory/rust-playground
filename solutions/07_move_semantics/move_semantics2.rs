// move_semantics2.rs - 정답

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    
    print_string(s.clone());  // 복사본 전달
    print_string(s);  // 원본 사용
}
