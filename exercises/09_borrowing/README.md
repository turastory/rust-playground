# 09. 빌림 (Borrowing)

## 학습 목표

- 참조(&)와 역참조(*)
- 불변 참조와 가변 참조
- 빌림 규칙
- 댕글링 참조 방지

## 참조와 빌림

소유권을 이전하지 않고 값을 사용하려면 **참조(reference)**를 사용합니다:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s는 drop되지 않음 (소유권이 없으므로)

let s1 = String::from("hello");
let len = calculate_length(&s1);  // 빌림
println!("{} {}", s1, len);  // s1은 여전히 유효
```

## Node.js와의 비교

JavaScript:
```javascript
function calculateLength(s) {
    return s.length;
}

const s1 = "hello";
const len = calculateLength(s1);
console.log(s1, len);  // 문제없음
```

Rust에서는 명시적으로 참조(`&`)를 사용합니다.

## 빌림 규칙

1. 어느 한 시점에, **하나의 가변 참조** 또는 **여러 개의 불변 참조** 가능
2. 참조는 항상 유효해야 함

```rust
let mut s = String::from("hello");

// ✅ OK - 여러 개의 불변 참조
let r1 = &s;
let r2 = &s;

// ❌ 에러 - 가변 참조와 불변 참조를 동시에
let r3 = &mut s;
```

## 가변 참조

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s);  // "hello, world"
```

## 왜 이런 규칙이 필요할까?

**데이터 레이스(data race)** 방지:
- 두 개 이상의 포인터가 동시에 같은 데이터 접근
- 최소 하나가 쓰기 작업
- 동기화 메커니즘 없음

JavaScript:
```javascript
// JavaScript - 레이스 컨디션 가능
const arr = [1, 2, 3];
arr.forEach(x => arr.push(x * 2));  // 무한 루프 위험!
```

Rust:
```rust
// Rust - 컴파일 타임에 방지
let mut vec = vec![1, 2, 3];
for x in &vec {
    vec.push(x * 2);  // ❌ 컴파일 에러!
}
```

## 연습 문제

- `borrowing1.rs` - 불변 참조
- `borrowing2.rs` - 가변 참조
- `borrowing3.rs` - 빌림 규칙
- `borrowing4.rs` - 댕글링 참조

## 참고 자료

- [The Rust Book - References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
