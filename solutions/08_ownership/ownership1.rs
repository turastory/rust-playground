// ownership1.rs - 정답

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
}
