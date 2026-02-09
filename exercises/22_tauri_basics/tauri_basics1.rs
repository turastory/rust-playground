// tauri_basics1.rs
//
// Tauri 기본 구조를 이해하는 연습입니다.
// 
// 참고: 이 파일은 실제 Tauri 프로젝트에서 사용할 수 없습니다.
// Tauri 프로젝트를 만들려면: cargo install tauri-cli && cargo tauri init

// 기본 Tauri 앱 구조:
// 
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

fn main() {
    println!("Tauri 프로젝트를 만들려면:");
    println!("1. cargo install tauri-cli");
    println!("2. cargo tauri init");
    println!("3. cargo tauri dev");
}
