#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(5, 3), 2);
    }
}
