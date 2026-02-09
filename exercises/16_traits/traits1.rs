// traits1.rs
// 실행: cargo run --bin traits1

trait Speak {
    fn speak(&self) -> String;
}

struct Dog;

// TODO: Dog에 대한 Speak 트레이트를 구현하세요
// "Woof!"를 반환하도록

fn main() {
    let dog = Dog;
    println!("{}", dog.speak());
}

// 힌트: impl Speak for Dog { fn speak(&self) -> String { String::from("Woof!") } }
