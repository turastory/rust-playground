# 16. 트레이트 (Traits)

## 학습 목표

- 트레이트 정의 및 구현
- 기본 구현
- 트레이트 바운드

## 트레이트

공통 동작을 정의합니다 (TypeScript interface와 유사):

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        self.title.clone()
    }
}
```

## 참고 자료

- [The Rust Book - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
