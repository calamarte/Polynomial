use core::panic;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Index, Mul, Neg},
};

use num::{Integer, Zero};

use crate::{mono::Monomial, MonomialValue};

/// Equations differents types
#[derive(PartialEq, Debug)]
pub enum EquationType {
    /// [Linear equation](https://en.wikipedia.org/wiki/Linear_equation)
    /// `2x + 1`
    Linear,

    /// [Quadratic equation](https://en.wikipedia.org/wiki/Quadratic_equation)
    /// `2x^2 + 2x + 1`
    Quadratic,

    /// [Biquadratic equation](https://en.wikipedia.org/wiki/Quartic_equation)
    /// `2x^4 + 2x^2 + 1`
    Biquadratic,

    /// Equation with exponent grater than **2**
    /// `2x^3 + 3x^2`
    BigExp,

    /// Equation with exponent grater than **2**, but they only have **two** terms
    /// `2x^3 + 100`
    BigExp2Terms,

    /// Invalid equation type
    Invalid,
}

/// [Polynomial](https://en.wikipedia.org/wiki/Polynomial) representation
#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T> {
    mono_vec: Vec<Monomial<T>>,
}

impl<T: MonomialValue> Polynomial<T> {
    /// Constructs a new `Polynomial<T>`
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let mono_vec: Vec<Monomial<i32>> = vec![
    ///     Monomial::new(2, 2),
    ///     Monomial::new(-1, 1),
    ///     Monomial::new(10, 0),
    /// ];
    ///
    /// let poly: Polynomial<i32> = Polynomial::new(mono_vec);
    /// ```
    pub fn new(mono_vec: Vec<Monomial<T>>) -> Polynomial<T> {
        let mut poly = Polynomial { mono_vec };
        poly.collapse();
        poly
    }

    /// Sum all Monomials with the same exponent and collapse in a simplificated
    fn collapse(&mut self) {
        let mut group_by_exp: HashMap<i32, Vec<Monomial<T>>> = HashMap::new();
        for mono in self.mono_vec.iter() {
            group_by_exp
                .entry(mono.get_exp())
                .or_insert_with(Vec::new)
                .push(*mono);
        }

        let mut mono_vec: Vec<Monomial<T>> = group_by_exp
            .into_iter()
            .map(|(_, m)| m.into_iter().sum::<Monomial<T>>())
            .collect();

        mono_vec.retain(|&m| m.get_value() != T::zero());

        mono_vec.sort_by(|m1, m2| m2.get_exp().cmp(&m1.get_exp()));

        self.mono_vec = mono_vec;
    }

    /// Returns the monomial with the max exponent
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let poly: Polynomial<i32> = Polynomial::try_from("x^2 - 5x - 100").unwrap();
    ///
    /// assert_eq!(poly.max_exp().get_exp(), 2);
    /// ```
    pub fn max_exp(&self) -> Monomial<T> {
        if self.mono_vec.is_empty() {
            return Monomial::default();
        }

        self.mono_vec[0]
    }

    /// Add a monomial but without [`collapse`]
    fn push_raw(&mut self, mono: Monomial<T>) {
        self.mono_vec.push(mono);
    }

    /// Returns the number of Monomials in the Polynomial, also referred to as its 'length'
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let poly: Polynomial<i32> = Polynomial::default();
    ///
    /// assert_eq!(poly.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.mono_vec.len()
    }

    /// Returns the equation type
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial, EquationType};
    /// let poly: Polynomial<i32> = Polynomial::try_from("5x - 100").unwrap();
    ///
    /// assert_eq!(poly.equation_type(), EquationType::Linear);
    /// ```
    pub fn equation_type(&self) -> EquationType {
        match self.max_exp().get_exp() {
            0 => EquationType::Invalid,
            1 => EquationType::Linear,
            e if e > 1 && self.len() == 2 => EquationType::BigExp2Terms,
            2 => EquationType::Quadratic,
            4 if self.find_by_exp(3).get_value().is_zero()
                && self.find_by_exp(1).get_value().is_zero() =>
            {
                EquationType::Biquadratic
            }
            _ => EquationType::BigExp,
        }
    }

    /// Add a monomial
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let mut poly: Polynomial<i32> = Polynomial::try_from("5x - 100").unwrap();
    /// let mono: Monomial<i32> = Monomial::try_from("2x^2").unwrap();
    ///
    /// poly.push(mono);
    ///
    /// assert_eq!(format!("{poly}"), "2x^2 + 5x - 100");
    ///
    /// ```
    pub fn push(&mut self, mono: Monomial<T>) {
        self.push_raw(mono);
        self.collapse();
    }

    /// Find monomial in a polynomial by the exponent if don't find the monomial returns
    /// [`Monomial::default()`]
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let poly: Polynomial<i32> = Polynomial::try_from("2x^2 + 5x - 100").unwrap();
    ///
    /// assert_eq!(poly.find_by_exp(1), Monomial::new(5, 1));
    /// assert_eq!(poly.find_by_exp(10), Monomial::default());
    ///
    /// ```
    pub fn find_by_exp(&self, exp: i32) -> Monomial<T> {
        self.into_iter()
            .find(|m| m.get_exp() == exp)
            .cloned()
            .unwrap_or_default()
    }

    /// Returns a new polynomial as result of dividing a monomial
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let poly: Polynomial<i32> = Polynomial::try_from("10x - 10").unwrap();
    /// let mono: Monomial<i32> = Monomial::try_from("2").unwrap();
    ///
    /// let result = poly.div_mono(mono);
    ///
    /// assert_eq!(format!("{result}"), "5x - 5");
    ///
    /// ```
    pub fn div_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m / rhs).collect();

        Polynomial::new(mono_vec)
    }

    /// Returns a new polynomial as result of multiplying a monomial
    /// # Examples
    /// ```
    /// # use rust_polynomial::{Polynomial, Monomial};
    /// let poly: Polynomial<i32> = Polynomial::try_from("10x - 10").unwrap();
    /// let mono: Monomial<i32> = Monomial::try_from("2").unwrap();
    ///
    /// let result = poly.mul_mono(mono);
    ///
    /// assert_eq!(format!("{result}"), "20x - 20");
    ///
    /// ```
    pub fn mul_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m * rhs).collect();

        Polynomial::new(mono_vec)
    }

    /// Returns an [`Option`] containing the roots of the equation
    /// This function uses different strategies based on [`EquationType`]
    /// # Examples
    /// ```
    ///# use rust_polynomial::Polynomial;
    /// let poly: Polynomial<i32> = Polynomial::try_from("x - 9").unwrap();
    ///
    /// assert_eq!(poly.roots(), Some(vec![9]));
    /// ```
    pub fn roots(&self) -> Option<Vec<T>> {
        match self.equation_type() {
            EquationType::Linear => Polynomial::<T>::linear_root(self),
            EquationType::Quadratic => Polynomial::<T>::quadratic_root(self),
            EquationType::Biquadratic => Polynomial::<T>::biquadratic_root(self),
            EquationType::BigExp2Terms => Polynomial::<T>::big_exp2_root(self),
            EquationType::BigExp => Polynomial::<T>::big_exp_root(self),
            EquationType::Invalid => None,
        }
    }

    fn linear_root(poly: &Self) -> Option<Vec<T>> {
        let len = poly.into_iter().len();

        if len > 2 || len < 1 {
            panic!("{poly} is not a linear equation");
        }

        if len == 1 {
            return Some(vec![T::zero()]);
        }

        let result = poly[1].neg().get_value() / poly[0].get_value();

        Some(vec![result])
    }

    fn quadratic_root(poly: &Self) -> Option<Vec<T>> {
        #[rustfmt::skip]
        let [a, b, c] = [
            poly.max_exp(),
            poly.find_by_exp(1),
            poly.find_by_exp(0)
        ].map(|m|m.get_value().to_f64().unwrap());

        if b.is_zero() && c.is_zero() {
            return Some(vec![T::zero()]);
        }

        if b.is_zero() {
            let mut linear = poly.clone();
            linear.mono_vec[0].exp = 1;
            let linear_result = Polynomial::<T>::linear_root(&linear)?[0].to_f64()?;
            let sqrt = T::from(linear_result.sqrt())?;

            if sqrt.is_zero() {
                return Some(vec![sqrt]);
            }

            return Some(vec![sqrt.neg(), sqrt]);
        }

        let sqrt = ((b * b) - (4f64 * a * c)).sqrt();

        let result_1 = T::from((b.neg() + sqrt) / (2f64 * a))?;
        let result_2 = T::from((b.neg() - sqrt) / (2f64 * a))?;

        if result_1 == result_2 {
            return Some(vec![result_1]);
        }

        let mut result = vec![result_1, result_2];
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Some(result)
    }

    fn biquadratic_root(poly: &Self) -> Option<Vec<T>> {
        let [mut a, mut b, c] = [
            poly.find_by_exp(4),
            poly.find_by_exp(2),
            poly.find_by_exp(0),
        ];

        a.exp = 2;
        b.exp = 1;

        let quadratic: Polynomial<T> = Polynomial::new(vec![a, b, c]);
        let quadrtic_result = Polynomial::<T>::quadratic_root(&quadratic)?;

        if quadrtic_result.len() == 1 {
            let result = T::from(quadrtic_result[0].to_f64()?.sqrt())?;

            return Some(vec![result.neg(), result]);
        }

        let sqrt_converter = |x: T| T::from(x.to_f64()?.abs().sqrt());

        if let [r1, r2] = *quadrtic_result {
            let mut result = if r1.abs() == r2.abs() {
                let r = sqrt_converter(r1)?;
                vec![r.neg(), r]
            } else {
                let r1_1 = sqrt_converter(r1)?;
                let r2_1 = sqrt_converter(r2)?;
                vec![r1_1.neg(), r2_1.neg(), r1_1, r2_1]
            };
            result.sort_by(|a, b| a.partial_cmp(b).unwrap());

            return Some(result);
        }

        None
    }

    fn big_exp2_root(poly: &Self) -> Option<Vec<T>> {
        if poly.len() != 2 {
            return None;
        }

        let [a, b] = [poly.max_exp(), poly.find_by_exp(0)];
        let value = (b.get_value().neg() / a.get_value()).to_f64()?;

        if a.get_exp().is_even() && value.is_sign_negative() {
            return None;
        }

        let mut result_val = T::from(value.abs().powf(1f64 / a.get_exp() as f64))?;

        if a.get_exp().is_odd() {
            if value < 0f64 {
                result_val = result_val.neg();
            }

            return Some(vec![result_val]);
        }

        let mut result = vec![result_val.neg(), result_val];
        result.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Some(result)
    }

    fn big_exp_root(poly: &Self) -> Option<Vec<T>> {
        let (root, rest) = Polynomial::<T>::find_root(poly);

        let mut roots: Vec<T> = Vec::new();

        if let Some(r) = root {
            roots.push(T::from(r).unwrap())
        }

        if let Some(poly) = rest {
            if let Some(r) = poly.roots() {
                let generic: Vec<T> = r.into_iter().map(T::from).map(|o| o.unwrap()).collect();

                roots.extend(generic);
            }
        }

        if roots.is_empty() {
            return None;
        }

        roots.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Some(roots)
    }

    fn find_divs(value: u64) -> Vec<i64> {
        let mut divs: Vec<i64> = Vec::new();

        for v in 1..=value {
            if value % v == 0 {
                divs.push(v as i64);
            }
        }

        let negative: Vec<_> = divs.iter().map(|d| d.neg()).collect();
        divs.extend(negative);

        divs
    }

    fn find_root(poly: &Self) -> (Option<i64>, Option<Polynomial<i64>>) {
        let root_base = match poly.find_by_exp(0).get_value().abs().to_u64() {
            Some(rb) => rb,
            None => return (None, None),
        };

        let divs = Polynomial::<T>::find_divs(root_base);

        let mut target: Polynomial<i64> = Polynomial::default();
        let mut root: Option<i64> = None;

        'div_loop: for div in divs {
            let max_exp = poly.max_exp().get_exp();
            let mut current = 0i64;
            let mut current_poly: Polynomial<i64> = Polynomial::default();
            for (i, exp) in (0..=max_exp).rev().enumerate() {
                let mono_val = match poly.find_by_exp(exp).get_value().to_i64() {
                    Some(val) => val,
                    None => return (None, None),
                };

                let sum = mono_val + current;

                current_poly.push_raw(Monomial::new(sum, exp - 1));

                if i as i32 == max_exp && sum.is_zero() {
                    root.replace(div);
                    current_poly.collapse();
                    target = current_poly;
                    break 'div_loop;
                }

                current = sum * div;
            }
        }

        if root.is_none() {
            return (None, None);
        }

        if target == Polynomial::<i64>::default() {
            return (root, None);
        }

        (root, Some(target))
    }
}

impl<T: MonomialValue> Default for Polynomial<T> {
    fn default() -> Self {
        Polynomial::new(vec![Monomial::default()])
    }
}

impl<T: MonomialValue> From<Vec<Monomial<T>>> for Polynomial<T> {
    fn from(value: Vec<Monomial<T>>) -> Self {
        Polynomial::new(value)
    }
}

impl<T: MonomialValue> TryFrom<&str> for Polynomial<T> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let clean_value = value.trim().replace(" ", "");

        if clean_value.len() == 1 {
            let mono = Monomial::try_from(&clean_value as &str)?;
            return Ok(Polynomial::new(vec![mono]));
        }

        let mut mono_vec: Vec<Monomial<T>> = Vec::new();
        let mut tmp_mono_split = String::new();
        for (i, char) in clean_value.char_indices() {
            if i == 0 {
                tmp_mono_split.push(char);
                continue;
            }

            if vec!['-', '+'].contains(&char) {
                mono_vec.push(Monomial::try_from(&tmp_mono_split as &str)?);
                tmp_mono_split.clear();
                tmp_mono_split.push(char);
                continue;
            }

            if i == clean_value.len() - 1 {
                tmp_mono_split.push(char);
                mono_vec.push(Monomial::try_from(&tmp_mono_split as &str)?);
                continue;
            }

            tmp_mono_split.push(char);
        }

        Ok(Polynomial::new(mono_vec))
    }
}

impl<T: MonomialValue> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mono_vec = self.into_iter().map(Monomial::neg).collect();

        Polynomial::new(mono_vec)
    }
}

impl<T: MonomialValue> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Polynomial::new([self.mono_vec.clone(), rhs.mono_vec.clone()].concat())
    }
}

impl<T: MonomialValue> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: Vec<Monomial<T>> = Vec::new();
        for self_mono in &self {
            for rhs_mono in &rhs {
                result.push(*self_mono * *rhs_mono);
            }
        }

        Polynomial::new(result)
    }
}

impl<T: MonomialValue> Div for Polynomial<T> {
    type Output = (Self, Self);

    fn div(self, rhs: Self) -> Self::Output {
        let mut dividend = self;
        let divider = rhs;
        let mut quotient: Polynomial<T> = Polynomial::default();

        while dividend.max_exp().get_exp() >= divider.max_exp().get_exp() {
            let result = dividend.max_exp() / divider.max_exp();
            quotient.push_raw(result);

            dividend = dividend.clone() + (divider.clone().mul_mono(result)).neg();
        }

        quotient.collapse();

        (quotient, dividend)
    }
}

impl<T: MonomialValue> TryFrom<Vec<T>> for Polynomial<T> {
    type Error = &'static str;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        let mut mono_vec: Vec<Monomial<T>> = Vec::new();

        for (i, v) in value.iter().enumerate() {
            if *v == T::zero() {
                continue;
            }

            let exp = value.len() - 1 - i;
            mono_vec.push(Monomial::new(*v as T, exp as i32));
        }

        if mono_vec.is_empty() {
            return Ok(Polynomial::default());
        }

        Ok(Polynomial::new(mono_vec))
    }
}

impl<T: MonomialValue> Index<usize> for Polynomial<T> {
    type Output = Monomial<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mono_vec[index]
    }
}

impl<T: MonomialValue> IntoIterator for Polynomial<T> {
    type Item = Monomial<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mono_vec.into_iter()
    }
}

impl<'a, T: MonomialValue> IntoIterator for &'a Polynomial<T> {
    type Item = &'a Monomial<T>;
    type IntoIter = std::slice::Iter<'a, Monomial<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.mono_vec.iter()
    }
}

impl<T: MonomialValue> Display for Polynomial<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mono_vec.is_empty() {
            write!(f, "0")?;
            return Ok(());
        }

        for (i, mono) in self.mono_vec.iter().enumerate() {
            let sign = match mono.get_value() < T::zero() {
                true if i == 0 => "-".to_string(),
                true => " - ".to_string(),
                false if i == 0 => "".to_string(),
                false => " + ".to_string(),
            };

            let mono_abs = Monomial::new(mono.get_value().abs(), mono.get_exp());

            write!(f, "{sign}{mono_abs}")?;
        }

        Ok(())
    }
}
