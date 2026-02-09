// move_semantics4.rs - 정답

fn main() {
    // 정수는 Copy 타입
    let x = 5;
    let y = x;  // 복사 발생
    println!("x = {}, y = {}", x, y);
    
    // String은 Move 타입
    let s1 = String::from("hello");
    let s2 = s1;  // 이동 발생
    
    // s1은 더 이상 유효하지 않음
    println!("s2 = {}", s2);
}
