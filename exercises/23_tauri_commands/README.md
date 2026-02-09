# 23. Tauri 커맨드

## 학습 목표

- Tauri 커맨드 정의
- 프론트엔드에서 Rust 함수 호출 (IPC)

## 커맨드

프론트엔드에서 호출할 수 있는 Rust 함수:

```rust
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// 등록:
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
```

프론트엔드 (JavaScript):
```javascript
import { invoke } from '@tauri-apps/api/tauri'

const greeting = await invoke('greet', { name: 'World' })
console.log(greeting)  // "Hello, World!"
```

## Electron과 비교

Electron (ipcMain/ipcRenderer):
```javascript
// Main process
ipcMain.handle('greet', async (event, name) => {
    return `Hello, ${name}!`
})

// Renderer process
const greeting = await ipcRenderer.invoke('greet', 'World')
```

Tauri:
- 더 타입 안전함
- 직렬화/역직렬화 자동
- 더 나은 성능

## 참고 자료

- [Tauri Commands](https://tauri.app/v1/guides/features/command)
