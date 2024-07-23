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
        ("3x + 5 + 31x^2 - 7", "31x^2 + 3x - 2"),
        ("0", "0"),
        ("3 - 3", "0"),
        ("3x - 3x", "0"),
        ("0x^2 + 6x + 0", "6x"),
        (
            "-81x + 9x^9 - 6x^5 - x^90 - 6 + 8x^2",
            "-x^90 + 9x^9 - 6x^5 + 8x^2 - 81x - 6",
        ),
        (
            "31x^7 + 3x^7 - 5x - 12x^2 - 15x - 2 - 9 + 20 + 2x^2",
            "34x^7 - 10x^2 - 20x + 9",
        ),
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
            Polynomial::<i32>::try_from("x").unwrap() * Polynomial::<i32>::try_from("x").unwrap()
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

#[test]
fn div_op() {
    #[rustfmt::skip]
    let to_check = HashMap::from([
        (("x4 - 6x2 + 8", "x-1"), ("x^3 + x^2 - 5x - 5", "3")),
        (("4x^2 + 32x + 64", "2x + 8"), ("2x + 8", "0")),
        (("-5x^7 + 42x^3 - 9", "x^2 + 1"),("-5x^5 + 5x^3 + 37x", "-37x - 9")),
    ])
    .into_iter()
    .map(|((str1, str2), (result, rest))| {
        (
            (
                Polynomial::<i32>::try_from(str1).unwrap(),
                Polynomial::<i32>::try_from(str2).unwrap(),
            ),
            (
                Polynomial::<i32>::try_from(result).unwrap(),
                Polynomial::<i32>::try_from(rest).unwrap(),
            ),
        )
    });

    for ((p1, p2), (expect_result, expect_rest)) in to_check {
        let (result, rest) = p1 / p2;

        assert_eq!(result, expect_result);
        assert_eq!(rest, expect_rest);
    }
}

#[test]
fn roots_op() {
    #[rustfmt::skip]
    let to_check = HashMap::from([
        ("x - 9", Some(vec![9])),
        ("-x^2 + 4", Some(vec![-2, 2])),
        ("2x^2 + 4x - 30", Some(vec![-5, 3])),
        ("23x^2 + 90x + 100", None),
        ("x^2 + 81 + 18x", Some(vec![-9])),
        ("x^4 - 13x^2 + 36", Some(vec![-3, -2, 2, 3])),
        ("x^3 - 5x^2 - x + 5", Some(vec![-1, 1, 5])),
        ("x^4 + 12x^3 + 11x^2 - 132x + 108", Some(vec![-9, -6, 1, 2])),
        ("x^12 + 1", None)
    ]);

    for (p_str, expect) in to_check {
        let poly = Polynomial::<i32>::try_from(p_str).unwrap();
        assert_eq!(poly.roots(), expect);
    }
}

#[test]
fn roots_op_float() {
    #[rustfmt::skip]
    let to_check = HashMap::from([
        ("x^4 - 10x^2 + 25", Some(vec![-2.236068, 2.236068])),
        ("x^4 - 100", Some(vec![-3.1622777, 3.1622777])),
        ("x^3 - 100", Some(vec![4.6415887])),
        ("x^8 - 100", Some(vec![-1.7782794, 1.7782794])),
        ("x^13 + 150", Some(vec![-1.4702516]))
    ]);

    for (p_str, expect) in to_check {
        let poly = Polynomial::<f32>::try_from(p_str).unwrap();
        assert_eq!(poly.roots(), expect);
    }
}
