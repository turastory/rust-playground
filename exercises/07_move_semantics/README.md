# 07. 이동 의미론 (Move Semantics)

## 학습 목표

- 이동(move)과 복사(copy)의 차이 이해
- 값이 이동되는 시점 파악
- Clone 트레이트 사용

## 이동 의미론

Rust에서는 값을 할당하거나 함수에 전달할 때 **이동(move)**이 발생합니다:

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1의 값이 s2로 이동

println!("{}", s1);  // ❌ 에러! s1은 더 이상 유효하지 않음
```

## Node.js 개발자를 위한 설명

JavaScript:
```javascript
// JavaScript - 참조 복사
const s1 = { value: "hello" };
const s2 = s1;  // s1과 s2는 같은 객체를 가리킴

console.log(s1.value);  // ✅ OK - "hello"
console.log(s2.value);  // ✅ OK - "hello"

s2.value = "world";
console.log(s1.value);  // "world" - s1도 변경됨!
```

Rust:
```rust
// Rust - 이동 (소유권 이전)
let s1 = String::from("hello");
let s2 = s1;  // s1의 소유권이 s2로 이동

println!("{}", s1);  // ❌ 컴파일 에러!
println!("{}", s2);  // ✅ OK
```

**핵심 차이점**:
- JavaScript: 여러 변수가 같은 객체 참조 가능 (GC가 정리)
- Rust: 한 번에 하나의 소유자만 가능 (이동 후 이전 변수 무효화)

## Copy vs Move

### Copy 타입 (스택 데이터)

```rust
let x = 5;
let y = x;  // 복사 발생 (이동 아님)

println!("x = {}, y = {}", x, y);  // ✅ 둘 다 유효
```

Copy 트레이트를 구현하는 타입:
- 모든 정수, 부동소수점 타입
- bool, char
- Copy 타입만으로 구성된 튜플

### Move 타입 (힙 데이터)

```rust
let s1 = String::from("hello");
let s2 = s1;  // 이동 발생

// s1은 더 이상 사용 불가
```

Move가 발생하는 타입:
- String
- Vec<T>
- 구조체 (기본적으로)

## Clone으로 깊은 복사

명시적으로 데이터를 복제하려면 `clone()` 사용:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 깊은 복사

println!("s1 = {}, s2 = {}", s1, s2);  // ✅ 둘 다 유효
```

JavaScript 비교:
```javascript
// JavaScript - 명시적 복사 필요
const s1 = { value: "hello" };
const s2 = { ...s1 };  // 또는 structuredClone(s1)
```

## 함수와 이동

```rust
fn take_ownership(s: String) {
    println!("{}", s);
}  // s가 여기서 drop됨

let s = String::from("hello");
take_ownership(s);  // s의 소유권이 함수로 이동
// println!("{}", s);  // ❌ 에러!
```

소유권을 유지하려면:
```rust
fn borrow(s: &String) {  // 참조로 빌림
    println!("{}", s);
}

let s = String::from("hello");
borrow(&s);  // 소유권 유지
println!("{}", s);  // ✅ OK
```

## 연습 문제

- `move_semantics1.rs` - 기본 이동
- `move_semantics2.rs` - 함수와 이동
- `move_semantics3.rs` - Clone 사용
- `move_semantics4.rs` - 이동 vs 복사

## 참고 자료

- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
