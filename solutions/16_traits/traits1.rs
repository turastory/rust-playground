// traits1.rs - 정답

trait Speak {
    fn speak(&self) -> String;
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("Woof!")
    }
}

fn main() {
    let dog = Dog;
    println!("{}", dog.speak());
}
