use adder::prints_and_returns_10;

#[test]
fn prints_and_returns_10() {
    let value = prints_and_returns_10(4);
    assert_eq!(value, 10);
}