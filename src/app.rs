use std::error::Error;


use ratatui::widgets::ListState;

use crate::manager::{command_processing::{list_categories, list_transaction, process}, Category, Transaction};

#[derive(Debug)]
pub enum AppState{
    
    CategoriesList(ListingState),
    TransactionsList(Category, ListingState),
    ChangeCategory(Transaction),
    NewTransaction(Category),
    NewCategory,
    
}

#[derive(Debug)]
pub enum ListingState{
    Listing,
    Searching
}

#[derive(Debug)]
pub struct App{
    pub app_state: AppState,
    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
    pub should_quit: bool,

    pub categories_list_state: ListState,
    pub transactions_list_state: ListState
}

pub enum MoveSelection{
    Up,
    Down
}

impl App {

    fn create_list_state()->ListState{
        ListState::default().with_selected(Some(0))
    }

    pub fn move_categories_list_selection(&mut self, move_selection: MoveSelection){
        match move_selection {
            MoveSelection::Up => {
                if let Some(selected) = self.categories_list_state.selected() {
                    if selected + 1 >= self.categories.len(){
                        self.categories_list_state.select(Some(0));
                    }else{
                        self.categories_list_state.select(Some(selected+1));
                    }
                }
            }
            MoveSelection::Down => {
                if let Some(selected) = self.categories_list_state.selected() {
                    if let Some(res) = selected.checked_sub(1){
                        self.categories_list_state.select(Some(res));
                    }else{
                        self.categories_list_state.select(Some(self.categories.len()-1));
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

    pub fn new()->Result<Self, Box<(dyn Error)>>{
        let transactions = list_transaction()?;
        let categories = list_categories()?;

        Ok(Self {
            app_state: AppState::CategoriesList(ListingState::Listing),
            transactions: transactions,
            categories: categories,
            should_quit: false,
            categories_list_state: App::create_list_state(),
            transactions_list_state: App::create_list_state()
        })
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }
}