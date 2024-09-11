use trust::{add, sub, mul, div};


#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn test_sub() {
    assert_eq!(sub(2, 1), 1);
    assert_eq!(sub(0, 0), 0);
    assert_eq!(sub(1, -1), 2);
}

#[test]
fn test_mul() {
    assert_eq!(mul(2, 3), 6);
    assert_eq!(mul(0, 5), 0);
    assert_eq!(mul(-2, 3), -6);
}

#[test]
fn test_div() {
    assert_eq!(div(6, 3), 2);
    assert_eq!(div(5, 2), 2); // Assuming integer division
    assert_eq!(div(-6, 3), -2);
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_div_by_zero() {
    div(1, 0); // This should panic
}
