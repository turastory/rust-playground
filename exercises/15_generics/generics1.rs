// generics1.rs
// 실행: cargo run --bin generics1

// TODO: 제네릭 함수를 작성하세요
// fn identity<T>(value: T) -> T { value }

fn main() {
    let num = identity(5);
    let string = identity("hello");
    
    println!("{} {}", num, string);
}

// 힌트: fn identity<T>(value: T) -> T { value }
