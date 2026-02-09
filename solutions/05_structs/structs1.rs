// structs1.rs - 정답

struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    
    println!("{} is {} years old.", person.name, person.age);
}
