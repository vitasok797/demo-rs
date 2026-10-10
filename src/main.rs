use regex::Regex;
use std::sync::LazyLock;

const EMAIL_PATTERN: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
static EMAIL_CHECK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(EMAIL_PATTERN).unwrap());

fn main() {
    println!("Hello, world!");
    test_regex();
}

fn test_regex() {
    check_email("user@example.com");
    check_email("us@er@example.com");

    // let email_to_check = String::from("user@example2.com");
    // let email_to_check: String = "user@example2.com".into();
    // let email_to_check = "user@example2.com".to_string();
    let email_to_check = "user@example2.com".to_owned();

    check_email(&email_to_check);
}

fn check_email(text_to_check: &str) {
    if EMAIL_CHECK_RE.is_match(text_to_check) {
        println!("{text_to_check} — valid email");
    } else {
        println!("{text_to_check} — INVALID email");
    }
}

#[allow(dead_code)]
fn add(a: u16, b: u16) -> i32 {
    i32::from(a) + i32::from(b)
}

#[test]
fn test_add() {
    // let x: u8 = 1;
    let x: u16 = 1;
    assert_eq!(add(x, 2), 3);
}
