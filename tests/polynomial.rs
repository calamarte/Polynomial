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

    assert_eq!(
        Polynomial::try_from("0").unwrap(),
        Polynomial::try_from(vec![0]).unwrap()
    );
}

#[test]
fn construct_by_str() {
    let to_check = HashMap::from([
        ("x + 5", "x + 5"),
        ("3x + 5 + 2x^2", "2x^2 + 3x + 5"),
        (
            "-81x + 9x^9 - 6x^5 - x^90 - 6 + 8x^2",
            "-x^90 + 9x^9 - 6x^5 + 8x^2 - 81x - 6",
        ),
        ("3x + 5 + 31x^2 - 7", "31x^2 + 3x - 2"),
        (
            "31x^7 + 3x^7 - 5x - 12x^2 - 15x - 2 - 9 + 20 + 2x^2",
            "34x^7 - 10x^2 - 20x + 9",
        ),
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

#[test]
fn add_op() {
    assert_eq!(
        "9x^2 + 3x + 5",
        format!(
            "{}",
            Polynomial::<i32>::try_from("2x^2 + 3x - 5").unwrap()
                + Polynomial::<i32>::try_from("7x^2 + 10").unwrap()
        )
    );

    assert_eq!(
        "73x^8 + 3x^4 - x^2 + 10",
        format!(
            "{}",
            Polynomial::<i32>::try_from("73x^8 + 3x^4").unwrap()
                + Polynomial::<i32>::try_from("-x^2 + 10").unwrap()
        )
    );

    assert_eq!(
        "3x^7 - x^2 + 15x",
        format!(
            "{}",
            Polynomial::<i32>::try_from("-2x").unwrap()
                + Polynomial::<i32>::try_from("8x^7 + 5x").unwrap()
                + Polynomial::<i32>::try_from("-x^2 + 12x - 5x^7").unwrap()
        )
    );

    assert_eq!(
        Polynomial::<i32>::try_from("2x + 18").unwrap(),
        Polynomial::<i32>::try_from("5x^2 + 2x + 10").unwrap()
            + Polynomial::<i32>::try_from("8 - 7x^2 + 2x^2").unwrap()
    );
}

#[test]
fn mul_op() {
    assert_eq!(
        "-6x^10 + 36x^8 - 48x^6 - 91x^5 + 12x^4 + 546x^3 - 72x^2 - 728x + 96",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x^4 - 6x^2 + 8").unwrap()
                * Polynomial::<i32>::try_from("-6x^6 - 91x + 12").unwrap()
        )
    );
    
    assert_eq!(
        "x^2",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x").unwrap()
                * Polynomial::<i32>::try_from("x").unwrap()
        )
    );
    
    assert_eq!(
        "0",
        format!(
            "{}",
            Polynomial::<i32>::try_from("0").unwrap()
                * Polynomial::<i32>::try_from("x^4 - 7x + 9").unwrap()
        )
    );
    
    assert_eq!(
        "x^2 - 1",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x + 1").unwrap()
                * Polynomial::<i32>::try_from("x - 1").unwrap()
        )
    );

    assert_eq!(
        "x^2 + 5x + 6",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x + 3").unwrap()
                * Polynomial::<i32>::try_from("x + 2").unwrap()
        )
    );

    assert_eq!(
        "4x^2 + 32x + 64",
        format!(
            "{}",
            Polynomial::<i32>::try_from("2x + 8").unwrap()
                * Polynomial::<i32>::try_from("2x + 8").unwrap()
        )
    );

    assert_eq!(
        "x^2 + x",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x").unwrap()
                * Polynomial::<i32>::try_from("x + 1").unwrap()
        )
    );
    
    assert_eq!(
        "x^2 + 2x + 1",
        format!(
            "{}",
            Polynomial::<i32>::try_from("x + 1").unwrap()
                * Polynomial::<i32>::try_from("x + 1").unwrap()
        )
    );
}
