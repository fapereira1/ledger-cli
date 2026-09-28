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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::reportable::Reportable;

    // helper — evita repetir Account::new em cada teste
    fn conta_corrente() -> Account {
        Account::new(1, String::from("Conta Corrente"), AccountType::Checking)
    }

    fn conta_credito() -> Account {
        Account::new(2, String::from("Nubank"), AccountType::Credit)
    }

    // --- construção ---

    #[test]
    fn new_define_id_corretamente() {
        let account = conta_corrente();
        assert_eq!(account.id, 1);
    }

    #[test]
    fn new_define_nome_corretamente() {
        let account = conta_corrente();
        assert_eq!(account.name, "Conta Corrente");
    }

    #[test]
    fn new_inicializa_saldo_zerado() {
        let account = conta_corrente();
        assert_eq!(account.balance, 0.0);
    }

    // --- is_overdrawn ---

    #[test]
    fn is_overdrawn_com_saldo_zero() {
        let account = conta_corrente();
        assert!(!account.is_overdrawn());
    }

    #[test]
    fn is_overdrawn_com_saldo_positivo() {
        let mut account = conta_corrente();
        account.balance = 100.0;
        assert!(!account.is_overdrawn());
    }

    #[test]
    fn is_overdrawn_com_saldo_negativo() {
        let mut account = conta_corrente();
        account.balance = -0.01;
        assert!(account.is_overdrawn());
    }

    // --- AccountType::display_name ---

    #[test]
    fn display_name_checking() {
        assert_eq!(AccountType::Checking.display_name(), "Conta Corrente");
    }

    #[test]
    fn display_name_savings() {
        assert_eq!(AccountType::Savings.display_name(), "Poupança");
    }

    #[test]
    fn display_name_de_conta_credito() {
        let account = conta_credito();
        assert_eq!(account.account_type.display_name(), "Crédito");
    }

    #[test]
    fn display_name_investment() {
        assert_eq!(AccountType::Investment.display_name(), "Investimento");
    }

    // --- AccountType::allows_negative_balance ---

    #[test]
    fn checking_nao_permite_saldo_negativo() {
        assert!(!AccountType::Checking.allows_negative_balance());
    }

    #[test]
    fn savings_nao_permite_saldo_negativo() {
        assert!(!AccountType::Savings.allows_negative_balance());
    }

    #[test]
    fn credit_permite_saldo_negativo() {
        assert!(AccountType::Credit.allows_negative_balance());
    }

    #[test]
    fn investment_nao_permite_saldo_negativo() {
        assert!(!AccountType::Investment.allows_negative_balance());
    }

    // --- Display ---

    #[test]
    fn display_formata_corretamente() {
        let account = conta_corrente();
        let output = format!("{}", account);
        assert_eq!(output, "[1] Conta Corrente | Conta Corrente | saldo: 0");
    }

    // --- Reportable ---

    #[test]
    fn summary_contem_nome_e_tipo() {
        let account = conta_corrente();
        let summary = account.summary();
        assert!(summary.contains("Conta Corrente"));
        assert!(summary.contains("Conta Corrente")); // display_name do tipo
    }

    #[test]
    fn detail_contem_id() {
        let account = conta_corrente();
        assert!(account.detail().contains("1"));
    }

    #[test]
    fn detail_contem_status_negativo() {
        let mut account = conta_corrente();
        account.balance = -50.0;
        assert!(account.detail().contains("true")); // is_overdrawn
    }

    // --- PartialEq em AccountType ---

    #[test]
    fn account_types_iguais() {
        assert_eq!(AccountType::Checking, AccountType::Checking);
    }

    #[test]
    fn account_types_diferentes() {
        assert_ne!(AccountType::Checking, AccountType::Credit);
    }
}
