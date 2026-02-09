// lifetimes1.rs
// 실행: cargo run --bin lifetimes1

// TODO: 라이프타임 애너테이션을 추가하세요
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long");
    let s2 = String::from("short");
    
    let result = longest(&s1, &s2);
    println!("{}", result);
}

// 힌트: fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
