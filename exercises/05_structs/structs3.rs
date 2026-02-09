// structs3.rs
//
// 메서드를 정의하는 연습입니다.
//
// 실행: cargo run --bin structs3

struct Rectangle {
    width: u32,
    height: u32,
}

// TODO: impl 블록을 추가하고 area 메서드를 정의하세요
// area는 사각형의 넓이(width * height)를 반환해야 합니다

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("사각형의 넓이: {}", rect.area());
}

// 힌트:
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
