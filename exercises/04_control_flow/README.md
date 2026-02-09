# 04. 제어 흐름 (Control Flow)

## 학습 목표

- if/else 조건문
- loop, while, for 반복문
- break와 continue
- loop에서 값 반환

## if 표현식

```rust
let number = 5;

if number < 10 {
    println!("작습니다");
} else if number == 10 {
    println!("같습니다");
} else {
    println!("큽니다");
}
```

if는 **표현식**이므로 값을 반환할 수 있습니다:

```rust
let number = if condition { 5 } else { 6 };
```

## Node.js 개발자를 위한 설명

JavaScript:
```javascript
if (number < 10) {  // 괄호 필요
    console.log("작습니다");
}

// 삼항 연산자
const number = condition ? 5 : 6;
```

Rust:
```rust
if number < 10 {    // 괄호 불필요
    println!("작습니다");
}

// if는 표현식
let number = if condition { 5 } else { 6 };
```

## 반복문

### loop - 무한 루프

```rust
loop {
    println!("계속!");
    break;  // break로 탈출
}

// 값 반환 가능
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // 20 반환
    }
};
```

### while - 조건부 반복

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

### for - 컬렉션 순회

```rust
let arr = [10, 20, 30, 40, 50];
for element in arr {
    println!("값: {}", element);
}

// 범위 (Range)
for number in 1..4 {  // 1, 2, 3 (4 제외)
    println!("{}", number);
}

for number in 1..=4 {  // 1, 2, 3, 4 (4 포함)
    println!("{}", number);
}
```

## JavaScript vs Rust

| JavaScript | Rust |
|-----------|------|
| `while (true) { }` | `loop { }` |
| `for (let i = 0; i < 5; i++) { }` | `for i in 0..5 { }` |
| `arr.forEach(x => ...)` | `for x in arr { }` |
| `break;` | `break;` (동일) |
| `continue;` | `continue;` (동일) |

## 연습 문제

- `control_flow1.rs` - if/else
- `control_flow2.rs` - loop
- `control_flow3.rs` - while
- `control_flow4.rs` - for

## 참고 자료

- [The Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
