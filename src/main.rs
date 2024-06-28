#![allow(warnings)]

use rust_polynomial::{Monomial, Polynomial};

fn main() {
    let mono_vec: Vec<Monomial<i8>> = vec![
        Monomial::try_from("-x^2").unwrap(),
        Monomial::try_from("-2x").unwrap(),
        Monomial::try_from("1").unwrap(),
    ];

    // let poly: Polynomial<i32> = Polynomial::try_from("-2x^2 + 2x^3 - 10 - 1").unwrap();
    let poly: Polynomial<i32> = Polynomial::try_from("-2x^2 + 2x^3 - 10 - 1").unwrap();
    let poly2: Polynomial<i32> = Polynomial::try_from("-2x^2 + 2x^3 - 10 - 1").unwrap();
    // let poly_2: Polynomial<i32> = Polynomial::try_from(vec![1,0,0,0, 1, 1]).unwrap();

    // for mono in &poly {
    //     println!("{mono}");
    // }
    //
    //
    println!("{}", poly * poly2);
    // println!("{}", poly_2);
}

