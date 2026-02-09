// options1.rs - 정답

fn main() {
    let some_number = Some(5);
    
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No value"),
    }
}
