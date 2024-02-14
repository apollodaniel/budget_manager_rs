use std::{error::Error, fmt::Display, fs::{create_dir, File}, str::FromStr, time::{Instant, UNIX_EPOCH}};


use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};

use self::command_processing::{get_new_category_id, get_new_transaction_id};


fn connect_db()->Result<Connection, Box<dyn Error>>{
    let home_dir = simple_home_dir::home_dir().expect("unable to get home dir");
    let app_dir = home_dir.join(".budget_manager_rs");
    if !app_dir.exists(){
        create_dir(&app_dir)?;
    }

    let database_location = app_dir.join("budget_manager.db");

    if !database_location.exists(){
        File::create(&database_location)?;
        
        let connection = rusqlite::Connection::open(database_location)?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS categories (category_id INTEGER PRIMARY KEY,name TEXT NOT NULL)", [])?;
            
            connection.execute("INSERT INTO categories (category_id,name) values (1,'General')", [])?;
            
            connection.execute("CREATE TABLE IF NOT EXISTS transaction_event (transaction_id INTEGER PRIMARY KEY,amount REAL NOT NULL,category_id INTEGER NOT NULL,timestamp INTEGER NOT NULL,description TEXT)", [])?;
            
            Ok(connection)
        }else{    
        let connection = rusqlite::Connection::open(database_location)?;
        Ok(connection)
    }
    
}

pub mod command_processing{
    use std::error::Error;
    use super::*;

    pub fn get_new_category_id() -> Result<u32, Box<(dyn Error)>>{
        let mut categories = list_categories()?;
        categories.sort_by(|a,b|{
            a.category_id.cmp(&b.category_id)
        });
        Ok(if categories.is_empty() {1}else{categories.last().unwrap().category_id+1})
    }

    pub fn get_new_transaction_id()->Result<u32, Box<(dyn Error)>>{
        let mut transactions = list_transaction()?;
        transactions.sort_by(|a,b|{
            a.id.cmp(&b.id)
        });
        Ok(if transactions.is_empty() {1}else{transactions.last().unwrap().id+1})
    }

    pub fn list_transaction()->Result<Vec<Transaction>, Box<(dyn Error)>>{
        let connection = connect_db()?;

        let mut stmt = connection.prepare("SELECT * FROM transaction_event;")?;

        let transactions = stmt.query_and_then([], |row|{
            Transaction::from_row(row)
        })?;

        let mut transactions_vec = Vec::new();
        for transaction in transactions{
            transactions_vec.push(transaction?);
        }


        Ok(transactions_vec)
    }

    pub fn list_categories()->Result<Vec<Category>, Box<(dyn Error)>>{
        let connection = connect_db()?;
        let mut stmt = connection.prepare("SELECT * FROM categories")?;

        let categories = stmt.query_map([], |row|Category::from_row(row))?;

        let mut categories_vec = Vec::new();
        for transaction in categories{
            categories_vec.push(transaction?);
        }

        Ok(categories_vec)
    }

    pub fn process(command: BudgetCommand) -> Result<(), Box<(dyn Error)>>{
        let connection = connect_db()?;
        
        match command {
            // transactions
            BudgetCommand::CreateTransaction(t) => connection.execute(&t.to_sql_insert(), [])?,
            BudgetCommand::DeleteTransaction(t) => connection.execute(&t.to_sql_delete(), [])?,
            BudgetCommand::UpdateTransaction(t,o) => connection.execute(&t.update(o), [])?,
            // categories
            BudgetCommand::CreateCategory(c) => connection.execute(&c.to_sql_insert(), [])?,
            BudgetCommand::DeleteCategory(c) => connection.execute(&c.to_sql_delete(), [])?,
            BudgetCommand::UpdateCategory(c, o) => connection.execute(&c.update(&o), [])?,
        };

        Ok(())
    }
}


#[derive(Debug)]
pub struct InstantSubError{
    message: String
}

impl Display for InstantSubError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InstantSubError {}

#[derive(Debug, Clone)]
pub struct Transaction{
    pub id: u32,
    pub amount: f64,
    pub category_id: u32,
    pub description: String,
    pub timestamp: i64,
}

impl Transaction {
    fn from_row(row: &Row)->Result<Self, rusqlite::Error>{
        Ok(Self { 
            id: row.get(0)?,
            amount: row.get(1)?,
            category_id: row.get(2)?,
            description: row.get(4)?,
            timestamp: row.get(3)?
        })
    }

    pub fn get_date_formatted(&self)->Option<String>{
        let datetime = chrono::DateTime::from_timestamp_millis(self.timestamp);
        Some(datetime?.format("%e %b %Y").to_string())
    }

    pub fn new(amount: f64, category_id: u32, description: String, date: Option<String>) -> Result<Self, Box<(dyn Error)>>{
        let id = get_new_transaction_id()?;
        match date {
            Some(e) => {
                let datetime: DateTime<Utc> = chrono::DateTime::from_str(&e)?;

                Ok(
                    Self { 
                        id: id,
                        amount: amount,
                        category_id: category_id,
                        description: description,
                        timestamp: datetime.timestamp_millis()
                    }
                )
            },
            None =>{
                let error = InstantSubError{message: "unable to get current time".to_string()};
                Ok(Self { 
                    id: id,
                    amount: amount,
                    category_id: category_id,
                    description: description,
                    timestamp: Instant::now().checked_sub(UNIX_EPOCH.elapsed()?).ok_or(error)?.elapsed().as_secs() as i64
                })

            }
        }
    }

    fn to_sql_insert(&self) -> String{
        format!("INSERT INTO transaction_event (transaction_id,amount, category_id, timestamp, description) values ({},{}, {}, {}, '{}')", self.id, self.amount, self.category_id, self.timestamp, self.description)
    }
    fn to_sql_delete(&self) -> String{
        format!("DELETE FROM transaction_event WHERE transaction_id={}", self.id)
    }
    fn update(&mut self, other: &Self)->String{
        self.amount = other.amount;
        self.category_id = other.category_id;
        self.description = other.description.clone();
        self.timestamp = other.timestamp;

        format!("UPDATE transaction_event SET amount={}, category_id={}, timestamp={}, description='{}' WHERE transaction_id={}", self.amount, self.category_id, self.timestamp, self.description, self.id)
    }

}

#[derive(Debug, Clone)]
pub struct Category{
    pub category_id: u32,
    pub name: String
}

impl Category {
    fn from_row(row: &Row)->Result<Self, rusqlite::Error>{
        Ok(Self { category_id: row.get(0)?, name: row.get(1)? })
    }

    pub fn new(name: String) -> Result<Self, Box<(dyn Error)>>{
        let id = get_new_category_id()?;
        Ok(Self { category_id: id, name: name })
    }

    fn to_sql_insert(&self) -> String{
        format!("INSERT INTO categories (category_id, name) values ({},'{}')", self.category_id, self.name)
    }
    fn to_sql_delete(&self) -> String{
        format!("DELETE FROM categories WHERE category_id={}", self.category_id)
    }
    fn update(&mut self, other: &Self)->String{
        self.name = other.name.clone();

        format!("UPDATE categories SET name='{}' WHERE category_id={}", self.name, self.category_id)
    }
}

#[derive(Debug)]
pub enum BudgetCommand<'a>{
    // creation
    CreateTransaction(Transaction),
    CreateCategory(Category),
    // delete
    DeleteTransaction(Transaction),
    DeleteCategory(Category),
    // update
    UpdateTransaction(&'a mut Transaction, &'a Transaction),
    UpdateCategory(&'a mut Category, &'a Category),
}