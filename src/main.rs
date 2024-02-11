use std::error::Error;

use manager::{command_processing::process, Transaction};



pub mod manager;

fn main() -> Result<(), Box<(dyn Error)>> {
    process(manager::BudgetCommand::CreateTransaction(Transaction::new(2.0, 1, "teste".to_string())?))?;

    Ok(())
}
