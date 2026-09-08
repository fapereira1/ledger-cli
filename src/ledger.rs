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
