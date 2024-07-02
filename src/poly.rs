use core::panic;
use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Index, Mul, Neg},
};

use num::Zero;

use crate::{mono::Monomial, MonomialValue};

#[derive(PartialEq, Debug)]
pub enum EquationType {
    Linear,
    Quadratic,
    Biquadratic,
    Invalid,
    BigExp,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polynomial<T> {
    mono_vec: Vec<Monomial<T>>,
}

impl<T: MonomialValue> Polynomial<T> {
    pub fn new(mono_vec: Vec<Monomial<T>>) -> Polynomial<T> {
        let mut poly = Polynomial { mono_vec };
        poly.collapse();
        poly
    }

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

    fn max_exp(&self) -> Monomial<T> {
        if self.mono_vec.is_empty() {
            return Monomial::default();
        }

        self.mono_vec[0]
    }

    fn push_raw(&mut self, mono: Monomial<T>) {
        self.mono_vec.push(mono);
    }

    pub fn equation_type(&self) -> EquationType {
        match self.max_exp().get_exp() {
            0 => EquationType::Invalid,
            1 => EquationType::Linear,
            2 => EquationType::Quadratic,
            4 if self.find_by_exp(3).get_value().is_zero()
                && self.find_by_exp(1).get_value().is_zero() =>
            {
                EquationType::Biquadratic
            }
            _ => EquationType::BigExp,
        }
    }

    pub fn push(&mut self, mono: Monomial<T>) {
        self.push_raw(mono);
        self.collapse();
    }

    pub fn find_by_exp(&self, exp: i32) -> Monomial<T> {
        self.into_iter()
            .find(|m| m.get_exp() == exp)
            .cloned()
            .unwrap_or_default()
    }

    pub fn div_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m / rhs).collect();

        Polynomial::new(mono_vec)
    }

    pub fn mul_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m * rhs).collect();

        Polynomial::new(mono_vec)
    }

    pub fn roots(&self) -> Option<Vec<T>> {
        match self.equation_type() {
            EquationType::Linear => Polynomial::<T>::linear_root(self),
            EquationType::Quadratic => Polynomial::<T>::quadratic_root(self),
            EquationType::Biquadratic => Polynomial::<T>::biquadratic_root(self),
            EquationType::Invalid => None,
            t @ _ => panic!("{t:?} not implemeted yet!"),
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

           return Some(result)
        } 

        None

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
