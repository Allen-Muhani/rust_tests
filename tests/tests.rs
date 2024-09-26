use rust_tests::bodmas::{add, sqrt};

#[test]
fn test_integration_add() {
    let result = add(5, 10);
    assert_eq!(result, 15);
}

#[test]
fn test_integration_sqrt() {
    let number = 9.0;
    match sqrt(number) {
        Ok(result) => assert_eq!(result, 3.0),
        Err(_) => panic!("Square root test failed"),
    }
}

#[test]
fn test_integration_failed_sqrt() {
    let number = -9.0;
    let rst = sqrt(number);

    assert!(rst.is_err(), "Expected an error for negative value.");
    assert_eq!(rst.err().unwrap(), "Negative floats do not have a square root.");
}
