// tauri_state1.rs
//
// Tauri 상태 관리를 이해하는 연습입니다.

use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

// TODO: increment 커맨드를 작성하세요
// #[tauri::command]
// fn increment(state: tauri::State<AppState>) -> i32 {
//     ???
// }

fn main() {
    let state = AppState {
        counter: Mutex::new(0),
    };
    
    // counter 증가 예시
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    println!("Counter: {}", *counter);
}

// 힌트: state.counter.lock().unwrap()로 접근
