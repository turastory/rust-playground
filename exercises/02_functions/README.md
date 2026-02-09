# 02. 함수 (Functions)

## 학습 목표

- 함수 선언과 호출
- 매개변수와 반환값
- 표현식(expression) vs 구문(statement)
- 반환값 생략 (Unit 타입)

## 함수 선언

Rust에서 함수는 `fn` 키워드로 선언합니다:

```rust
fn greet() {
    println!("안녕하세요!");
}
```

## 매개변수 (Parameters)

매개변수는 반드시 타입을 명시해야 합니다:

```rust
fn greet(name: &str) {
    println!("안녕하세요, {}님!", name);
}
```

## 반환값 (Return Values)

화살표(`->`)로 반환 타입을 지정합니다:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 세미콜론 없음 - 표현식!
}
```

## Node.js 개발자를 위한 설명

| JavaScript/TypeScript | Rust |
|---------------------|------|
| `function add(a, b) { return a + b; }` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| `const add = (a, b) => a + b` | `fn add(a: i32, b: i32) -> i32 { a + b }` |
| `function greet() { console.log("Hi"); }` | `fn greet() { println!("Hi"); }` |

주요 차이점:
- **타입 명시 필수**: 매개변수와 반환값의 타입을 반드시 지정
- **화살표 함수 없음**: 하지만 클로저(closure)가 비슷한 역할
- **세미콜론 주의**: 마지막 표현식은 세미콜론 없이 반환

## 표현식 vs 구문

이것이 Rust에서 가장 중요한 개념 중 하나입니다!

**구문(Statement)**: 동작을 수행하지만 값을 반환하지 않음
```rust
let x = 5;  // 구문
```

**표현식(Expression)**: 값을 평가함
```rust
5 + 6  // 표현식 (11을 반환)
{
    let x = 3;
    x + 1  // 표현식 (4를 반환)
}
```

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // ✅ 표현식 - 값을 반환
}

fn add_wrong(a: i32, b: i32) -> i32 {
    a + b;  // ❌ 구문 - 세미콜론 때문에 값을 반환하지 않음!
}
```

JavaScript에서는:
```javascript
// JavaScript - 명시적 return 필요
function add(a, b) {
    return a + b;  // return 키워드 필수
}

// 화살표 함수에서만 암묵적 반환
const add = (a, b) => a + b;
```

Rust에서는:
```rust
// Rust - 마지막 표현식이 자동으로 반환값
fn add(a: i32, b: i32) -> i32 {
    a + b  // return 키워드 없이 반환
}

// 명시적 return도 가능
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // 하지만 관례상 마지막에는 사용 안 함
}
```

## Unit 타입 ()

반환값이 없는 함수는 `()` (Unit 타입)을 반환합니다:

```rust
fn greet() -> () {  // 반환 타입 생략 가능
    println!("Hi");
}

// 위와 같음
fn greet() {
    println!("Hi");
}
```

## 연습 문제

- `functions1.rs` - 기본 함수 선언
- `functions2.rs` - 매개변수
- `functions3.rs` - 반환값
- `functions4.rs` - 표현식 vs 구문

## 참고 자료

- [The Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
