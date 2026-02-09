// tauri_commands1.rs
//
// Tauri 커맨드를 이해하는 연습입니다.

// TODO: greet 커맨드를 작성하세요
// #[tauri::command]
// fn greet(name: &str) -> String {
//     ???
// }

fn main() {
    println!("Tauri 커맨드 예시:");
    println!("#[tauri::command]");
    println!("fn greet(name: &str) -> String {{");
    println!("    format!(\"Hello, {{}}!\", name)");
    println!("}}");
}

// 힌트: format!("Hello, {}!", name)
