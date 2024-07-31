//! # Polynomials
//!
//! Math lib to operate **Polynomials expressions**
//!
//! ## Features
//!
//! Math sintax support
//! ```
//! # use rust_polynomial::Polynomial;
//! let poly: Polynomial<i32> = Polynomial::try_from("x^2 + 4x - 100").unwrap();
//! ```
//!
//! Support opertaions for monomials and polynomials:
//! - `+` Add
//! - `-` Negate
//! - `*` Multiply
//! - `/` Divide
//! - Root Calculation (Only Polynomial)
//!

mod mono;
mod poly;

pub use mono::*;
pub use poly::*;
