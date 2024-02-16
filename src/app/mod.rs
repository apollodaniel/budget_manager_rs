use std::{error::Error, fmt::Display};


use ratatui::{style::{Style, Stylize}, widgets::{Block, Borders, ListState}};
use tui_textarea::TextArea;


use self::{categories_list::CategoryListScreen, category_selection::CategorySelectionScreen, date_list::DateListScreen, new_transaction::NewTransactionScreen, transactions_list::TransactionListScreen};

pub mod categories_list;
pub mod date_list;
pub mod transactions_list;
pub mod new_transaction;
pub mod category_selection;


#[derive(Debug)]
pub struct InvalidNumberError;

impl Display for InvalidNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unable to convert\ninvalid number")
    }
}

impl Error for InvalidNumberError {
    
}

pub trait StringToFloat{
    fn check_symbol(f: String) -> bool;
    fn check_valid(input: String, symbols: Vec<String>)->bool;
    fn to_float(&self)->Result<f64, Box<(dyn Error)>>;
}

impl StringToFloat for String {
    fn check_valid(input: String, mut symbols: Vec<String>)->bool {
        if symbols.len()<2{
            return true;
        }
        // let mut words = Vec::<Vec<String>>::new();
        // let mut temp = Vec::<String>::new();
        // for word in input.split("") {
        //     if Self::check_symbol(word.to_string()){
        //         words.push(temp.clone());
        //         temp.clear();
        //     }else{
        //         temp.push(word.to_string());
        //     }
        // }

        let mut is_valid: bool = true;
        
        let mut result = input.split(|c| {
            Self::check_symbol(format!("{}", c))
        } ).collect::<Vec<&str>>();
        result.remove(result.len()-1);

        if result.len() != symbols.len()/2 {
            symbols.remove(0);
            for r in result{
                if r.len()<3{
                    is_valid = false;
                }
            }
        }

        // for word in &words{
        //     if word.len()<3 && words.len()!=1{
        //         is_valid = false;
        //     }
        // }

        is_valid
    }
    fn check_symbol(f: String) -> bool {
        f.eq(&".") || f.eq(&",") || f.eq(&"'") || f.eq(&" ")
    }
    fn to_float(&self)->Result<f64, Box<(dyn Error)>> {
        let input = self;
        let result = input.split("").filter(|f|
            Self::check_symbol(f.to_string())
        ).map(|f|f.to_string()).collect::<Vec<String>>();
    
        let mut non_repeated_symbols: Vec<String> = result.clone();
        non_repeated_symbols.dedup();
    
        if non_repeated_symbols.len() > 2 || !Self::check_valid(input.clone(), result.clone()){
            return Err(Box::new(InvalidNumberError{}));
        }
    
        if non_repeated_symbols.len()==1 && !result.is_empty(){
            let input = input.replace(result.first().unwrap(), ".");
            return Ok(input.parse::<f64>()?);
        }else if result.len()>1{
            let input = input.replace(result.first().unwrap(), "");
            let input = if !result.last().unwrap().eq("."){
                input.replace(result.last().unwrap(), ".")
            }else{
                input
            };
    
            return Ok(input.parse::<f64>()?);
        }else{
            return Ok(input.parse::<f64>()?);
        }

    }
}

#[derive(Debug)]
pub enum AppState{
    CategoriesList(CategoryListScreen),
    DateList(DateListScreen),
    TransactionsList(TransactionListScreen),
    ChangeCategory(CategorySelectionScreen),
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




