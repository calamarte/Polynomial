use std::collections::HashMap;

use rust_polynomial::Polynomial;

#[test]
fn check_test() {}

#[test]
fn construct_by_vec() {
    let to_check = HashMap::from([
        (vec![1, 5], "x + 5"),
        (vec![0], "0"),
        (Vec::<i32>::new(), "0"),
        (vec![0, 0, 0], "0"),
        (vec![-6, 0, 0, 20, -8], "-6x^4 + 20x - 8"),
    ]);

    to_check.into_iter().for_each(|(v, expect)| {
        let result = format!("{}", Polynomial::try_from(v).unwrap());
        assert_eq!(result, expect);
    });

    assert_eq!(Polynomial::try_from("0").unwrap(), Polynomial::try_from(vec![0]).unwrap());
}


#[test]
fn construct_by_str() {
    let to_check = HashMap::from([
        ("x + 5", "x + 5"),
        ("3x + 5 + 2x^2", "2x^2 + 3x + 5"),
        ("-81x + 9x^9 - 6x^5 - x^90 - 6 + 8x^2", "-x^90 + 9x^9 - 6x^5 + 8x^2 - 81x - 6"),
        ("3x + 5 + 31x^2 - 7", "31x^2 + 3x - 2"),
        ("31x^7 + 3x^7 - 5x - 12x^2 - 15x - 2 - 9 + 20 + 2x^2", "34x^7 - 10x^2 - 20x + 9"),
        ("0", "0"),
        ("3 - 3", "0"),
        ("3x - 3x", "0"),
        ("0x^2 + 6x + 0", "6x"),
    ]);

    to_check.into_iter().for_each(|(v, expect)| {
        let result = format!("{}", Polynomial::<i32>::try_from(v).unwrap());
        assert_eq!(result, expect);
    });

}