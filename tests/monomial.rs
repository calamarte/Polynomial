use rust_polynomial::Monomial;

#[test]
fn check_test() {}

#[test]
fn test_try_from_valid_input() {
    let tests = std::collections::HashMap::from([
        ("1", Monomial::new(1, 0)),
        (" 1 ", Monomial::new(1, 0)),
        ("- 1 ", Monomial::new(-1, 0)),
        ("+ 1 ", Monomial::new(1, 0)),
        ("2", Monomial::new(2, 0)),
        ("x", Monomial::new(1, 1)),
        ("-x", Monomial::new(-1, 1)),
        ("2x", Monomial::new(2, 1)),
        ("2X", Monomial::new(2, 1)),
        ("2x^2", Monomial::new(2, 2)),
        ("2x2", Monomial::new(2, 2)),
        ("-23x^2", Monomial::new(-23, 2)),
        ("-23x^-2", Monomial::new(-23, -2)),
        ("-23x-2", Monomial::new(-23, -2)),
    ]);

    for (str, mono) in tests {
        assert_eq!(Monomial::try_from(str).unwrap(), mono);
    }
}

#[test]
fn test_try_from_invalid_input() {
    let invalid = ["a", "2b", "2xabc", "flkjasdf"];

    for input in invalid {
        assert!(Monomial::<i32>::try_from(input).is_err());
    }
}
