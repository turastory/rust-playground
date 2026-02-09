# 18. 반복자 (Iterators)

## 학습 목표

- Iterator 트레이트
- map, filter, collect
- 반복자 어댑터

## 반복자

JavaScript 배열 메서드와 유사합니다:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .collect();
```

## 참고 자료

- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
