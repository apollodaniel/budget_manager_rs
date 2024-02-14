use std::error::Error;


use ratatui::{style::{Style, Stylize}, text, widgets::{Block, Borders, ListState, Padding}};
use tui_textarea::TextArea;

use crate::manager::{Category, Transaction};

use self::{categories_list::CategoryListScreen, date_list::DateListScreen, transactions_list::TransactionListScreen, NewTransaction::NewTransactionScreen};

pub mod categories_list;
pub mod date_list;
pub mod transactions_list;
pub mod NewTransaction;


#[derive(Debug)]
pub enum AppState{
    CategoriesList(CategoryListScreen),
    DateList(DateListScreen),
    TransactionsList(TransactionListScreen),
    ChangeCategory(Transaction),
    NewTransaction(NewTransactionScreen),
    NewCategory,
    
}

#[derive(Debug, Clone)]
pub enum ListingState{
    List,
    Search,
    Add
}

pub enum MoveSelection{
    Up,
    Down
}


pub trait MoveListSelection<T> {

    fn move_list_selection(&mut self, move_selection: MoveSelection);

    fn move_list_selection_logic(move_selection: MoveSelection, list_state: &mut ListState, items: &Vec<T>) {
        match move_selection {
            MoveSelection::Up => {
                if let Some(selected) = list_state.selected() {
                    if let Some(res) = selected.checked_sub(1) {
                        list_state.select(Some(res));
                    } else {
                        list_state.select(Some(items.len() - 1));
                    }
                }    
            }
            MoveSelection::Down => {
                if let Some(selected) = list_state.selected() {
                    if selected + 1 >= items.len() {
                        list_state.select(Some(0));
                    } else {
                        list_state.select(Some(selected + 1));
                    }
                }
            }
        }
    }
}

pub trait ListScreen{

    fn clear_input(&mut self);

    fn clear_input_logic(
        listing_state: &mut ListingState,
        add_text_area: &mut TextArea<'_>,
        search_text_area: &mut TextArea<'_>,
    )    
    {

        match listing_state {
            ListingState::Add => {
                add_text_area.delete_line_by_head();
            },
            ListingState::Search => {
                search_text_area.delete_line_by_head();
            },
            ListingState::List=>{
                add_text_area.delete_line_by_head();
                search_text_area.delete_line_by_head();
            }
        }
    }

    fn change_listing_state(&mut self, listing_state: ListingState);


}


#[derive(Debug)]
pub struct App{
    pub app_state: AppState,    
    pub should_quit: bool,

}

impl App {


    fn create_list_state(selected: usize)->ListState{
        ListState::default().with_selected(Some(selected))
    }

    pub fn change_app_state(&mut self, state: AppState){
        self.app_state = state;
    }

    pub fn get_new_text_area<'b>(label: &'b str, input: &str)->TextArea<'b>{
        let text_area_block = Block::new().title(label).borders(Borders::ALL).border_type(ratatui::widgets::BorderType::Plain);
        let mut text_area = TextArea::default();
        text_area.set_cursor_style(Style::new().black());
        text_area.set_block(text_area_block);
        text_area.insert_str(input);
        text_area
    }

    pub fn get_new_focused_text_area<'b>(label: &'b str, input: &str)->TextArea<'b>{
        let text_area_block = Block::new().title(label).white().borders(Borders::all()).border_type(ratatui::widgets::BorderType::Thick);
        let mut text_area = TextArea::default();
        text_area.set_block(text_area_block);
        text_area.insert_str(input);
        text_area
    }

    pub fn new()->Result<Self, Box<(dyn Error)>>{

        Ok(Self {
            app_state: AppState::CategoriesList(CategoryListScreen::new()?),
            should_quit: false,
        })
    }

    pub fn quit(&mut self){
        self.should_quit = true;
    }
}




