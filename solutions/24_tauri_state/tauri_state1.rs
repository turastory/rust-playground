// tauri_state1.rs - 정답

use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

// 실제 Tauri 프로젝트에서 사용:
// #[tauri::command]
// fn increment(state: tauri::State<AppState>) -> i32 {
//     let mut counter = state.counter.lock().unwrap();
//     *counter += 1;
//     *counter
// }

fn main() {
    let state = AppState {
        counter: Mutex::new(0),
    };
    
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    println!("Counter: {}", *counter);
}
