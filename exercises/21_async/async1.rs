// async1.rs
// 실행: cargo run --bin async1

// TODO: async 함수를 작성하세요
// async fn say_hello() -> String {
//     String::from("Hello, async!")
// }

#[tokio::main]
async fn main() {
    let message = say_hello().await;
    println!("{}", message);
}

// 힌트: async fn say_hello() -> String { String::from("Hello, async!") }
