#![allow(warnings)]

use rust_polynomial::{Monomial, Polynomial};

fn main() {
    let mono_vec: Vec<Monomial<i8>> = vec![
        Monomial::try_from("-2x").unwrap(),
        Monomial::try_from("1").unwrap(),
    ];

    // let poly: Polynomial<i32> = Polynomial::try_from("-2x^2 + 2x^3 - 10 - 1").unwrap();
    let poly: Polynomial<i32> = Polynomial::try_from("6x4 + 5x3 - 7x2 + 3x +2").unwrap();
    let poly2: Polynomial<i32> = Polynomial::try_from("2x2 + 3x - 1").unwrap();
    // let poly_2: Polynomial<i32> = Polynomial::try_from(vec![1,0,0,0, 1, 1]).unwrap();

    // for mono in &poly {
    //     println!("{mono}");
    // }
    //
    //
    let (result, rest) = poly / poly2;
    println!("{result}\n{rest}");
    // println!("{}", poly_2);
}

