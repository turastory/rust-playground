# 08. 소유권 (Ownership)

## 학습 목표

- 소유권 규칙 이해
- 스택과 힙 메모리
- 함수의 소유권 전달과 반환

## 소유권 규칙

1. Rust의 각 값은 **소유자(owner)**가 있다
2. 한 번에 딱 **하나의 소유자**만 존재
3. 소유자가 스코프 밖으로 벗어나면 값은 **버려진다(dropped)**

## Node.js와의 비교

JavaScript (가비지 컬렉션):
```javascript
function process() {
    const data = { value: 100 };
    // 함수 종료 후에도 data는 GC가 정리할 때까지 메모리에 존재
}
// GC가 나중에 메모리 정리
```

Rust (소유권 시스템):
```rust
fn process() {
    let data = String::from("hello");
    // 함수 종료 시 즉시 data의 메모리 해제
}
```

**장점**:
- 예측 가능한 메모리 해제
- GC 오버헤드 없음
- 메모리 안전성 보장 (댕글링 포인터, 더블 프리 등 방지)

## 스택 vs 힙

| 스택 | 힙 |
|-----|-----|
| 고정 크기 데이터 | 가변 크기 데이터 |
| 빠름 | 상대적으로 느림 |
| i32, bool 등 | String, Vec 등 |
| 자동 관리 | 소유권으로 관리 |

## 함수와 소유권

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s가 drop됨

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s  // 소유권 반환
}
```

## 연습 문제

- `ownership1.rs` - 기본 소유권
- `ownership2.rs` - 함수와 소유권
- `ownership3.rs` - 소유권 반환
- `ownership4.rs` - 스코프

## 참고 자료

- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
