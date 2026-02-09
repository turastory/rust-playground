# 13. Option

## 학습 목표

- Option<T> 이해
- Some과 None
- Option 다루기

## Option<T>

값이 있을 수도, 없을 수도 있는 상황을 표현:

```rust
let some_number = Some(5);
let no_number: Option<i32> = None;
```

## Node.js와 비교

JavaScript:
```javascript
const value = null;  // 또는 undefined
if (value !== null) {
    console.log(value);
}
```

Rust:
```rust
let value: Option<i32> = None;
match value {
    Some(v) => println!("{}", v),
    None => println!("No value"),
}
```

**장점**: null/undefined 관련 버그 방지!

## 참고 자료

- [The Rust Book - Option](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
