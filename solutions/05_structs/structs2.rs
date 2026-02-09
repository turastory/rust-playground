// structs2.rs - 정답

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut person = Person {  // mut 추가
        name: String::from("Bob"),
        age: 25,
    };
    
    person.age = 26;
    
    println!("{} is {} years old.", person.name, person.age);
}
