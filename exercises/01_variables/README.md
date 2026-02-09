# 01. 변수와 가변성

## 학습 목표

- Rust의 불변성(immutability) 개념 이해
- `let`과 `mut` 키워드 사용법
- 변수 섀도잉(shadowing) 이해
- 상수(`const`) 선언

## 불변성 (Immutability)

Rust에서는 기본적으로 모든 변수가 **불변(immutable)**입니다.
이는 컴파일러가 더 많은 버그를 사전에 잡아낼 수 있게 해줍니다.

```rust
let x = 5;
x = 6; // ❌ 에러! x는 불변입니다
```

가변 변수가 필요하면 `mut` 키워드를 추가합니다:

```rust
let mut x = 5;
x = 6; // ✅ OK! mut를 사용했습니다
```

## Node.js 개발자를 위한 설명

JavaScript/TypeScript에서 `let`은 재할당이 가능하지만,
Rust에서는 `let`만으로는 **불변** 변수를 만듭니다.

| JavaScript/TypeScript | Rust | 설명 |
|---------------------|------|------|
| `const x = 5;` | `let x = 5;` | 불변 변수 |
| `let y = 10;` | `let mut y = 10;` | 가변 변수 |

### 왜 기본이 불변일까?

1. **안전성**: 의도하지 않은 값 변경 방지
2. **동시성**: 여러 스레드에서 안전하게 읽기 가능
3. **최적화**: 컴파일러가 더 나은 최적화 가능

## 변수 섀도잉 (Shadowing)

같은 이름으로 새로운 변수를 선언할 수 있습니다:

```rust
let x = 5;
let x = x + 1;  // ✅ 새로운 x를 만듭니다
let x = x * 2;  // ✅ 또 다른 새로운 x를 만듭니다
println!("x = {}", x);  // x = 12
```

이는 `mut`와 다릅니다:
- 섀도잉: 타입도 변경 가능
- `mut`: 같은 타입만 재할당 가능

```rust
// 섀도잉 - 타입 변경 가능
let spaces = "   ";
let spaces = spaces.len();  // ✅ String → usize

// mut - 타입 변경 불가
let mut spaces = "   ";
spaces = spaces.len();  // ❌ 에러!
```

## 상수 (Constants)

`const`는 항상 불변이며, 타입을 명시해야 합니다:

```rust
const MAX_POINTS: u32 = 100_000;
```

상수 vs 불변 변수:
- 상수는 `mut` 사용 불가
- 상수는 컴파일 타임에 값이 결정됨
- 상수는 전역 스코프에서 선언 가능
- 상수 이름은 대문자와 언더스코어 사용 (관례)

## 연습 문제

- `variables1.rs` - 기본 변수 선언
- `variables2.rs` - 가변 변수
- `variables3.rs` - 변수 섀도잉
- `variables4.rs` - 상수

## 참고 자료

- [The Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
