# 10. 문자열 (Strings)

## 학습 목표

- String vs &str 차이점
- 문자열 생성 및 조작
- UTF-8 인코딩
- 문자열 슬라이스

## String vs &str

Rust에는 두 가지 주요 문자열 타입이 있습니다:

### String (소유된 문자열)

- 힙에 할당
- 가변 크기
- 소유권 가짐

```rust
let mut s = String::from("hello");
s.push_str(", world");
```

### &str (문자열 슬라이스)

- 문자열의 일부를 참조
- 불변
- 소유권 없음

```rust
let s = "hello";  // &str 타입
let slice = &s[0..2];  // "he"
```

## Node.js와의 비교

JavaScript:
```javascript
// JavaScript - 하나의 string 타입
let s = "hello";
s += " world";  // 새로운 문자열 생성
const slice = s.slice(0, 5);
```

Rust:
```rust
// String - 가변, 소유됨
let mut s = String::from("hello");
s.push_str(" world");

// &str - 불변, 참조
let hello = &s[0..5];
```

**왜 두 가지 타입?**
- `&str`: 효율적 (복사 없이 참조)
- `String`: 유연함 (수정 가능)

## 문자열 생성

```rust
// 리터럴에서
let s1 = String::from("hello");

// to_string() 사용
let s2 = "hello".to_string();

// 빈 문자열
let mut s3 = String::new();
s3.push_str("hello");
```

## 문자열 조작

```rust
let mut s = String::from("hello");

// 추가
s.push_str(" world");  // 문자열 추가
s.push('!');           // 문자 추가

// 연결
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1은 이동됨

// format! 매크로 (소유권 이동 안 함)
let s = format!("{}{}", s2, "!");
```

## UTF-8

Rust 문자열은 UTF-8로 인코딩됩니다:

```rust
let hello = String::from("안녕하세요");
println!("길이: {}", hello.len());  // 15바이트 (문자 5개 아님!)
```

인덱싱이 지원되지 않는 이유:
```rust
let s = String::from("hello");
// let h = s[0];  // ❌ 에러! 인덱싱 불가
let h = &s[0..1];  // ✅ 슬라이스는 가능
```

## 문자 순회

```rust
// 문자 단위
for c in "안녕".chars() {
    println!("{}", c);
}

// 바이트 단위
for b in "안녕".bytes() {
    println!("{}", b);
}
```

## 연습 문제

- `strings1.rs` - String 생성
- `strings2.rs` - String vs &str
- `strings3.rs` - 문자열 조작
- `strings4.rs` - 슬라이스

## 참고 자료

- [The Rust Book - Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
