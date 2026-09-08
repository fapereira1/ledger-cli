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

// src/models/currency.rs — adicione no final

#[cfg(test)]
mod tests {
    use super::*;

    // --- construção ---

    #[test]
    fn from_cents_armazena_valor_correto() {
        let amount = Amount::from_cents(1050);
        assert_eq!(amount.cents(), 1050);
    }

    #[test]
    fn from_reais_converte_para_centavos() {
        let amount = Amount::from_reais(10.50);
        assert_eq!(amount.cents(), 1050);
    }

    #[test]
    fn from_reais_arredonda_corretamente() {
        let amount = Amount::from_reais(10.999);
        assert_eq!(amount.cents(), 1100);
    }

    #[test]
    fn from_reais_valor_negativo() {
        let amount = Amount::from_reais(-50.0);
        assert_eq!(amount.cents(), -5000);
    }

    // --- predicados ---

    #[test]
    fn is_negative_com_valor_negativo() {
        assert!(Amount::from_cents(-1).is_negative());
    }

    #[test]
    fn is_negative_com_valor_positivo() {
        assert!(!Amount::from_cents(1).is_negative());
    }

    #[test]
    fn is_zero_com_zero() {
        assert!(Amount::from_cents(0).is_zero());
    }

    #[test]
    fn is_zero_com_valor_nao_zero() {
        assert!(!Amount::from_cents(1).is_zero());
    }

    // --- aritmética ---

    #[test]
    fn add_dois_amounts() {
        let a = Amount::from_reais(10.00);
        let b = Amount::from_reais(5.00);
        assert_eq!((a + b).cents(), 1500);
    }

    #[test]
    fn sub_dois_amounts() {
        let a = Amount::from_reais(10.00);
        let b = Amount::from_reais(3.00);
        assert_eq!((a - b).cents(), 700);
    }

    #[test]
    fn neg_inverte_sinal() {
        let a = Amount::from_reais(10.00);
        assert_eq!((-a).cents(), -1000);
    }

    #[test]
    fn sum_colecao_de_amounts() {
        let valores = vec![
            Amount::from_reais(10.00),
            Amount::from_reais(20.00),
            Amount::from_reais(30.00),
        ];
        let total: Amount = valores.into_iter().sum();
        assert_eq!(total.cents(), 6000);
    }

    #[test]
    fn sum_colecao_vazia_retorna_zero() {
        let valores: Vec<Amount> = vec![];
        let total: Amount = valores.into_iter().sum();
        assert!(total.is_zero());
    }

    // --- comparação ---

    #[test]
    fn amounts_iguais() {
        assert_eq!(Amount::from_cents(100), Amount::from_cents(100));
    }

    #[test]
    fn amount_menor_que_outro() {
        assert!(Amount::from_reais(5.00) < Amount::from_reais(10.00));
    }

    #[test]
    fn amount_maior_que_outro() {
        assert!(Amount::from_reais(10.00) > Amount::from_reais(5.00));
    }

    // --- display ---

    #[test]
    fn display_valor_positivo() {
        let amount = Amount::from_reais(10.50);
        assert_eq!(format!("{}", amount), "R$ 10.50");
    }

    #[test]
    fn display_valor_negativo() {
        let amount = Amount::from_reais(-10.50);
        assert_eq!(format!("{}", amount), "-R$ 10.50");
    }

    #[test]
    fn display_zero() {
        let amount = Amount::from_cents(0);
        assert_eq!(format!("{}", amount), "R$ 0.00");
    }

    #[test]
    fn display_centavos_com_zero_a_esquerda() {
        let amount = Amount::from_cents(105);  // R$ 1,05
        assert_eq!(format!("{}", amount), "R$ 1.05");
    }
}
