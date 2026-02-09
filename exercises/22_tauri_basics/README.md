# 22. Tauri 기초

## 학습 목표

- Tauri 프로젝트 구조 이해
- 프론트엔드와 백엔드 연결

## Tauri란?

Tauri는 Rust와 웹 기술로 데스크톱 앱을 만드는 프레임워크입니다.

## Electron vs Tauri

| 특징 | Electron | Tauri |
|-----|---------|------|
| 백엔드 | Node.js | Rust |
| 프론트엔드 | Chromium | 시스템 WebView |
| 번들 크기 | ~120MB | ~3MB |
| 메모리 사용 | 높음 | 낮음 |
| 보안 | 좋음 | 매우 좋음 |

## 프로젝트 구조

```
tauri-app/
├── src/               # 웹 프론트엔드 (React, Vue 등)
├── src-tauri/
│   ├── src/
│   │   └── main.rs    # Rust 백엔드
│   ├── Cargo.toml
│   └── tauri.conf.json
```

## 참고 자료

- [Tauri Documentation](https://tauri.app/)
- [Tauri Guide](https://tauri.app/v1/guides/)
