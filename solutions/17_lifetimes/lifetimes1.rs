// lifetimes1.rs - 정답

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long");
    let s2 = String::from("short");
    
    let result = longest(&s1, &s2);
    println!("{}", result);
}
