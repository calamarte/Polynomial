use std::num::IntErrorKind;

#[derive(Default, Debug, PartialEq)]
pub struct Monomial {
    value: i32,
    exp: i32,
}

impl TryFrom<&str> for Monomial {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let clean_value = value.trim()
            .to_lowercase()
            .replace(" ", "")
            .replace("^", "");

        let split: Vec<&str> = clean_value.split("x").collect();

        if split.is_empty() {
            return Ok(Monomial::default());
        }

        let base = match split[0].parse::<i32>() {
            Ok(v) => v,
            Err(err) if err.kind() == &IntErrorKind::Empty => 1,
            Err(_) => return Err("Not valid base"),
        };

        let exp = match split.len() {
            1 => 0,
            2 => match split[1].parse::<i32>() {
                Ok(v) => v,
                Err(err) if err.kind() == &IntErrorKind::Empty => 1,
                Err(_) => return Err("Not valid exponent"),
            }
            _ => return Err("To much symbols")

        };

        Ok(Monomial { value: base, exp })
    }
}




#[derive(Default, Debug)]
pub struct Polynomial {
    internal: Vec<f64>,
}

impl Polynomial {
    pub fn new(internal: Vec<f64>) -> Polynomial {
        Polynomial { internal }
    }
}

impl From<Vec<f64>> for Polynomial {
    fn from(value: Vec<f64>) -> Self {
        Polynomial { internal: value }
    }
}

impl TryFrom<&str> for Polynomial {
    type Error = &'static str;

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}



#[test]
fn test_try_from_valid_input() {
   let tests = std::collections::HashMap::from([
        ("1", Monomial {value: 1, exp: 0}),
        (" 1 ", Monomial {value: 1, exp: 0}),
        ("- 1 ", Monomial {value: -1, exp: 0}),
        ("+ 1 ", Monomial {value: 1, exp: 0}),
        ("2", Monomial {value: 2, exp: 0}),
        ("x", Monomial {value: 1, exp: 1}),
        ("2x", Monomial {value: 2, exp: 1}),
        ("2X", Monomial {value: 2, exp: 1}),
        ("2x^2", Monomial {value: 2, exp: 2}),
        ("2x2", Monomial {value: 2, exp: 2}),
        ("-23x^2", Monomial {value: -23, exp: 2}),
        ("-23x^-2", Monomial {value: -23, exp: -2}),
        ("-23x-2", Monomial {value: -23, exp: -2}),
    ]);

    for (str, mono) in tests {
        println!("{} -> {:?}", str, &mono);
        assert_eq!(Monomial::try_from(str).unwrap(), mono);
    }
}

#[test]
fn test_try_from_invalid_input() {
    let invalid = [
        "a", "2b", "2xabc", "flkjasdf"
    ];

    for input in invalid {
        assert!(Monomial::try_from(input).is_err());
    }
}
