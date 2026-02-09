// borrowing2.rs
// 실행: cargo run --bin borrowing2

fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let s = String::from("hello");
    
    // TODO: s를 가변으로 만들고 change 함수에 가변 참조를 전달하세요
    change(&mut s);
    
    println!("{}", s);
}

// 힌트: let mut s = ... 그리고 &mut s
