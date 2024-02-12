use std::{collections::HashMap, error::Error, time::{Instant, SystemTime}};


use ratatui::widgets::{Block, Borders, ListState};
use tui_textarea::TextArea;

use crate::manager::{command_processing::{list_categories, list_transaction, process}, Category, Transaction};

#[derive(Debug)]
pub enum AppState{
    
    CategoriesList,
    TransactionDateList(Category),
    TransactionsList(u32),
    ChangeCategory(Transaction),
    NewTransaction(Category),
    NewCategory,
    
}

#[derive(Debug)]
pub enum ListingState{
    List,
    Search,
    Add
}

#[derive(Debug)]
pub struct App<'a>{
    pub app_state: AppState,
    pub listing_state: ListingState,
    pub search_text_area: TextArea<'a>,
    pub add_text_area: TextArea<'a>,

    pub transactions: HashMap<String, Vec<Transaction>>,
    pub categories: Vec<Category>,
    
    pub transactions_search: Vec<Transaction>,
    pub transactions_date_search: Vec<String>,
    pub categories_search: Vec<Category>,
    
    pub should_quit: bool,

    pub categories_list_state: ListState,
    pub date_list_state: ListState,
    pub transactions_list_state: ListState
}

pub enum MoveSelection{
    Up,
    Down
}

impl<'a> App<'a> {

    pub fn get_selected_category(&self) -> Option<Category>{
        let selected = self.categories_list_state.selected();
        if let Some(selected) = selected {
            let id = self.categories_search[selected].category_id;

            let mut category: Option<Category> = None;
            for _category in &self.categories{
                if _category.category_id == id{
                    category = Some(_category.clone());
                    break;
                }
            }
            return category;  
        }else{
            return None;
        }
    }

    pub fn get_selected_date(&self) -> Option<String>{
        let selected = self.categories_list_state.selected()?;
        return Some(self.transactions_date_search[selected].clone());  
    }

    pub fn get_transactions_by_date(&self, date: String)-> &Vec<Transaction>{
        &self.transactions[&date]
    }

    pub fn get_category_transactions_date(transactions: Vec<Transaction>)->Option<HashMap<String, Vec<Transaction>>>{
        
        let mut transactions_hashmap: HashMap<String, Vec<Transaction>> = HashMap::new();
        for transaction in transactions{
            let entry = transactions_hashmap.entry(transaction.get_date_formatted()?).or_insert(vec![]);
            entry.push(transaction);
        }

        Some(transactions_hashmap)
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

    pub fn update_categories(&mut self)->Result<(), Box<(dyn Error)>>{
        self.categories = list_categories()?;
        self.search_category();
        Ok(())
    }
    pub fn update_transactions(&mut self)->Result<(), Box<(dyn Error)>>{
        self.transactions = Self::get_category_transactions_date(list_transaction()?).unwrap_or_default();
        self.search_transactions();
        Ok(())
    }

    fn create_list_state()->ListState{
        ListState::default().with_selected(Some(0))
    }

    pub fn change_listing_state(&mut self, listing_state: ListingState){
        self.listing_state = listing_state;
    }

    pub fn change_app_state(&mut self, state: AppState){
        self.app_state = state;
    }

    pub fn move_categories_list_selection(&mut self, move_selection: MoveSelection){
        match move_selection {
            MoveSelection::Up => {
                if let Some(selected) = self.categories_list_state.selected() {
                    if let Some(res) = selected.checked_sub(1){
                        self.categories_list_state.select(Some(res));
                    }else{
                        self.categories_list_state.select(Some(self.categories.len()-1));
                    }
                }
            }
            MoveSelection::Down => {
                if let Some(selected) = self.categories_list_state.selected() {
                    if selected + 1 >= self.categories.len(){
                        self.categories_list_state.select(Some(0));
                    }else{
                        self.categories_list_state.select(Some(selected+1));
                    }
                }
                
            },
        }
    }

    pub fn move_transactions_list_selection(&mut self, move_selection: MoveSelection){
        match move_selection {
            MoveSelection::Up => {
                if let Some(selected) = self.transactions_list_state.selected() {
                    if selected + 1 >= self.transactions.len(){
                        self.transactions_list_state.select(Some(0));
                    }else{
                        self.transactions_list_state.select(Some(selected+1));
                    }
                }
            }
            MoveSelection::Down => {
                if let Some(selected) = self.transactions_list_state.selected() {
                    if let Some(res) = selected.checked_sub(1){
                        self.transactions_list_state.select(Some(res));
                    }else{
                        self.transactions_list_state.select(Some(self.transactions.len()-1));
                    }
                }
            },
        }
    }

    fn get_new_text_area(label: &'a str)->TextArea<'a>{
        let text_area_block = Block::new().title(label).borders(Borders::all());
        let mut text_area = TextArea::default();
        text_area.set_block(text_area_block);
        text_area
    }
    pub fn clear_input(&mut self){
        match self.listing_state {
            ListingState::Add => {
                self.add_text_area.delete_line_by_head();
            },
            ListingState::Search => {
                self.search_text_area.delete_line_by_head();
                match self.app_state {
                    AppState::CategoriesList => self.search_category(),
                    AppState::TransactionsList(_) => self.search_transactions(),
                    _=>{}
                }
            },
            ListingState::List=>{
                self.add_text_area.delete_line_by_head();
                self.search_text_area.delete_line_by_head();
            }
        }
    }
    pub fn search_category(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.categories_search = self.categories.clone().into_iter().filter(|f| f.name.to_lowercase().contains(query.as_str())).collect::<Vec<Category>>();
    }
    pub fn search_transactions(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        let transactions = self.get_transactions_by_date(self.get_selected_date().unwrap_or_default()).clone();
        self.transactions_search = transactions.into_iter().filter(|f| f.description.to_lowercase().contains(query.as_str())).collect::<Vec<Transaction>>();
    }

    pub fn new()->Result<Self, Box<(dyn Error)>>{
        let transactions_list = list_transaction()?;
        let categories = list_categories()?;

        let transactions = Self::get_category_transactions_date(list_transaction()?).unwrap_or_default();

        Ok(Self {
            app_state: AppState::CategoriesList,
            listing_state: ListingState::List,
            search_text_area: App::get_new_text_area("Search"),
            add_text_area: App::get_new_text_area("Add"),
            transactions_search: transactions_list.clone(),
            categories_search: categories.clone(),
            transactions_date_search: transactions.keys().map(|f|f.clone()).collect::<Vec<String>>(),
            transactions: transactions,
            categories: categories,
            should_quit: false,
            date_list_state: App::create_list_state(),
            categories_list_state: App::create_list_state(),
            transactions_list_state: App::create_list_state(),
        })
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }
}