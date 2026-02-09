// generics1.rs - 정답

fn identity<T>(value: T) -> T {
    value
}

fn main() {
    let num = identity(5);
    let string = identity("hello");
    
    println!("{} {}", num, string);
}
