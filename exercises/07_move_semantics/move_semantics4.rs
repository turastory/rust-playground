// move_semantics4.rs
//
// Copy 타입과 Move 타입의 차이를 이해하는 연습입니다.
//
// 실행: cargo run --bin move_semantics4

fn main() {
    // 정수는 Copy 타입
    let x = 5;
    let y = x;  // 복사 발생
    println!("x = {}, y = {}", x, y);  // ✅ 둘 다 유효
    
    // String은 Move 타입
    let s1 = String::from("hello");
    let s2 = s1;  // 이동 발생
    
    // TODO: 아래 주석을 해제하면 에러가 발생합니다
    // println!("s1 = {}, s2 = {}", s1, s2);
    
    println!("s2 = {}", s2);
}

// 힌트: 스택 데이터(정수 등)는 Copy, 힙 데이터(String 등)는 Move
