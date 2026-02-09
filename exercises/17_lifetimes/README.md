# 17. 라이프타임 (Lifetimes)

## 학습 목표

- 라이프타임 애너테이션
- 참조의 유효 범위

## 라이프타임

컴파일러가 참조가 유효한 시간을 추적하도록 돕습니다:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

JavaScript에는 없는 개념입니다 (GC가 알아서 처리).

## 참고 자료

- [The Rust Book - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
