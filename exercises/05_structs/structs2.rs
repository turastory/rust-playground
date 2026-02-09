// structs2.rs
//
// 가변 구조체 인스턴스를 다루는 연습입니다.
//
// 실행: cargo run --bin structs2

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Bob"),
        age: 25,
    };
    
    // TODO: person의 age를 26으로 변경하세요
    // person.age = 26;
    
    println!("{} is {} years old.", person.name, person.age);
}

// 힌트: person을 가변(mut)으로 만들어야 합니다
