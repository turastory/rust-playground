# 05. 구조체 (Structs)

## 학습 목표

- 구조체 정의와 인스턴스 생성
- 메서드(method)와 연관 함수(associated function)
- 구조체 업데이트 문법
- 튜플 구조체

## 구조체 정의

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

## 인스턴스 생성

```rust
let user1 = User {
    email: String::from("user@example.com"),
    username: String::from("user123"),
    age: 30,
    active: true,
};

// 필드 접근
println!("{}", user1.email);
```

## Node.js 개발자를 위한 설명

JavaScript/TypeScript:
```javascript
// JavaScript 객체
const user = {
    username: "user123",
    email: "user@example.com",
    age: 30,
    active: true
};

// TypeScript 인터페이스
interface User {
    username: string;
    email: string;
    age: number;
    active: boolean;
}
```

Rust:
```rust
// 구조체 정의 (타입)
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// 인스턴스 생성
let user = User {
    username: String::from("user123"),
    email: String::from("user@example.com"),
    age: 30,
    active: true,
};
```

주요 차이점:
- Rust 구조체는 컴파일 타임에 타입이 정해짐
- JavaScript 객체처럼 동적으로 필드 추가/삭제 불가능
- 메서드는 `impl` 블록에 정의

## 메서드

```rust
impl User {
    // 메서드 (인스턴스 필요)
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    // 연관 함수 (정적 메서드)
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            age: 0,
            active: true,
        }
    }
}

// 사용
let user = User::new(
    String::from("user123"),
    String::from("user@example.com")
);
println!("성인? {}", user.is_adult());
```

JavaScript 비교:
```javascript
class User {
    constructor(username, email) {
        this.username = username;
        this.email = email;
        this.age = 0;
        this.active = true;
    }
    
    isAdult() {
        return this.age >= 18;
    }
    
    static new(username, email) {
        return new User(username, email);
    }
}
```

## 구조체 업데이트 문법

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 나머지는 user1에서 복사
};
```

JavaScript 스프레드와 유사:
```javascript
const user2 = {
    ...user1,
    email: "another@example.com"
};
```

## 튜플 구조체

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 연습 문제

- `structs1.rs` - 기본 구조체 정의
- `structs2.rs` - 인스턴스 생성
- `structs3.rs` - 메서드 정의
- `structs4.rs` - 연관 함수

## 참고 자료

- [The Rust Book - Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
