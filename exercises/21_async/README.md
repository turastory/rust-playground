# 21. 비동기 프로그래밍 (Async Programming)

## 학습 목표

- async/await
- Future
- tokio 런타임

## async/await

JavaScript Promise와 유사:

```rust
async fn fetch_data() -> String {
    String::from("data")
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

## 참고 자료

- [The Rust Book - Async](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
