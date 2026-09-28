// src/ledger.rs

use crate::errors::LedgerError;
use crate::models::account::{Account, AccountType};
use crate::models::currency::Amount;
use crate::models::transaction::{Transaction, TransactionType};
use crate::storage::LedgerState;
use std::collections::HashMap;

pub struct Ledger {
    accounts: HashMap<u64, Account>,
    transactions: Vec<Transaction>,
    next_account_id: u64,
    next_transaction_id: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: Vec::new(),
            next_account_id: 1,
            next_transaction_id: 1,
        }
    }

    // ------------------------------------------------------------------
    // Contas
    // ------------------------------------------------------------------

    pub fn create_account(&mut self, name: String, account_type: AccountType) -> u64 {
        let id = self.next_account_id;
        let account = Account::new(id, name, account_type);
        self.accounts.insert(id, account);
        self.next_account_id += 1;
        id
    }

    pub fn get_account(&self, id: u64) -> Result<&Account, LedgerError> {
        self.accounts
            .get(&id)
            .ok_or(LedgerError::AccountNotFound { id })
    }

    pub fn list_accounts(&self) -> Vec<&Account> {
        let mut accounts: Vec<&Account> = self.accounts.values().collect();
        accounts.sort_by_key(|a| a.id);
        accounts
    }

    // ------------------------------------------------------------------
    // Transações
    // ------------------------------------------------------------------

    pub fn add_transaction(
        &mut self,
        transaction_type: TransactionType,
        amount: Amount,
        description: Option<String>,
    ) -> Result<u64, LedgerError> {
        // valida: valor não pode ser zero
        if amount.is_zero() {
            return Err(LedgerError::ZeroAmount);
        }

        // valida: contas referenciadas em transferências devem existir
        if let TransactionType::Credit { account_id } = &transaction_type {
            if !self.accounts.contains_key(account_id) {
                return Err(LedgerError::AccountNotFound { id: *account_id });
            }
        }
        if let TransactionType::Debit { account_id } = &transaction_type {
            if !self.accounts.contains_key(account_id) {
                return Err(LedgerError::AccountNotFound { id: *account_id });
            }
        }
        if let TransactionType::Transfer { from_id, to_id } = &transaction_type {
            if !self.accounts.contains_key(from_id) {
                return Err(LedgerError::AccountNotFound { id: *from_id });
            }
            if !self.accounts.contains_key(to_id) {
                return Err(LedgerError::AccountNotFound { id: *to_id });
            }
        }

        let id = self.next_transaction_id;
        let tx = Transaction::new(id, transaction_type, amount, description);
        self.transactions.push(tx);
        self.next_transaction_id += 1;

        Ok(id)
    }

    pub fn list_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    pub fn list_credits(&self) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.transaction_type.is_credit())
            .collect()
    }

    pub fn list_debits(&self) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.transaction_type.is_debit())
            .collect()
    }

    pub fn list_transfers(&self) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.transaction_type.is_transfer())
            .collect()
    }

    pub fn total_credits(&self, account_id: u64) -> Amount {
        self.transactions
            .iter()
            .filter(|tx| match &tx.transaction_type {
                TransactionType::Credit { account_id: id } => *id == account_id,
                TransactionType::Transfer { to_id, .. } => *to_id == account_id,
                _ => false,
            })
            .map(|tx| tx.amount)
            .sum()
    }

    pub fn total_debits(&self, account_id: u64) -> Amount {
        self.transactions
            .iter()
            .filter(|tx| match &tx.transaction_type {
                TransactionType::Debit { account_id: id } => *id == account_id,
                TransactionType::Transfer { from_id, .. } => *from_id == account_id,
                _ => false,
            })
            .map(|tx| tx.amount)
            .sum()
    }

    /// Retorna todas as transações de um tipo específico.
    pub fn transactions_by_type(&self, transaction_type: &TransactionType) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| &tx.transaction_type == transaction_type)
            .collect()
    }

    /// Retorna as N transações mais recentes.
    pub fn latest_transactions(&self, n: usize) -> Vec<&Transaction> {
        self.transactions.iter().rev().take(n).collect()
    }

    /// Retorna transações acima de um valor mínimo.
    pub fn transactions_above(&self, minimum: Amount) -> Vec<&Transaction> {
        self.transactions
            .iter()
            .filter(|tx| tx.amount > minimum)
            .collect()
    }

    /// Valor total movimentado.
    pub fn total_volume(&self) -> Amount {
        self.transactions.iter().map(|tx| tx.amount).sum()
    }

    /// Constrói um Ledger a partir de um estado salvo.
    pub fn from_state(state: LedgerState) -> Self {
        let accounts = state.accounts.into_iter().map(|a| (a.id, a)).collect();

        Self {
            accounts,
            transactions: state.transactions,
            next_account_id: state.next_account_id,
            next_transaction_id: state.next_transaction_id,
        }
    }

    /// Exporta o estado atual para ser salvo.
    pub fn to_state(&self) -> LedgerState {
        LedgerState {
            accounts: self.accounts.values().cloned().collect(),
            transactions: self.transactions.clone(),
            next_account_id: self.next_account_id,
            next_transaction_id: self.next_transaction_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::account::AccountType;
    use crate::models::currency::Amount;
    use crate::models::transaction::TransactionType;

    // --- helper — monta um Ledger com duas contas e algumas transações ---

    fn ledger_com_dados() -> (Ledger, u64, u64) {
        let mut ledger = Ledger::new();
        let id_corrente =
            ledger.create_account(String::from("Conta Corrente"), AccountType::Checking);
        let id_nubank = ledger.create_account(String::from("Nubank"), AccountType::Credit);

        ledger
            .add_transaction(
                TransactionType::Credit {
                    account_id: id_corrente,
                },
                Amount::from_reais(5000.00),
                Some(String::from("Salário")),
            )
            .unwrap();

        ledger
            .add_transaction(
                TransactionType::Debit {
                    account_id: id_corrente,
                },
                Amount::from_reais(1200.00),
                Some(String::from("Aluguel")),
            )
            .unwrap();

        ledger
            .add_transaction(
                TransactionType::Transfer {
                    from_id: id_corrente,
                    to_id: id_nubank,
                },
                Amount::from_reais(500.00),
                Some(String::from("Fatura")),
            )
            .unwrap();

        (ledger, id_corrente, id_nubank)
    }

    // --- create_account ---

    #[test]
    fn create_account_retorna_id_sequencial() {
        let mut ledger = Ledger::new();
        let id1 = ledger.create_account(String::from("A"), AccountType::Checking);
        let id2 = ledger.create_account(String::from("B"), AccountType::Checking);
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn create_account_persiste_na_lista() {
        let mut ledger = Ledger::new();
        ledger.create_account(String::from("Poupança"), AccountType::Savings);
        assert_eq!(ledger.list_accounts().len(), 1);
    }

    // --- get_account ---

    #[test]
    fn get_account_retorna_conta_existente() {
        let (ledger, id_corrente, _) = ledger_com_dados();
        let account = ledger.get_account(id_corrente).unwrap();
        assert_eq!(account.name, "Conta Corrente");
    }

    #[test]
    fn get_account_retorna_erro_para_id_inexistente() {
        let ledger = Ledger::new();
        let result = ledger.get_account(99);
        assert!(result.is_err());
    }

    // --- add_transaction ---

    #[test]
    fn add_transaction_retorna_id_sequencial() {
        let (ledger, _, _) = ledger_com_dados();
        assert_eq!(ledger.list_transactions().len(), 3);
    }

    #[test]
    fn add_transaction_rejeita_valor_zero() {
        let mut ledger = Ledger::new();
        let id = ledger.create_account(String::from("A"), AccountType::Checking);
        let result = ledger.add_transaction(
            TransactionType::Credit { account_id: id },
            Amount::from_cents(0),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn add_transaction_rejeita_conta_inexistente() {
        let mut ledger = Ledger::new();
        let result = ledger.add_transaction(
            TransactionType::Credit { account_id: 99 },
            Amount::from_reais(100.00),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn add_transaction_rejeita_transfer_com_origem_inexistente() {
        let mut ledger = Ledger::new();
        let id_nubank = ledger.create_account(String::from("Nubank"), AccountType::Credit);
        let result = ledger.add_transaction(
            TransactionType::Transfer {
                from_id: 99,
                to_id: id_nubank,
            },
            Amount::from_reais(100.00),
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn add_transaction_rejeita_transfer_com_destino_inexistente() {
        let mut ledger = Ledger::new();
        let id_corrente = ledger.create_account(String::from("Corrente"), AccountType::Checking);
        let result = ledger.add_transaction(
            TransactionType::Transfer {
                from_id: id_corrente,
                to_id: 99,
            },
            Amount::from_reais(100.00),
            None,
        );
        assert!(result.is_err());
    }

    // --- cálculos ---

    #[test]
    fn total_credits_soma_apenas_creditos_da_conta() {
        let (ledger, id_corrente, _) = ledger_com_dados();
        assert_eq!(ledger.total_credits(id_corrente).cents(), 500_000);
    }

    #[test]
    fn total_debits_inclui_transferencia_saindo() {
        let (ledger, id_corrente, _) = ledger_com_dados();
        // débito direto (1200) + transferência saindo (500) = 1700
        assert_eq!(ledger.total_debits(id_corrente).cents(), 170_000);
    }

    #[test]
    fn total_credits_nubank_inclui_transferencia_recebida() {
        let (ledger, _, id_nubank) = ledger_com_dados();
        assert_eq!(ledger.total_credits(id_nubank).cents(), 50_000);
    }

    #[test]
    fn total_volume_soma_todas_as_transacoes() {
        let (ledger, _, _) = ledger_com_dados();
        // 5000 + 1200 + 500 = 6700
        assert_eq!(ledger.total_volume().cents(), 670_000);
    }

    // --- listagens ---

    #[test]
    fn list_credits_retorna_apenas_creditos() {
        let (ledger, _, _) = ledger_com_dados();
        let credits = ledger.list_credits();
        assert_eq!(credits.len(), 1);
        assert!(credits[0].transaction_type.is_credit());
    }

    #[test]
    fn list_debits_retorna_apenas_debitos() {
        let (ledger, _, _) = ledger_com_dados();
        let debits = ledger.list_debits();
        assert_eq!(debits.len(), 1);
        assert!(debits[0].transaction_type.is_debit());
    }

    #[test]
    fn list_transfers_retorna_apenas_transferencias() {
        let (ledger, _, _) = ledger_com_dados();
        let transfers = ledger.list_transfers();
        assert_eq!(transfers.len(), 1);
        assert!(transfers[0].transaction_type.is_transfer());
    }

    #[test]
    fn latest_transactions_limita_quantidade() {
        let (ledger, _, _) = ledger_com_dados();
        assert_eq!(ledger.latest_transactions(2).len(), 2);
    }

    #[test]
    fn latest_transactions_ordem_reversa() {
        let (ledger, _, _) = ledger_com_dados();
        let latest = ledger.latest_transactions(1);
        // a última adicionada foi a transferência (id 3)
        assert!(latest[0].transaction_type.is_transfer());
    }

    #[test]
    fn transactions_above_filtra_por_valor_minimo() {
        let (ledger, _, _) = ledger_com_dados();
        // acima de R$ 1000: salário (5000) e aluguel (1200)
        let resultado = ledger.transactions_above(Amount::from_reais(1000.00));
        assert_eq!(resultado.len(), 2);
    }
}
