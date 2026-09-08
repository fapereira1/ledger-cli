// src/errors.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("conta não encontrada: id {id}")]
    AccountNotFound { id: u64 },

    #[error("valor da transação não pode ser zero")]
    ZeroAmount,

    #[error(
        "saldo insuficiente na conta {account_id}: disponível {available}, necessário {required}"
    )]
    InsufficientFunds {
        account_id: u64,
        available: i64,
        required: i64,
    },

    #[error("operação inválida: {reason}")]
    InvalidOperation { reason: String },
}
