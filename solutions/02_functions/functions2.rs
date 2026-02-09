// functions2.rs - 정답

fn main() {
    call_me(3);
}

fn call_me(num: i32) {  // 타입 명시
    for i in 0..num {
        println!("Ring! 호출 번호 {}", i + 1);
    }
}
