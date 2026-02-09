# 00. Rust 소개

## 학습 목표

- Rust 프로그래밍 언어의 특징 이해
- `cargo` 기본 명령어 익히기
- 첫 번째 Rust 프로그램 작성하기

## Rust란?

Rust는 메모리 안전성, 동시성, 성능을 중점으로 하는 시스템 프로그래밍 언어입니다.
가비지 컬렉터 없이도 메모리 안전성을 보장하는 것이 가장 큰 특징입니다.

## Node.js 개발자를 위한 설명

| 특징 | Node.js | Rust |
|-----|---------|------|
| 타입 | 동적 타입 (TypeScript는 정적) | 정적 타입 |
| 메모리 관리 | 가비지 컬렉터 | 소유권 시스템 |
| 실행 | 인터프리터 (V8) | 네이티브 컴파일 |
| 패키지 관리 | npm, yarn | cargo |
| 비동기 | Promise, async/await | Future, async/await |

## Cargo 명령어

```bash
# 새 프로젝트 생성
cargo new my_project

# 빌드 (개발용)
cargo build

# 빌드 및 실행
cargo run

# 릴리스 빌드 (최적화)
cargo build --release

# 테스트 실행
cargo test

# 프로젝트 체크 (컴파일 없이 검사만)
cargo check
```

## 연습 문제

- `intro1.rs` - Hello, World!
- `intro2.rs` - 변수 출력하기

## 참고 자료

- [The Rust Book - Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
