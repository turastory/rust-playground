// control_flow4.rs - 정답

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    for num in numbers {
        println!("{}", num);
    }
    
    println!("\n범위 사용:");
    for i in 1..=5 {
        println!("{}", i);
    }
}
