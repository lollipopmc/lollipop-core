pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
