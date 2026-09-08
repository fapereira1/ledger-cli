use crate::models::reportable::Reportable;
use std::fmt;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AccountType {
    Checking,   //conta corrente
    Savings,    // poupança
    Credit,     // crédito
    Investment, // investimento
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: u64,
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
}

impl Account {
    pub fn new(id: u64, name: String, account_type: AccountType) -> Self {
        Self {
            id,
            name,
            account_type,
            balance: 0.0,
        }
    }

    pub fn is_overdrawn(&self) -> bool {
        self.balance < 0.0
    }
}

impl AccountType {
    pub fn display_name(&self) -> &str {
        match self {
            AccountType::Checking => "Conta Corrente",
            AccountType::Savings => "Poupança",
            AccountType::Credit => "Crédito",
            AccountType::Investment => "Investimento",
        }
    }

    pub fn allows_negative_balance(&self) -> bool {
        match self {
            AccountType::Credit => true,
            _ => false, // _ é o caso "todos os outros"
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} | {} | saldo: {}",
            self.id,
            self.name,
            self.account_type.display_name(),
            self.balance,
        )
    }
}

impl Reportable for Account {
    fn summary(&self) -> String {
        format!("{} ({})", self.name, self.account_type.display_name())
    }

    fn detail(&self) -> String {
        format!(
            "Conta #{} | Nome: {} | Tipo: {} | Saldo: {} | Negativa: {}",
            self.id,
            self.name,
            self.account_type.display_name(),
            self.balance,
            self.is_overdrawn(),
        )
    }
}
