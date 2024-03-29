use std::{collections::HashMap, error::Error, vec};

use ratatui::widgets::ListState;
use crate::manager::{command_processing::list_transaction, Category, Transaction};
use tui_textarea::TextArea;

use super::{transactions_list::TransactionHashmapValueError, App, ListScreen, ListingState, MoveListSelection};

#[derive(Debug, Clone)]
pub struct DateListScreen{
    pub date_list_state: ListState,
    pub listing_state: ListingState,
    pub search_text_area: TextArea<'static>,
    pub date_search: Vec<(String, bool)>,
    pub add_text_area: TextArea<'static>,

    pub category: Category,
    pub transactions: HashMap<String, Vec<Transaction>>    
}

impl DateListScreen {

    pub fn get_transactions_hashmaps(category: &Category) -> Result<HashMap<String, Vec<Transaction>>, Box<(dyn Error)>>{
        let transactions = list_transaction()?;
        let transactions = transactions.into_iter().filter(|f|f.category_id==category.category_id).map(|f|f.clone()).collect::<Vec<Transaction>>();
        
        let transactions_hashmap = DateListScreen::transaction_list_to_date_hashmap(transactions).ok_or(
            TransactionHashmapValueError{
                message: "unable to get transactions hashmap value".to_string()
            }
        )?;
        Ok(transactions_hashmap)
    }

    pub fn search_dates(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.date_search = self.transactions
            .keys()
            .filter(|f| f.to_lowercase().contains(query.as_str()))
            .map(|f|(f.clone(), false))
            .collect::<Vec<(String, bool)>>();
    }

    pub fn get_dates(transactions: Vec<Transaction>)->Option<Vec<String>>{
        let dates = Self::transaction_list_to_date_hashmap(transactions)?;
        let dates = dates.keys().map(|f|f.clone()).collect();
        Some(dates)
    }

    pub fn new(category: Category)->Result<Self, Box<(dyn Error)>>{
        let transactions = Self::get_transactions_hashmaps(&category)?;

        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Procurar",""),
            add_text_area: App::get_new_focused_text_area("Nova transação",""),
            listing_state: ListingState::List,
            date_search: transactions
            .keys()
            .map(|f|(f.clone(), false))
            .collect::<Vec<(String, bool)>>(),
            transactions: transactions,
            category: category,
            date_list_state: App::create_list_state(0),
        })
    }

    pub fn new_with_selected(category: Category, date: String)->Result<Self, Box<(dyn Error)>>{
        let transactions = Self::get_transactions_hashmaps(&category)?;
        let date_search = transactions
            .keys()
            .map(|f|(f.clone(), false))
            .collect::<Vec<(String, bool)>>();

        let index = date_search.iter().position(|f| f.0==date).unwrap_or(0);
        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Procurar",""),
            add_text_area: App::get_new_focused_text_area("Nova transação",""),
            listing_state: ListingState::List,
            date_search: date_search,
            transactions: transactions,
            category: category,
            date_list_state: App::create_list_state(index),
        })
    }

    pub fn get_selected_date(&self, single_selection: bool) -> Option<Vec<String>>{
        let selected_dates = self.date_search.iter().filter(|f|f.1).map(|f|f.0.clone()).collect::<Vec<String>>();
    
        if selected_dates.is_empty() || single_selection{
            let selected = self.date_list_state.selected()?;
            return Some(vec![self.date_search.get(selected)?.0.clone()]);  
        }else{
            return Some(selected_dates);  
        }
        
    }

    pub fn get_selected_date_index(&self) -> Option<usize>{
        let selected = self.date_list_state.selected()?;
        return Some(selected);  
    }

    pub fn update_dates(&mut self)->Result<(), Box<(dyn Error)>>{
        self.transactions = Self::get_transactions_hashmaps(&self.category)?;
        self.date_search = self.transactions.keys().map(|f|(f.clone(), false)).collect();
        Ok(())
    }

    pub fn transaction_list_to_date_hashmap(transactions: Vec<Transaction>)->Option<HashMap<String, Vec<Transaction>>>{
        
        let mut transactions_hashmap: HashMap<String, Vec<Transaction>> = HashMap::new();
        for transaction in transactions{
            let entry = transactions_hashmap.entry(transaction.get_date_formatted()?).or_insert(vec![]);
            entry.push(transaction);
        }

        Some(transactions_hashmap)
    }
}

impl MoveListSelection<String> for DateListScreen {
    fn move_list_selection(&mut self, move_selection: super::MoveSelection) {
        if !self.date_search.is_empty(){
            Self::move_list_selection_logic(move_selection,&mut self.date_list_state, &self.date_search.iter().map(|f|f.0.clone()).collect())
        }
    }
}

impl ListScreen for DateListScreen {
    fn change_listing_state(&mut self, listing_state: ListingState) {
        self.listing_state = listing_state;
    }
    fn clear_input(&mut self) {
        let listing_state = &mut self.listing_state;
        let add_text_area = &mut self.add_text_area;
        let search_text_area = &mut self.search_text_area;
        

        Self::clear_input_logic(listing_state, add_text_area, search_text_area);
        if let ListingState::Search = listing_state {
            self.search_dates();
        }
        //Self::clear_input_logic(&mut self.listing_state, &mut self.add_text_area, &mut self.search_text_area, ||{self.search_category()});
    }
}