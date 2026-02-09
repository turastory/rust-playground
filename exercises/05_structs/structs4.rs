// structs4.rs
//
// 연관 함수를 정의하는 연습입니다.
//
// 실행: cargo run --bin structs4

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // TODO: 정사각형을 만드는 연관 함수 square를 정의하세요
    // square(size) -> width와 height가 모두 size인 Rectangle 반환
}

fn main() {
    let sq = Rectangle::square(10);
    println!("정사각형: {}x{}", sq.width, sq.height);
}

// 힌트:
// fn square(size: u32) -> Rectangle {
//     Rectangle { width: size, height: size }
// }
