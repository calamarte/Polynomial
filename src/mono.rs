use std::{default::Default, fmt::{Display, Error}, i64, iter::Sum, num::IntErrorKind, ops::Add, str::FromStr };

use num::{Num, NumCast, Signed};


#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Monomial<T>{
    value: T,
    exp: i32,
}

impl<T> Monomial<T>
where 
    T: Num + NumCast + Signed + Copy + Default + Display + FromStr + PartialOrd
{
    pub fn new(value: T, exp: i32) -> Monomial<T> {
        Monomial { value, exp }
    }

    pub fn get_value(&self) -> T {
        self.value
    }
    
    pub fn get_exp(&self) -> i32 {
        self.exp
    }

    pub fn is_operable(&self, other: &Self) -> bool {
        self.exp == other.exp
    }
}

impl<T> TryFrom<&str> for Monomial<T> 
where 
    T: Num + NumCast + Signed + Copy + Default + Display + FromStr + PartialOrd
{
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let clean_value = value
            .trim()
            .to_lowercase()
            .replace(" ", "")
            .replace("^", "");

        let mut split: Vec<&str> = clean_value.split("x").collect();

        if split.is_empty() {
            return Ok(Monomial::default());
        }

        if "-" == split[0] {
            split[0] = "-1";
        }

        let base = match split[0].parse::<T>() {
            Ok(v) => v,
            Err(_) if split[0].is_empty() => T::one(),
            Err(_) => return Err("Not valid base"),
        };

        let exp = match split.len() {
            1 => 0,
            2 => match split[1].parse::<i32>() {
                Ok(v) => v,
                Err(err) if err.kind() == &IntErrorKind::Empty => 1,
                Err(_) => return Err("Not valid exponent"),
            },
            _ => return Err("To much symbols"),
        };

        Ok(Monomial { value: base, exp })
    }
}

impl<T> Add for Monomial<T>
where 
    T: Num + NumCast + Signed + Copy + Default + Display + FromStr + PartialOrd
{
    type Output = Result<Self, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.is_operable(&rhs) {
            return Err("Monomials only allow add same exponent");
        }

        Ok(Monomial::new(self.value + rhs.value, self.exp))
    }
}

impl<T> Sum<Self> for Monomial<T>
where 
    T: Num + NumCast + Signed + Copy + Default + Display + FromStr + PartialOrd
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {

        let mut exp = 0;
        let mut sum = T::zero();
        for mono in iter {
            exp = mono.get_exp();
            sum = sum + mono.get_value();
        }

        Monomial::new(sum, exp)
    }
}

impl<T> Display for Monomial<T>
where 
    T: Num + NumCast + Signed + Copy + Default + Display + FromStr + PartialOrd
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: i64 = T::to_i64(&self.value).ok_or(Error)?;
        let base: String = match val {
            -1 if self.exp == 0 => "-1".to_string(),
            -1 => "-".to_string(),
            1 if self.exp == 0 => "1".to_string(),
            1 => "".to_string(),
            _ => format!("{}", self.value),
        };

        let exp: String = match self.exp {
            0 => "".to_string(),
            1 => "x".to_string(),
            _ => format!("x^{}", self.exp),
        };

        write!(f, "{}{}", base, exp)
    }
}

