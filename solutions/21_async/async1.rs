// async1.rs - 정답

async fn say_hello() -> String {
    String::from("Hello, async!")
}

#[tokio::main]
async fn main() {
    let message = say_hello().await;
    println!("{}", message);
}
