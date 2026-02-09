# 15. 제네릭 (Generics)

## 학습 목표

- 제네릭 함수, 구조체, 열거형
- 타입 매개변수

## 제네릭

여러 타입에 대해 작동하는 코드 작성:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

TypeScript와 유사:
```typescript
function largest<T>(list: T[]): T {
    // ...
}
```

## 참고 자료

- [The Rust Book - Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)
