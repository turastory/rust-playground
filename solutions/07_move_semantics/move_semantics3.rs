// move_semantics3.rs - 정답

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 깊은 복사
    
    println!("s1 = {}, s2 = {}", s1, s2);
}
