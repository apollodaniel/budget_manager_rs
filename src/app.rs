use std::error::Error;


use ratatui::widgets::{Block, Borders, ListState};
use tui_textarea::TextArea;

use crate::manager::{command_processing::{list_categories, list_transaction, process}, Category, Transaction};

#[derive(Debug)]
pub enum AppState{
    
    CategoriesList,
    TransactionsList(Category),
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
pub struct App<'a>{
    pub app_state: AppState,
    pub listing_state: ListingState,
    pub text_area: TextArea<'a>,

    pub transactions: Vec<Transaction>,
    pub categories: Vec<Category>,
    
    pub transactions_search: Vec<String>,
    pub categories_search: Vec<String>,
    
    pub should_quit: bool,

    pub categories_list_state: ListState,
    pub transactions_list_state: ListState
}

pub enum MoveSelection{
    Up,
    Down
}

impl<'a> App<'a> {

    fn create_list_state()->ListState{
        ListState::default().with_selected(Some(0))
    }

    pub fn change_listing_state(&mut self, listing_state: ListingState){
        self.listing_state = listing_state;
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

    fn get_new_text_area()->TextArea<'a>{
        let text_area_block = Block::new().title("Search category").borders(Borders::all());
        let mut text_area = TextArea::default();
        text_area.set_block(text_area_block);
        text_area
    }
    pub fn clear_input(&mut self){
        self.text_area.delete_line_by_head();
    }
    pub fn search_category(&mut self){
        let query = self.text_area.lines().first().unwrap().to_lowercase();
        self.categories_search = self.categories.iter().filter(|f| f.name.to_lowercase().starts_with(query.as_str())).map(|f|f.name.clone()).collect();
    }
    pub fn search_transactions(&mut self){
        let query = self.text_area.lines().first().unwrap().to_lowercase();
        self.transactions_search = self.transactions.iter().filter(|f| f.description.to_lowercase().starts_with(query.as_str())).map(|f|f.description.clone()).collect();
    }

    pub fn new()->Result<Self, Box<(dyn Error)>>{
        let transactions = list_transaction()?;
        let categories = list_categories()?;

        Ok(Self {
            app_state: AppState::CategoriesList,
            listing_state: ListingState::Listing,
            text_area: App::get_new_text_area(),
            transactions_search: transactions.iter().map(|f|f.description.clone()).collect(),
            categories_search: categories.iter().map(|f|f.name.clone()).collect(),
            transactions: transactions,
            categories: categories,
            should_quit: false,
            categories_list_state: App::create_list_state(),
            transactions_list_state: App::create_list_state(),
        })
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }
}