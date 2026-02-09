# Rust Playground 🦀

Node.js 개발자를 위한 Rust 학습 저장소입니다. Tauri 기반 데스크톱 앱 개발을 목표로 Rust의 핵심 개념을 단계별로 학습합니다. 특히 Node.js 개발자를 위한 비교 내용도 포함되어 있습니다.

## 📚 학습 목표

- Rust 기본 문법과 개념 마스터
- 소유권 시스템 이해
- Tauri를 사용한 데스크톱 앱 개발 준비

## 🚀 시작하기

### 1. Rust 설치

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 설치 확인
rustc --version
cargo --version
```

### 2. 저장소 클론

```bash
git clone <repository-url>
cd rust-playground
```

### 3. 첫 연습 문제 실행

```bash
# 예시: intro1 실행
cargo run --bin intro1

# 컴파일만 (실행 안 함)
cargo check
```

## 📖 학습 경로

### Phase 1: 기초 개념 (00-06)

기본 문법과 제어 흐름을 배웁니다.

- **00_intro** - Rust 소개 및 환경 설정
- **01_variables** - 변수와 가변성, 섀도잉
- **02_functions** - 함수, 표현식 vs 구문
- **03_primitive_types** - 기본 타입, 튜플, 배열
- **04_control_flow** - if, loop, while, for
- **05_structs** - 구조체, 메서드
- **06_enums** - 열거형, 패턴 매칭

**추천 학습 시간**: 1-2일

### Phase 2: 소유권 시스템 (07-10)

Rust의 가장 중요한 개념을 배웁니다.

- **07_move_semantics** - 이동 의미론
- **08_ownership** - 소유권 규칙
- **09_borrowing** - 빌림, 참조
- **10_strings** - String vs &str

**추천 학습 시간**: 2-3일

### Phase 3: 컬렉션 및 에러 처리 (11-14)

실용적인 데이터 구조와 에러 처리를 배웁니다.

- **11_vecs** - 벡터 (가변 배열)
- **12_hashmaps** - 해시맵
- **13_options** - Option<T>
- **14_error_handling** - Result<T, E>, ? 연산자

**추천 학습 시간**: 2일

### Phase 4: 고급 개념 (15-21)

프로덕션 코드를 위한 고급 기능을 배웁니다.

- **15_generics** - 제네릭 타입
- **16_traits** - 트레이트 (인터페이스)
- **17_lifetimes** - 라이프타임
- **18_iterators** - 반복자
- **19_closures** - 클로저
- **20_smart_pointers** - 스마트 포인터
- **21_async** - 비동기 프로그래밍

**추천 학습 시간**: 3-4일

### Phase 5: Tauri 특화 (22-24)

데스크톱 앱 개발을 위한 Tauri 기초를 배웁니다.

- **22_tauri_basics** - Tauri 프로젝트 구조
- **23_tauri_commands** - 커맨드와 IPC
- **24_tauri_state** - 상태 관리

**추천 학습 시간**: 1-2일

## 💡 학습 방법

### 1. 각 폴더의 README 읽기

```bash
# 예시
cat exercises/01_variables/README.md
```

각 폴더에는 개념 설명과 Node.js와의 비교가 포함되어 있습니다.

### 2. 연습 문제 풀기

각 `.rs` 파일에는 TODO 주석이 있습니다. 코드를 수정하여 컴파일되도록 만드세요.

```bash
# 예시: variables1 연습
cargo run --bin variables1
```

### 3. 막힐 때는 solutions 폴더 참고

```bash
# 정답 확인
cat solutions/01_variables/variables1.rs
```

### 4. 다음 단계로 진행

한 폴더의 모든 연습 문제를 완료하면 다음 폴더로 넘어가세요.

## 🎯 Node.js 개발자를 위한 팁

### 주요 차이점 비교

| 개념 | JavaScript/Node.js | Rust |
|-----|-------------------|------|
| 변수 선언 | `let`, `const` | `let`, `let mut` |
| 함수 | `function`, `=>` | `fn` |
| 문자열 | `string` | `String`, `&str` |
| 배열 | `Array` (동적) | `[T; N]` (고정), `Vec<T>` (동적) |
| 객체 | `Object`, `Map` | `struct`, `HashMap` |
| null/undefined | `null`, `undefined` | `Option<T>` |
| 에러 처리 | `try/catch` | `Result<T, E>` |
| 비동기 | `Promise`, `async/await` | `Future`, `async/await` |

### 자주 하는 실수

1. **mut 빼먹기**: 변수를 변경하려면 `let mut` 사용
2. **세미콜론**: 마지막 표현식에는 세미콜론 없어야 값 반환
3. **소유권**: 값을 이동한 후 다시 사용하려고 시도
4. **참조**: `&`를 붙이지 않고 함수에 전달

더 자세한 내용은 [GUIDE_NODEJS.md](GUIDE_NODEJS.md)를 참고하세요.

## 🛠️ 유용한 명령어

```bash
# 모든 코드 컴파일 체크 (빠름)
cargo check

# 특정 연습 문제 실행
cargo run --bin variables1

# 릴리스 빌드 (최적화)
cargo build --release

# 코드 포맷팅
cargo fmt

# Linter 실행
cargo clippy

# 문서 보기
rustup doc
```

## 📚 추가 학습 자료

### 공식 문서

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - 가장 권장하는 공식 문서
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 예제로 배우는 Rust
- [rustlings](https://github.com/rust-lang/rustlings) - 공식 연습 문제

### Tauri 관련

- [Tauri Documentation](https://tauri.app/)
- [Tauri Examples](https://github.com/tauri-apps/tauri/tree/dev/examples)

### 커뮤니티

- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

## 🤝 기여하기

버그를 발견하거나 개선 사항이 있다면 Issue나 Pull Request를 보내주세요!

## 📝 라이선스

MIT License

## 🙏 감사의 말

이 저장소는 다음 프로젝트들의 영향을 받았습니다:

- [rustlings](https://github.com/rust-lang/rustlings) - 구조와 접근 방식
- [The Rust Book](https://doc.rust-lang.org/book/) - 개념 설명
- [Tauri](https://tauri.app/) - 실용적인 사용 사례

---

Happy Coding! 🦀✨
