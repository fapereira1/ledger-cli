// src/main.rs

use clap::Parser;
use ledger::cli::{AccountAction, Cli, Command, ListAction, TxAction};
use ledger::ledger::Ledger;
use ledger::models::account::AccountType;
use ledger::models::currency::Amount;
use ledger::models::reportable::Reportable;
use ledger::models::transaction::TransactionType;
use ledger::storage;

fn parse_account_type(s: &str) -> AccountType {
    match s.to_lowercase().as_str() {
        "savings" => AccountType::Savings,
        "credit" => AccountType::Credit,
        "investment" => AccountType::Investment,
        _ => AccountType::Checking,
    }
}

fn main() {
    // carrega estado do disco — ou começa vazio
    let state = storage::load().unwrap_or_else(|e| {
        eprintln!("⚠️  erro ao carregar estado: {}", e);
        std::process::exit(1);
    });

    let mut book = Ledger::from_state(state);
    let cli = Cli::parse();
    let mut dirty = false; // só salva se houve mudança

    match cli.command {
        Command::Account { action } => match action {
            AccountAction::Add { name, account_type } => {
                let tipo = parse_account_type(&account_type);
                let id = book.create_account(name, tipo);
                println!("✅ conta criada com id {}", id);
                dirty = true;
            }
            AccountAction::Show { id } => match book.get_account(id) {
                Ok(acc) => println!("{}", acc.detail()),
                Err(err) => println!("❌ {}", err),
            },
        },

        Command::Tx { action } => match action {
            TxAction::Add {
                account,
                kind,
                amount,
                desc,
            } => {
                let tx_type = match kind.to_lowercase().as_str() {
                    "debit" => TransactionType::Debit {
                        account_id: account,
                    },
                    _ => TransactionType::Credit {
                        account_id: account,
                    },
                };
                match book.add_transaction(tx_type, Amount::from_reais(amount), desc) {
                    Ok(id) => {
                        println!("✅ transação criada com id {}", id);
                        dirty = true;
                    }
                    Err(err) => println!("❌ {}", err),
                }
            }
        },

        Command::List { action } => match action {
            ListAction::Accounts => {
                println!("=== Contas ===");
                for acc in book.list_accounts() {
                    println!("{}", acc);
                }
            }
            ListAction::Transactions => {
                println!("=== Transações ===");
                for tx in book.list_transactions() {
                    println!("{}", tx);
                }
            }
            ListAction::Summary => {
                println!("=== Resumo ===");
                println!("contas:        {}", book.list_accounts().len());
                println!("transações:    {}", book.list_transactions().len());
                println!("volume total:  {}", book.total_volume());
            }
        },
    }

    // persiste apenas se houve alteração
    if dirty {
        if let Err(e) = storage::save(&book.to_state()) {
            eprintln!("⚠️  erro ao salvar estado: {}", e);
        }
    }
}
