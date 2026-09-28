// src/models/transaction.rs
use crate::models::reportable::Reportable;
use std::fmt;

use crate::models::currency::Amount;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionType {
    Credit { account_id: u64 },
    Debit { account_id: u64 },
    Transfer { from_id: u64, to_id: u64 },
}

impl TransactionType {
    pub fn display_name(&self) -> String {
        match self {
            TransactionType::Credit { .. } => String::from("Crédito"),
            TransactionType::Debit { .. } => String::from("Débito"),
            TransactionType::Transfer { from_id, to_id } => {
                format!("Transferência (conta {} → conta {})", from_id, to_id)
            }
        }
    }

    pub fn is_transfer(&self) -> bool {
        matches!(self, TransactionType::Transfer { .. })
    }

    pub fn is_credit(&self) -> bool {
        matches!(self, TransactionType::Credit { .. })
    }

    pub fn is_debit(&self) -> bool {
        matches!(self, TransactionType::Debit { .. })
    }
}

/// Representa uma transação financeira.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub transaction_type: TransactionType,
    pub amount: Amount,
    pub description: Option<String>, // descrição é opcional
}

impl Transaction {
    pub fn new(
        id: u64,
        transaction_type: TransactionType,
        amount: Amount,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            transaction_type,
            amount,
            description,
        }
    }

    /// Retorna a descrição ou um texto padrão se não houver.
    pub fn description_or_default(&self) -> &str {
        match &self.description {
            Some(desc) => desc,
            None => "(sem descrição)",
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | {} | {}",
            self.id,
            self.transaction_type.display_name(),
            self.amount,
            self.description_or_default(),
        )
    }
}

impl Reportable for Transaction {
    fn summary(&self) -> String {
        format!("{} | {}", self.transaction_type.display_name(), self.amount)
    }

    fn detail(&self) -> String {
        format!(
            "Tx #{} | Tipo: {} | Valor: {} | Descrição: {}",
            self.id,
            self.transaction_type.display_name(),
            self.amount,
            self.description_or_default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::currency::Amount;
    use crate::models::reportable::Reportable;

    // --- helpers ---

    fn tx_credit() -> Transaction {
        Transaction::new(
            1,
            TransactionType::Credit { account_id: 1 },
            Amount::from_reais(100.00),
            Some(String::from("Salário")),
        )
    }

    fn tx_debit() -> Transaction {
        Transaction::new(
            2,
            TransactionType::Debit { account_id: 1 },
            Amount::from_reais(50.00),
            None,
        )
    }

    fn tx_transfer() -> Transaction {
        Transaction::new(
            3,
            TransactionType::Transfer {
                from_id: 1,
                to_id: 2,
            },
            Amount::from_reais(200.00),
            Some(String::from("Fatura")),
        )
    }

    // --- TransactionType::is_credit ---

    #[test]
    fn is_credit_retorna_true_para_credit() {
        assert!(TransactionType::Credit { account_id: 1 }.is_credit());
    }

    #[test]
    fn is_credit_retorna_false_para_debit() {
        assert!(!TransactionType::Debit { account_id: 1 }.is_credit());
    }

    #[test]
    fn is_credit_retorna_false_para_transfer() {
        assert!(
            !TransactionType::Transfer {
                from_id: 1,
                to_id: 2
            }
            .is_credit()
        );
    }

    // --- TransactionType::is_debit ---

    #[test]
    fn is_debit_retorna_true_para_debit() {
        assert!(TransactionType::Debit { account_id: 1 }.is_debit());
    }

    #[test]
    fn is_debit_retorna_false_para_credit() {
        assert!(!TransactionType::Credit { account_id: 1 }.is_debit());
    }

    #[test]
    fn is_debit_retorna_false_para_transfer() {
        assert!(
            !TransactionType::Transfer {
                from_id: 1,
                to_id: 2
            }
            .is_debit()
        );
    }

    // --- TransactionType::is_transfer ---

    #[test]
    fn is_transfer_retorna_true_para_transfer() {
        assert!(
            TransactionType::Transfer {
                from_id: 1,
                to_id: 2
            }
            .is_transfer()
        );
    }

    #[test]
    fn is_transfer_retorna_false_para_credit() {
        assert!(!TransactionType::Credit { account_id: 1 }.is_transfer());
    }

    #[test]
    fn is_transfer_retorna_false_para_debit() {
        assert!(!TransactionType::Debit { account_id: 1 }.is_transfer());
    }

    // --- TransactionType::display_name ---

    #[test]
    fn display_name_credit() {
        let nome = TransactionType::Credit { account_id: 1 }.display_name();
        assert_eq!(nome, "Crédito");
    }

    #[test]
    fn display_name_debit() {
        let nome = TransactionType::Debit { account_id: 1 }.display_name();
        assert_eq!(nome, "Débito");
    }

    #[test]
    fn display_name_transfer_contem_ids() {
        let nome = TransactionType::Transfer {
            from_id: 1,
            to_id: 2,
        }
        .display_name();
        assert!(nome.contains("1"));
        assert!(nome.contains("2"));
    }

    // --- Transaction::new ---

    #[test]
    fn new_define_id_corretamente() {
        assert_eq!(tx_credit().id, 1);
    }

    #[test]
    fn new_define_amount_corretamente() {
        assert_eq!(tx_credit().amount.cents(), 10000);
    }

    // --- description_or_default ---

    #[test]
    fn description_or_default_com_descricao() {
        assert_eq!(tx_credit().description_or_default(), "Salário");
    }

    #[test]
    fn description_or_default_sem_descricao() {
        assert_eq!(tx_debit().description_or_default(), "(sem descrição)");
    }

    // --- Display ---

    #[test]
    fn display_credit_formatado() {
        let output = format!("{}", tx_credit());
        assert!(output.contains("[1]"));
        assert!(output.contains("Crédito"));
        assert!(output.contains("R$ 100.00"));
        assert!(output.contains("Salário"));
    }

    #[test]
    fn display_debit_sem_descricao() {
        let output = format!("{}", tx_debit());
        assert!(output.contains("(sem descrição)"));
    }

    #[test]
    fn display_transfer_contem_ids() {
        let output = format!("{}", tx_transfer());
        assert!(output.contains("conta 1"));
        assert!(output.contains("conta 2"));
    }

    // --- Reportable ---

    #[test]
    fn summary_contem_tipo_e_valor() {
        let summary = tx_credit().summary();
        assert!(summary.contains("Crédito"));
        assert!(summary.contains("R$ 100.00"));
    }

    #[test]
    fn detail_contem_id_tipo_valor_descricao() {
        let detail = tx_transfer().detail();
        assert!(detail.contains("3")); // id
        assert!(detail.contains("conta 1")); // from
        assert!(detail.contains("conta 2")); // to
        assert!(detail.contains("R$ 200.00"));
        assert!(detail.contains("Fatura"));
    }
}
