// structs1.rs
//
// 구조체를 정의하는 연습입니다.
//
// 실행: cargo run --bin structs1

// TODO: Person 구조체를 정의하세요
// 필드: name (String), age (u32)

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("{} is {} years old.", person.name, person.age);
}

// 힌트: struct Person { name: String, age: u32, }
