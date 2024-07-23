#![allow(warnings)]

use num::Integer;
use rust_polynomial::{Monomial, Polynomial};

fn main() {
    let mono_vec: Vec<Monomial<i8>> = vec![
        Monomial::try_from("-2x").unwrap(),
        Monomial::try_from("1").unwrap(),
    ];

    // ("x^8 - 100", Some(vec![-1.778, 1.778])),
    // ("x^13 + 150", Some(vec![-1.470]))
    let poly: Polynomial<f32> = Polynomial::try_from("x^8 - 100").unwrap();
    let poly2: Polynomial<f32> = Polynomial::try_from("x^13 + 150").unwrap();
    // let poly2: Polynomial<i32> = Polynomial::try_from("x^4 - 10x^2 + 25").unwrap();
    // let poly_2: Polynomial<i32> = Polynomial::try_from(vec![1,0,0,0, 1, 1]).unwrap();

    // (("x4 - 6x2 + 8", "x-1"), ("x^3 + x^2 - 5x - 5", "3")), TODO check try_from
    // let poly: Polynomial<i32> = Polynomial::try_from("x^3 + x^2 - 5x - 5").unwrap();

    // for mono in &poly {
    //     println!("{mono}");
    // }
    //

    // println!("{:?}", poly);
    println!("{:?}", poly.roots());
    println!("{:?}", poly2.roots());
}
