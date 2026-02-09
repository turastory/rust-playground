// move_semantics1.rs - 정답

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1이 s2로 이동
    
    // s1은 더 이상 유효하지 않으므로 사용할 수 없음
    
    println!("{}", s2);
}
