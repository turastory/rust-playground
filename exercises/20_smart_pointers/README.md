# 20. 스마트 포인터 (Smart Pointers)

## 학습 목표

- Box<T>
- Rc<T>
- RefCell<T>

## Box<T>

힙에 데이터 할당:

```rust
let b = Box::new(5);
println!("{}", b);
```

## 참고 자료

- [The Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
