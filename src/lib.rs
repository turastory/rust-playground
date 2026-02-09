//! # Rust Playground
//!
//! Node.js 개발자를 위한 Rust 학습 라이브러리
//!
//! 이 라이브러리는 연습 문제에서 사용할 수 있는 헬퍼 함수들을 제공합니다.

/// 헬퍼 모듈
pub mod helpers {
    /// 성공 메시지 출력
    pub fn print_success(message: &str) {
        println!("✅ 성공: {}", message);
    }

    /// 에러 메시지 출력
    pub fn print_error(message: &str) {
        eprintln!("❌ 에러: {}", message);
    }

    /// 힌트 출력
    pub fn print_hint(hint: &str) {
        println!("💡 힌트: {}", hint);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        // 헬퍼 함수들이 정상 작동하는지 확인
        helpers::print_success("테스트 통과!");
    }
}
