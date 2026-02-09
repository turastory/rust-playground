# 11. 벡터 (Vectors)

## 학습 목표

- Vec<T> 생성 및 사용
- 벡터 조작 (push, pop, 접근)
- 벡터 순회

## Vec<T>

가변 크기 배열입니다 (JavaScript Array와 유사):

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);

// 또는 매크로 사용
let vec = vec![1, 2, 3];
```

## Node.js와 비교

| JavaScript | Rust |
|-----------|------|
| `const arr = []` | `let mut vec = Vec::new()` |
| `const arr = [1, 2, 3]` | `let vec = vec![1, 2, 3]` |
| `arr.push(4)` | `vec.push(4)` |
| `arr.pop()` | `vec.pop()` |
| `arr[0]` | `vec[0]` or `vec.get(0)` |

## 참고 자료

- [The Rust Book - Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
