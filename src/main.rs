use rust_polynomial::{Monomial, Polynomial};

fn main() {
    // let poly = Polynomial::new(vec![10f64]);
    let mono = Monomial::try_from("2x").unwrap();


    println!("{:?}", mono);
}
