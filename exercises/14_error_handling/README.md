# 14. 에러 처리 (Error Handling)

## 학습 목표

- Result<T, E> 이해
- ? 연산자 사용
- panic! vs Result

## Result<T, E>

작업이 성공하거나 실패할 수 있는 상황:

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
```

## Node.js와 비교

JavaScript:
```javascript
function divide(a, b) {
    if (b === 0) {
        throw new Error("Division by zero");
    }
    return a / b;
}

try {
    const result = divide(10, 0);
} catch (error) {
    console.error(error);
}
```

Rust:
```rust
match divide(10, 0) {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("{}", e),
}
```

## ? 연산자

에러를 전파하는 간편한 방법:

```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}
```

## 참고 자료

- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
