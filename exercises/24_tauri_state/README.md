# 24. Tauri 상태 관리

## 학습 목표

- 전역 상태 관리
- State<T> 사용

## 상태 관리

앱 전체에서 공유되는 데이터:

```rust
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

#[tauri::command]
fn increment(state: tauri::State<AppState>) -> i32 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}

// 등록:
tauri::Builder::default()
    .manage(AppState {
        counter: Mutex::new(0),
    })
    .invoke_handler(tauri::generate_handler![increment])
```

## Node.js/Express와 비교

Express (전역 변수):
```javascript
let counter = 0

app.get('/increment', (req, res) => {
    counter += 1
    res.json({ counter })
})
```

Tauri:
- 타입 안전
- 스레드 안전 (Mutex)
- 더 나은 구조

## 참고 자료

- [Tauri State Management](https://tauri.app/v1/guides/features/command#accessing-managed-state)
