// move_semantics1.rs
//
// 기본적인 이동을 이해하는 연습입니다.
//
// 실행: cargo run --bin move_semantics1

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1이 s2로 이동
    
    // TODO: 아래 라인의 주석을 해제하면 에러가 발생합니다. 왜 그럴까요?
    // println!("{}", s1);
    
    println!("{}", s2);
}

// 힌트: s1의 소유권이 s2로 이동되어 s1은 더 이상 유효하지 않습니다
