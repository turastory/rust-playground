// variables3.rs
//
// 변수 섀도잉(shadowing)을 사용하면 같은 이름으로 새로운 변수를 만들 수 있습니다.
// 이는 타입 변환에 유용합니다.
//
// 실행: cargo run --bin variables3

fn main() {
    let spaces = "   ";  // 문자열
    println!("spaces = '{}'", spaces);

    // TODO: spaces를 섀도잉하여 문자열의 길이를 저장하세요
    // 힌트: spaces.len() 메서드를 사용하세요
    let spaces = spaces.len();
    
    println!("spaces의 길이 = {}", spaces);
}

// 힌트: let spaces = spaces.len();
