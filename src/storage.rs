// src/storage.rs

use crate::errors::LedgerError;
use crate::models::account::Account;
use crate::models::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Snapshot completo do estado do ledger — é isso que vai para o disco.
#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerState {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
    pub next_account_id: u64,
    pub next_transaction_id: u64,
}

const DB_PATH: &str = "ledger.json";

pub fn save(state: &LedgerState) -> Result<(), LedgerError> {
    let json = serde_json::to_string_pretty(state).map_err(|e| LedgerError::InvalidOperation {
        reason: format!("erro ao serializar: {}", e),
    })?;

    fs::write(DB_PATH, json).map_err(|e| LedgerError::InvalidOperation {
        reason: format!("erro ao salvar arquivo: {}", e),
    })?;

    Ok(())
}

pub fn load() -> Result<LedgerState, LedgerError> {
    if !Path::new(DB_PATH).exists() {
        // primeira execução — estado vazio
        return Ok(LedgerState {
            accounts: Vec::new(),
            transactions: Vec::new(),
            next_account_id: 1,
            next_transaction_id: 1,
        });
    }

    let json = fs::read_to_string(DB_PATH).map_err(|e| LedgerError::InvalidOperation {
        reason: format!("erro ao ler arquivo: {}", e),
    })?;

    serde_json::from_str(&json).map_err(|e| LedgerError::InvalidOperation {
        reason: format!("erro ao deserializar: {}", e),
    })
}
