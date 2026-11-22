use regex::Regex;
use std::sync::LazyLock;

#[allow(dead_code)]
fn check_email(text: &str) -> bool {
    const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(EMAIL_PATTERN).unwrap());
    RE.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_email() {
        assert!(check_email("user@example2.com"));
        assert!(!check_email("us@er@example.com"));
    }
}
