use core::fmt;
use std::error::Error;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;
use crate::manager::{command_processing::list_transaction, Category, Transaction};
use super::{date_list::DateListScreen, App, ListScreen, ListingState, MoveListSelection, MoveSelection};


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

#[derive(Debug, Clone)]
pub struct TransactionListScreen{
    pub search_text_area: TextArea<'static>,
    pub listing_state: ListingState,
    pub add_text_area: TextArea<'static>,

    pub transactions: Vec<Transaction>,
    pub transactions_search: Vec<(Transaction, bool)>,
    pub transactions_list_state: ListState,

    pub category: Category,
    pub current_date: String
}

impl TransactionListScreen{
    
    pub fn new<'a>(category: &'a Category, date: &'a String)->Result<Self, Box<(dyn Error)>>{
        let transactions = Self::get_transactions(&category, &date)?;
        
        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Procurar",""),
            add_text_area: App::get_new_focused_text_area("Nova transação",""),
            transactions_search: transactions.iter().map(|f|(f.clone(), false)).collect::<Vec<(Transaction, bool)>>(),
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
        Ok(transactions_hashmap.get(date).unwrap_or(&vec![]).clone())
    }

    pub fn get_selected_index(&self) -> Option<usize>{
        let selected = self.transactions_list_state.selected();
        if let Some(selected) = selected {
            let id = self.transactions_search[selected].0.id;

            let mut transaction: Option<usize> = None;
            for (index, _transaction) in self.transactions_search.iter().enumerate(){
                if _transaction.0.id == id{
                    transaction = Some(index);
                    break;
                }
            }
            return transaction;  
        }else{
            return None;
        }
    }

    pub fn get_selected_transaction(&self, single_selection: bool) -> Option<Vec<Transaction>>{
        let selected_transactions = self.transactions_search.iter().filter(|f|f.1).map(|f|f.0.clone()).collect::<Vec<Transaction>>();

        if selected_transactions.is_empty() || single_selection{
            let selected = self.transactions_list_state.selected();
            if let Some(selected) = selected {
                let id = self.transactions_search[selected].0.id;
    
                let mut transaction: Option<Vec<Transaction>> = None;
                for _transaction in &self.transactions_search{
                    if _transaction.0.id == id{
                        transaction = Some(vec![_transaction.0.clone()]);
                        break;
                    }
                }
                return transaction;  
            }else{
                return None;
            }
        }else{
            return Some(selected_transactions);
        }
    }

    pub fn update_transactions(&mut self)->Result<(), Box<(dyn Error)>>{
        self.transactions = Self::get_transactions(&self.category, &self.current_date)?;
        self.search_transactions();
        Ok(())
    }

    pub fn search_transactions(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.transactions_search = self.transactions.iter().filter(|f| f.description.to_lowercase().contains(query.as_str())).map(|f|(f.clone(), false)).collect::<Vec<(Transaction,bool)>>();
    }


}

impl MoveListSelection<Transaction> for TransactionListScreen {
    
    fn move_list_selection(&mut self, move_selection: MoveSelection) {
        if !self.transactions_search.is_empty(){
            Self::move_list_selection_logic(move_selection,&mut self.transactions_list_state, &self.transactions);   
        }
    }
}

impl ListScreen for TransactionListScreen {
    fn change_listing_state(&mut self, listing_state: ListingState) {
        self.listing_state = listing_state;
    }
    fn clear_input(&mut self) {
        Self::clear_input_logic(&mut self.listing_state, &mut self.add_text_area, &mut self.search_text_area)
    }
}