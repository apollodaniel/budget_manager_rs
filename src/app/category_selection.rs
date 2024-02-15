use std::error::Error;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;

use crate::manager::{command_processing::list_categories, Category, Transaction};

use super::{new_transaction::ParentScreen, App, ListScreen, ListingState, MoveListSelection, MoveSelection};



#[derive(Debug)]
pub struct CategorySelectionScreen{

    pub search_text_area: TextArea<'static>,
    pub categories: Vec<Category>,
    pub categories_search: Vec<Category>,
    pub categories_list_state: ListState,
    pub add_text_area: TextArea<'static>,
    pub transactions: Vec<Transaction>,
    pub parent: ParentScreen,

    pub listing_state: ListingState

}

impl CategorySelectionScreen{

    pub fn new(transactions: Vec<Transaction>, parent: ParentScreen)->Result<Self, Box<(dyn Error)>>{
        let categories = list_categories()?;

        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Search",""),
            add_text_area: App::get_new_focused_text_area("Add",""),
            categories_search: categories.clone(),
            categories_list_state: App::create_list_state(0),
            categories: categories,
            transactions: transactions,
            parent: parent,
            listing_state: ListingState::List
        })
    }

    pub fn new_with_selected(category: Category, transactions: Vec<Transaction>, parent: ParentScreen)->Result<Self, Box<(dyn Error)>>{
        let categories = list_categories()?;

        let mut index = 0;

        for c in &categories{
            if c.category_id == category.category_id{ 
                break;
            }
            index+=1;
        }

        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Search",""),
            add_text_area: App::get_new_focused_text_area("Add",""),
            categories_search: categories.clone(),
            categories_list_state: App::create_list_state(index),
            categories: categories,
            listing_state: ListingState::List,
            parent: parent,
            transactions: transactions
        })
    }

    pub fn update_categories(&mut self)->Result<(), Box<(dyn Error)>>{
        self.categories = list_categories()?;
        self.search_category();
        Ok(())
    }

    pub fn search_category(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.categories_search = self.categories.clone().into_iter().filter(|f| f.name.to_lowercase().contains(query.as_str())).collect::<Vec<Category>>();
    }

    

    pub fn get_selected_category(&self) -> Option<Category>{
        let selected = self.categories_list_state.selected();
        let id = self.categories_search.get(selected?)?.category_id;
        
        let mut category: Option<Category> = None;
        for _category in &self.categories{
            if _category.category_id == id{
                category = Some(_category.clone());
                break;
            }
        }
        return category;  
    }
}

impl MoveListSelection<Category> for CategorySelectionScreen {
    
    fn move_list_selection(&mut self, move_selection: MoveSelection) {
        if !self.categories_search.is_empty(){
            Self::move_list_selection_logic(move_selection,&mut self.categories_list_state, &self.categories);   
        }
    }
}

impl ListScreen for CategorySelectionScreen {
    fn change_listing_state(&mut self, listing_state: ListingState) {
        self.listing_state = listing_state;
    }
    fn clear_input(&mut self) {
        let listing_state = &mut self.listing_state;
        let add_text_area = &mut self.add_text_area;
        let search_text_area = &mut self.search_text_area;
        

        Self::clear_input_logic(listing_state, add_text_area, search_text_area);
        if let ListingState::Search = listing_state {
            self.search_category();
        }
        //Self::clear_input_logic(&mut self.listing_state, &mut self.add_text_area, &mut self.search_text_area, ||{self.search_category()});
    }
}