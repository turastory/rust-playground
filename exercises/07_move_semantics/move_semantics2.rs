// move_semantics2.rs
//
// 함수로 값을 전달할 때 이동이 발생합니다.
//
// 실행: cargo run --bin move_semantics2

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    
    print_string(s);
    
    // TODO: 다시 s를 사용하려고 하면 에러가 발생합니다. 어떻게 고칠까요?
    print_string(s);
}

// 힌트 1: clone()을 사용하여 복사본을 전달
// 힌트 2: 또는 참조(&s)를 사용 (다음 섹션에서 배움)
