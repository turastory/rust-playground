// borrowing2.rs - 정답

fn change(s: &mut String) {
    s.push_str(", world");
}

fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    
    println!("{}", s);
}
