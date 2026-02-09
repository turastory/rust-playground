// tauri_commands1.rs - 정답

// 실제 Tauri 프로젝트에서 사용:
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

fn main() {
    // 커맨드 시뮬레이션
    let name = "World";
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);
}
