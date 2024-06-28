use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Neg},
};

use crate::{mono::Monomial, MonomialValue};

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

    pub fn push(&mut self, mono: Monomial<T>) {
        self.push_raw(mono);
        self.collapse();
    }

    pub fn div_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m / rhs).collect();

        Polynomial::new(mono_vec)
    }

    pub fn mul_mono(self, rhs: Monomial<T>) -> Self {
        let mono_vec = self.into_iter().map(|m| m * rhs).collect();

        Polynomial::new(mono_vec)
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
        let mono_vec = self.mono_vec.into_iter().map(Monomial::neg).collect();

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
