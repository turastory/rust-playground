# 06. 열거형 (Enums)

## 학습 목표

- 열거형 정의와 사용
- 열거형에 데이터 포함
- match 표현식
- if let 패턴

## 열거형 정의

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

## 데이터를 포함하는 열거형

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

## Node.js 개발자를 위한 설명

TypeScript Union Types와 비슷하지만 더 강력합니다:

TypeScript:
```typescript
// Union type
type IpAddr = 
    | { kind: 'V4', address: string }
    | { kind: 'V6', address: string };

const home: IpAddr = { 
    kind: 'V4', 
    address: '127.0.0.1' 
};

// 타입 가드 필요
if (home.kind === 'V4') {
    console.log(home.address);
}
```

Rust:
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

// match로 패턴 매칭
match home {
    IpAddr::V4(addr) => println!("IPv4: {}", addr),
    IpAddr::V6(addr) => println!("IPv6: {}", addr),
}
```

장점:
- 컴파일러가 모든 케이스를 처리했는지 확인
- 잘못된 케이스 접근 불가능
- 각 variant가 다른 타입의 데이터를 가질 수 있음

## match 표현식

모든 가능한 값을 처리해야 합니다:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

JavaScript switch와 비교:
```javascript
// JavaScript switch (break 필요)
function valueInCents(coin) {
    switch(coin) {
        case 'penny':
            return 1;
        case 'nickel':
            return 5;
        case 'dime':
            return 10;
        case 'quarter':
            return 25;
        default:  // 선택적
            return 0;
    }
}
```

Rust match:
- fall-through 없음
- 모든 케이스 처리 필수 (exhaustive)
- 표현식이므로 값 반환

## 값 추출

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));

match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Text: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
}
```

## if let 패턴

하나의 케이스만 처리하고 싶을 때:

```rust
let some_value = Some(3);

// match 사용
match some_value {
    Some(3) => println!("three"),
    _ => (),  // 나머지는 무시
}

// if let 사용 (더 간결)
if let Some(3) = some_value {
    println!("three");
}
```

## 연습 문제

- `enums1.rs` - 기본 열거형
- `enums2.rs` - 데이터를 포함하는 열거형
- `enums3.rs` - match 표현식
- `enums4.rs` - if let 패턴

## 참고 자료

- [The Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Match](https://doc.rust-lang.org/book/ch06-02-match.html)
