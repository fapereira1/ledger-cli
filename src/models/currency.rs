use std::iter::Sum;
use std::ops::{Add, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Amount(i64);

impl Amount {
    /// Cria um Amount a partir de centavos.
    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    /// Cria um Amount a partir de reais (converte internamente).
    pub fn from_reais(reais: f64) -> Self {
        Self((reais * 100.0).round() as i64)
    }

    /// Retorna o valor em centavos.
    pub fn cents(&self) -> i64 {
        self.0
    }

    /// Retorna true se o valor for negativo.
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Retorna true se o valor for zero.
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reais = self.0.abs() / 100;
        let centavos = self.0.abs() % 100;
        let sinal = if self.0 < 0 { "-" } else { "" };

        write!(f, "{}R$ {}.{:02}", sinal, reais, centavos)
    }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Amount {
        Amount(self.0 + other.0)
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, other: Amount) -> Amount {
        Amount(self.0 - other.0)
    }
}

impl Neg for Amount {
    type Output = Amount;
    fn neg(self) -> Amount {
        Amount(-self.0)
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Amount>>(iter: I) -> Amount {
        iter.fold(Amount(0), |acc, x| acc + x)
    }
}
