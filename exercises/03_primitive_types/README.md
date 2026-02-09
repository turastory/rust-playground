# 03. 기본 타입 (Primitive Types)

## 학습 목표

- 정수, 부동소수점, 불리언, 문자 타입 이해
- 튜플(tuple)과 배열(array) 사용
- 타입 추론과 명시적 타입 지정

## 스칼라 타입

### 정수 (Integer)

| 길이 | 부호 있음 | 부호 없음 |
|-----|---------|---------|
| 8비트 | i8 | u8 |
| 16비트 | i16 | u16 |
| 32비트 | i32 | u32 |
| 64비트 | i64 | u64 |
| 128비트 | i128 | u128 |
| arch | isize | usize |

기본값: `i32`

```rust
let x = 42;        // i32 (기본)
let y: u8 = 255;   // u8 (명시적)
let z = 1_000;     // 가독성을 위한 언더스코어
```

### 부동소수점 (Floating Point)

```rust
let x = 2.0;       // f64 (기본)
let y: f32 = 3.0;  // f32
```

### 불리언 (Boolean)

```rust
let t = true;
let f: bool = false;
```

### 문자 (Character)

```rust
let c = 'z';
let emoji = '😀';  // 유니코드 지원!
```

## Node.js 개발자를 위한 설명

| JavaScript | Rust | 설명 |
|-----------|------|------|
| `number` | `i32`, `f64`, etc. | JS는 모두 number, Rust는 구분 |
| `boolean` | `bool` | 동일 |
| `string` | `char` | char는 단일 문자, string은 다름 |

JavaScript:
```javascript
let x = 42;        // number
let y = 3.14;      // number (구분 없음)
let b = true;      // boolean
let c = 'z';       // string (단일 문자도)
```

Rust:
```rust
let x = 42;        // i32 (정수)
let y = 3.14;      // f64 (부동소수점)
let b = true;      // bool
let c = 'z';       // char (단일 문자)
```

## 복합 타입

### 튜플 (Tuple)

서로 다른 타입의 값을 그룹화:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 구조 분해
let (x, y, z) = tup;

// 인덱스 접근
let first = tup.0;
let second = tup.1;
```

JavaScript 비교:
```javascript
// JavaScript - 배열 사용
const tup = [500, 6.4, 1];
const [x, y, z] = tup;
```

### 배열 (Array)

같은 타입의 고정 길이 컬렉션:

```rust
let arr = [1, 2, 3, 4, 5];
let arr: [i32; 5] = [1, 2, 3, 4, 5];  // 타입과 길이 명시

// 같은 값으로 초기화
let arr = [3; 5];  // [3, 3, 3, 3, 3]

// 인덱스 접근
let first = arr[0];
```

JavaScript 비교:
```javascript
// JavaScript - 배열은 동적 크기
const arr = [1, 2, 3, 4, 5];
arr.push(6);  // 크기 변경 가능

// Rust 배열은 고정 크기!
// 동적 크기가 필요하면 Vec<T> 사용 (나중에 학습)
```

## 연습 문제

- `primitive_types1.rs` - 정수와 부동소수점
- `primitive_types2.rs` - 문자와 불리언
- `primitive_types3.rs` - 튜플
- `primitive_types4.rs` - 배열

## 참고 자료

- [The Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
