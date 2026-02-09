# Node.js 개발자를 위한 Rust 가이드 🚀

Node.js/JavaScript/TypeScript 배경을 가진 개발자가 Rust를 배우는 데 도움이 되는 종합 가이드입니다.

## 목차

- [언어 철학 비교](#언어-철학-비교)
- [문법 비교표](#문법-비교표)
- [메모리 관리: GC vs 소유권](#메모리-관리-gc-vs-소유권)
- [타입 시스템](#타입-시스템)
- [비동기 프로그래밍](#비동기-프로그래밍)
- [Tauri vs Electron](#tauri-vs-electron)
- [자주 하는 실수와 해결 방법](#자주-하는-실수와-해결-방법)

---

## 언어 철학 비교

### JavaScript/Node.js

- **동적 타입**: 런타임에 타입 결정
- **프로토타입 기반**: 객체 지향
- **가비지 컬렉션**: 자동 메모리 관리
- **단일 스레드** (이벤트 루프): 비동기 I/O
- **유연성 중시**: "작동하면 OK"

### Rust

- **정적 타입**: 컴파일 타임에 타입 결정
- **시스템 프로그래밍**: 하드웨어 직접 제어
- **소유권 시스템**: 명시적 메모리 관리
- **멀티 스레드**: 데이터 레이스 방지
- **안전성 중시**: "컴파일되면 안전함"

### 왜 Rust를 배워야 할까?

| 상황 | JavaScript | Rust |
|-----|-----------|------|
| 웹 백엔드 | ✅ 빠른 개발 | ✅ 높은 성능 |
| 프론트엔드 | ✅ 생태계 풍부 | ⚠️ WebAssembly |
| CLI 도구 | ⚠️ 느린 시작 | ✅ 빠른 실행 |
| 데스크톱 앱 | ✅ Electron | ✅ Tauri (작고 빠름) |
| 시스템 프로그래밍 | ❌ 불가능 | ✅ 완벽 |
| 메모리 안전성 | ⚠️ GC 의존 | ✅ 보장됨 |

---

## 문법 비교표

### 변수와 상수

```javascript
// JavaScript
let x = 5;        // 재할당 가능
const y = 10;     // 재할당 불가

x = 6;            // ✅ OK
y = 11;           // ❌ 에러
```

```rust
// Rust
let x = 5;        // 불변 (기본)
let mut y = 10;   // 가변 (mut 키워드)

x = 6;            // ❌ 에러
y = 11;           // ✅ OK
```

**핵심 차이**: JavaScript의 `let`은 가변이지만, Rust의 `let`은 불변입니다!

### 함수

```javascript
// JavaScript
function add(a, b) {
    return a + b;
}

// 화살표 함수
const add = (a, b) => a + b;

// TypeScript
function add(a: number, b: number): number {
    return a + b;
}
```

```rust
// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // return 키워드 없이 반환 (표현식)
}

// 명시적 return도 가능
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;
}
```

**핵심 차이**: 
- Rust는 타입 명시 필수
- 마지막 표현식이 자동으로 반환값 (세미콜론 없을 때)
- 화살표 함수 같은 간결한 문법 없음 (클로저는 있음)

### 배열과 벡터

```javascript
// JavaScript - 동적 배열
const arr = [1, 2, 3];
arr.push(4);               // 크기 변경 가능
arr[0] = 10;               // 수정 가능
console.log(arr.length);   // 4
```

```rust
// Rust - 고정 크기 배열
let arr = [1, 2, 3];
// arr.push(4);            // ❌ 불가능! 고정 크기
// arr[0] = 10;            // ❌ 불가능! 불변

// 가변 벡터 (동적 배열)
let mut vec = vec![1, 2, 3];
vec.push(4);               // ✅ OK
vec[0] = 10;               // ✅ OK
println!("{}", vec.len()); // 4
```

**핵심 차이**: Rust는 배열(고정)과 벡터(동적)를 명확히 구분합니다.

### 객체와 구조체

```javascript
// JavaScript
const user = {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
};

user.age = 31;  // 수정 가능
user.newField = "value";  // 필드 추가 가능
```

```typescript
// TypeScript
interface User {
    name: string;
    age: number;
    email: string;
}

const user: User = {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
};
```

```rust
// Rust
struct User {
    name: String,
    age: u32,
    email: String,
}

let mut user = User {
    name: String::from("Alice"),
    age: 30,
    email: String::from("alice@example.com"),
};

user.age = 31;  // ✅ OK (mut 있으므로)
// user.new_field = "value";  // ❌ 불가능! 정의되지 않은 필드
```

**핵심 차이**:
- Rust 구조체는 컴파일 타임에 필드가 고정됨
- 동적으로 필드 추가/삭제 불가능
- TypeScript와 유사하지만 더 엄격함

### null/undefined vs Option

```javascript
// JavaScript
let value = null;  // 또는 undefined

if (value !== null && value !== undefined) {
    console.log(value);
} else {
    console.log("No value");
}

// Optional chaining (ES2020)
const street = user?.address?.street;
```

```rust
// Rust - Option<T>
let value: Option<i32> = None;

match value {
    Some(v) => println!("{}", v),
    None => println!("No value"),
}

// if let
if let Some(v) = value {
    println!("{}", v);
}
```

**핵심 차이**: Rust는 null이 없습니다! `Option<T>`로 명시적으로 표현합니다.

### try/catch vs Result

```javascript
// JavaScript
function divide(a, b) {
    if (b === 0) {
        throw new Error("Division by zero");
    }
    return a / b;
}

try {
    const result = divide(10, 0);
    console.log(result);
} catch (error) {
    console.error(error.message);
}
```

```rust
// Rust - Result<T, E>
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 0) {
    Ok(result) => println!("{}", result),
    Err(e) => eprintln!("{}", e),
}

// ? 연산자 (try/catch와 유사)
fn do_something() -> Result<i32, String> {
    let result = divide(10, 2)?;  // 에러 시 즉시 반환
    Ok(result * 2)
}
```

**핵심 차이**: Rust는 에러를 값으로 처리합니다 (예외 던지기 없음).

---

## 메모리 관리: GC vs 소유권

### JavaScript/Node.js (가비지 컬렉션)

```javascript
function process() {
    const data = { value: 100 };
    // 함수 종료 후에도 data는 메모리에 남아있음
    // GC가 나중에 알아서 정리
}

// 메모리 누수 예시
let cache = [];
setInterval(() => {
    cache.push(new Array(1000000));  // 메모리 계속 증가
}, 100);
```

**장점**:
- 편리함
- 메모리 관리 신경 안 써도 됨

**단점**:
- GC 일시 정지 (Stop-the-world)
- 메모리 사용량 예측 어려움
- 메모리 누수 가능

### Rust (소유권 시스템)

```rust
fn process() {
    let data = String::from("hello");
    // 함수 종료 시 즉시 data의 메모리 해제
}  // <- 여기서 drop 호출

// 소유권 규칙으로 메모리 누수 방지
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1의 소유권이 s2로 이동
    
    // println!("{}", s1);  // ❌ 컴파일 에러! s1은 더 이상 유효하지 않음
    println!("{}", s2);  // ✅ OK
}
```

**장점**:
- 예측 가능한 메모리 해제
- 런타임 오버헤드 없음
- 메모리 누수 방지

**단점**:
- 학습 곡선 가파름
- 작성 시 더 많은 고민 필요

### 소유권 규칙 3가지

1. **각 값은 하나의 소유자가 있다**
2. **한 번에 하나의 소유자만 존재한다**
3. **소유자가 스코프를 벗어나면 값은 drop된다**

```rust
// 1. 소유권 이동
let s1 = String::from("hello");
let s2 = s1;  // s1 -> s2로 이동
// s1은 더 이상 사용 불가

// 2. 복제 (Clone)
let s1 = String::from("hello");
let s2 = s1.clone();  // 깊은 복사
// s1, s2 모두 사용 가능

// 3. 참조 (Borrow)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // 빌림 (소유권 유지)
println!("{} {}", s1, len);  // s1 여전히 사용 가능

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

## 타입 시스템

### JavaScript vs TypeScript vs Rust

```javascript
// JavaScript - 동적 타입
let x = 5;
x = "hello";  // ✅ OK - 타입 변경 가능
```

```typescript
// TypeScript - 정적 타입 (컴파일 타임)
let x: number = 5;
x = "hello";  // ❌ 타입 에러 (IDE에서)
// 하지만 런타임에는 JavaScript로 변환되어 타입 체크 없음
```

```rust
// Rust - 정적 타입 (컴파일 타임 + 런타임)
let x: i32 = 5;
// x = "hello";  // ❌ 컴파일 에러!
```

### 제네릭 비교

```typescript
// TypeScript
function identity<T>(value: T): T {
    return value;
}

const num = identity(5);
const str = identity("hello");
```

```rust
// Rust
fn identity<T>(value: T) -> T {
    value
}

let num = identity(5);
let str = identity("hello");
```

**유사점**: 문법이 거의 동일합니다!

### 인터페이스 vs 트레이트

```typescript
// TypeScript
interface Greet {
    greet(): string;
}

class Person implements Greet {
    greet(): string {
        return "Hello!";
    }
}
```

```rust
// Rust
trait Greet {
    fn greet(&self) -> String;
}

struct Person;

impl Greet for Person {
    fn greet(&self) -> String {
        String::from("Hello!")
    }
}
```

**차이점**: 
- Rust는 클래스가 없음 (구조체 + impl)
- Rust 트레이트는 더 강력함 (제약 조건, 연관 타입 등)

---

## 비동기 프로그래밍

### Promise vs Future

```javascript
// JavaScript Promise
async function fetchData() {
    const response = await fetch('https://api.example.com/data');
    const data = await response.json();
    return data;
}

fetchData().then(data => {
    console.log(data);
}).catch(error => {
    console.error(error);
});
```

```rust
// Rust Future (tokio)
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let data = response.text().await?;
    Ok(data)
}

#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("{}", e),
    }
}
```

### 주요 차이점

| JavaScript | Rust |
|-----------|------|
| Promise는 즉시 실행 | Future는 lazy (await해야 실행) |
| 단일 런타임 (이벤트 루프) | 명시적 런타임 (tokio, async-std) |
| `.then()`, `.catch()` | `match`, `?` 연산자 |
| `async/await` 문법 | `async/await` 문법 (유사) |

---

## Tauri vs Electron

### 아키텍처 비교

**Electron**:
```
┌─────────────────────┐
│   Chromium (웹뷰)   │
├─────────────────────┤
│     Node.js         │
│  (JavaScript 런타임) │
└─────────────────────┘
```

**Tauri**:
```
┌─────────────────────┐
│  시스템 WebView      │
│  (OS 내장)          │
├─────────────────────┤
│      Rust           │
│   (네이티브 코드)    │
└─────────────────────┘
```

### 성능 비교

| 특징 | Electron | Tauri |
|-----|---------|-------|
| 번들 크기 | ~120MB | ~3MB |
| 메모리 사용 | ~100MB | ~30MB |
| 시작 시간 | 느림 | 빠름 |
| CPU 사용률 | 높음 | 낮음 |
| 크로스 플랫폼 | ✅ | ✅ |

### IPC 비교

**Electron (ipcMain/ipcRenderer)**:
```javascript
// Main Process
ipcMain.handle('get-data', async (event, arg) => {
    return { result: 'data' };
});

// Renderer Process
const data = await ipcRenderer.invoke('get-data', 'arg');
```

**Tauri (Commands)**:
```rust
// Rust 백엔드
#[tauri::command]
fn get_data(arg: String) -> Result<String, String> {
    Ok(String::from("data"))
}

// 등록
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_data])
```

```javascript
// 프론트엔드
import { invoke } from '@tauri-apps/api/tauri';

const data = await invoke('get_data', { arg: 'value' });
```

**Tauri의 장점**:
- 타입 안전성
- 자동 직렬화/역직렬화
- 더 나은 성능

---

## 자주 하는 실수와 해결 방법

### 1. mut 키워드 빼먹기

```rust
// ❌ 에러
let x = 5;
x = 6;  // 에러: cannot assign twice to immutable variable

// ✅ 해결
let mut x = 5;
x = 6;  // OK
```

### 2. 소유권 이동 후 재사용

```rust
// ❌ 에러
let s = String::from("hello");
let s2 = s;  // s의 소유권이 s2로 이동
println!("{}", s);  // 에러: value borrowed after move

// ✅ 해결 1: Clone
let s = String::from("hello");
let s2 = s.clone();
println!("{} {}", s, s2);  // OK

// ✅ 해결 2: 참조 사용
let s = String::from("hello");
let s2 = &s;  // 빌림
println!("{} {}", s, s2);  // OK
```

### 3. 세미콜론 위치 실수

```rust
// ❌ 에러
fn add(a: i32, b: i32) -> i32 {
    a + b;  // 세미콜론 때문에 값이 반환되지 않음!
}

// ✅ 해결
fn add(a: i32, b: i32) -> i32 {
    a + b  // 세미콜론 없음 - 표현식
}
```

### 4. String vs &str 혼동

```rust
// ❌ 에러
fn greet(name: &str) -> String {
    "Hello, " + name  // 에러: + 연산자 사용 불가
}

// ✅ 해결 1: format! 매크로
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}

// ✅ 해결 2: String::from + push_str
fn greet(name: &str) -> String {
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    greeting
}
```

### 5. 빌림 규칙 위반

```rust
// ❌ 에러
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // 에러: 불변 참조와 가변 참조 동시 존재
println!("{} {}", r1, r2);

// ✅ 해결: 참조 스코프 분리
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);  // r1 사용 완료
let r2 = &mut s;     // 이제 OK
r2.push_str(" world");
```

---

## 학습 로드맵

### 1주차: 기초 다지기

- Phase 1 (00-06) 완료
- 소유권 개념 이해 시작

### 2주차: 소유권 마스터

- Phase 2 (07-10) 완료
- 빌림 규칙 완전 이해

### 3주차: 실용적인 기능

- Phase 3 (11-14) 완료
- 작은 CLI 프로젝트 시작

### 4주차: 고급 개념

- Phase 4 (15-21) 완료
- 트레이트와 제네릭 활용

### 5주차: Tauri 앱 개발

- Phase 5 (22-24) 완료
- 첫 Tauri 앱 만들기

---

## 추가 리소스

### 책

- [The Rust Programming Language](https://doc.rust-lang.org/book/) - 필독서
- [Rust for Rustaceans](https://rust-for-rustaceans.com/) - 중급자용

### 동영상

- [Rust Crash Course](https://www.youtube.com/watch?v=zF34dRivLOw) - Traversy Media
- [Crust of Rust](https://www.youtube.com/playlist?list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa) - Jon Gjengset

### 실습

- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

**질문이 있으시면 언제든지 물어보세요! Happy Coding! 🦀✨**
