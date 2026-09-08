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
