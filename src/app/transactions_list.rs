use core::fmt;
use std::error::Error;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;
use crate::manager::{command_processing::list_transaction, Category, Transaction};
use super::{App, date_list::DateListScreen, ListingState};


#[derive(Debug)]
pub struct TransactionHashmapValueError {
    pub message: String,
}

impl fmt::Display for TransactionHashmapValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for TransactionHashmapValueError {}

#[derive(Debug)]
pub struct TransactionListScreen{
    pub search_text_area: TextArea<'static>,
    pub listing_state: ListingState,
    pub add_text_area: TextArea<'static>,

    pub transactions: Vec<Transaction>,
    pub transactions_search: Vec<Transaction>,
    pub transactions_list_state: ListState,

    pub category: Category,
    pub current_date: String
}

impl TransactionListScreen{
    
    pub fn new<'a>(category: &'a Category, date: &'a String)->Result<Self, Box<(dyn Error)>>{
        let transactions = Self::get_transactions(&category, &date)?;
        
        Ok(Self { 
            search_text_area: App::get_new_text_area("Search"),
            add_text_area: App::get_new_text_area("Add"),
            transactions_search: transactions.clone(),
            transactions_list_state: App::create_list_state(0),
            transactions: transactions,
            current_date: date.clone(),
            category: category.clone(),
            listing_state: ListingState::List
        })
    }

    pub fn get_transactions(category: &Category, date: &String) -> Result<Vec<Transaction>, Box<(dyn Error)>>{
        let transactions = list_transaction()?;
        let transactions = transactions.into_iter().filter(|f|f.category_id==category.category_id).map(|f|f.clone()).collect::<Vec<Transaction>>();
        
        let transactions_hashmap = DateListScreen::transaction_list_to_date_hashmap(transactions).ok_or(
            TransactionHashmapValueError{
                message: "unable to get transactions hashmap value".to_string()
            }
        )?;
                //&self.transactions[&date]
        Ok(transactions_hashmap[date].clone())
    }

    pub fn get_selected_transaction(&self) -> Option<Transaction>{
        let selected = self.transactions_list_state.selected();
        if let Some(selected) = selected {
            let id = self.transactions_search[selected].id;

            let mut transaction: Option<Transaction> = None;
            for _transaction in &self.transactions_search{
                if _transaction.id == id{
                    transaction = Some(_transaction.clone());
                    break;
                }
            }
            return transaction;  
        }else{
            return None;
        }
    }

    pub fn update_transactions(&mut self)->Result<(), Box<(dyn Error)>>{
        self.transactions = Self::get_transactions(&self.category, &self.current_date)?;
        self.search_transactions();
        Ok(())
    }

    pub fn search_transactions(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.transactions_search = self.transactions.clone().into_iter().filter(|f| f.description.to_lowercase().contains(query.as_str())).collect::<Vec<Transaction>>();
    }

}