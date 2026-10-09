fn main() {
    let msg = "Hello, world!";
    println!("{msg}");
}

#[allow(dead_code)]
fn add(a: u16, b: u16) -> i32 {
    i32::from(a) + i32::from(b)
}

#[test]
fn test_add() {
    let x: u16 = 1;
    assert_eq!(add(x, 2), 3);
}
