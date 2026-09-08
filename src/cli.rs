use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ledger", about = "CLI de ledger financeiro", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Operações em contas
    Account {
        #[command(subcommand)]
        action: AccountAction,
    },

    /// Operações em transações
    Tx {
        #[command(subcommand)]
        action: TxAction,
    },

    /// Listagens
    List {
        #[command(subcommand)]
        action: ListAction,
    },
}

#[derive(Subcommand)]
pub enum AccountAction {
    /// Cria uma nova conta
    Add {
        /// Nome da conta
        name: String,

        /// Tipo: checking | savings | credit | investment
        #[arg(default_value = "checking")]
        account_type: String,
    },

    /// Exibe detalhes de uma conta
    Show {
        /// ID da conta
        id: u64,
    },
}

#[derive(Subcommand)]
pub enum TxAction {
    /// Adiciona uma transação
    Add {
        #[arg(long)]
        account: u64,

        /// Tipo: credit | debit
        #[arg(long, default_value = "credit")]
        kind: String,

        #[arg(long)]
        amount: f64,

        #[arg(long)]
        desc: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ListAction {
    /// Lista todas as contas
    Accounts,

    /// Lista todas as transações
    Transactions,

    /// Resumo financeiro
    Summary,
}
