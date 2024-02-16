use std::error::Error;

use ratatui::widgets::ListState;
use tui_textarea::TextArea;

use crate::manager::{command_processing::list_categories, Category};

use super::{App, ListScreen, ListingState, MoveListSelection, MoveSelection};

#[derive(Debug)]
pub struct CategoryListScreen{

    pub search_text_area: TextArea<'static>,
    pub categories: Vec<Category>,
    pub categories_search: Vec<(Category, bool)>,
    pub categories_list_state: ListState,
    pub add_text_area: TextArea<'static>,

    pub listing_state: ListingState

}

impl CategoryListScreen{

    pub fn new()->Result<Self, Box<(dyn Error)>>{
        let categories = list_categories()?;

        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Procurar",""),
            add_text_area: App::get_new_focused_text_area("Nova categoria",""),
            categories_search: categories.iter().map(|f|(f.clone(), false)).collect::<Vec<(Category, bool)>>(),
            categories_list_state: App::create_list_state(0),
            categories: categories,
            listing_state: ListingState::List
        })
    }

    pub fn new_with_selected(category: Category)->Result<Self, Box<(dyn Error)>>{
        let categories = list_categories()?;

        let mut index = 0;

        for c in &categories{
            if c.category_id == category.category_id{ 
                break;
            }
            index+=1;
        }

        Ok(Self { 
            search_text_area: App::get_new_focused_text_area("Procurar",""),
            add_text_area: App::get_new_focused_text_area("Nova categoria",""),
            categories_search: categories.iter().map(|f|(f.clone(), false)).collect::<Vec<(Category, bool)>>(),
            categories_list_state: App::create_list_state(index),

            categories: categories,
            listing_state: ListingState::List
        })
    }

    pub fn update_categories(&mut self)->Result<(), Box<(dyn Error)>>{
        self.categories = list_categories()?;
        self.search_category();
        Ok(())
    }

    pub fn search_category(&mut self){
        let query = self.search_text_area.lines().first().unwrap().to_lowercase();
        self.categories_search = self.categories.iter().filter(|f| f.name.to_lowercase().contains(query.as_str())).map(|f|(f.clone(), false)).collect::<Vec<(Category, bool)>>();
    }

    pub fn get_selected_category_index(&self) -> Option<usize>{
        let selected = self.categories_list_state.selected();
        let id = self.categories_search.get(selected?)?.0.category_id;
        
        let mut category: Option<usize> = None;
        for (index, _category) in self.categories.iter().enumerate(){
            if _category.category_id == id{
                category = Some(index);
                break;
            }
        }

        return category;
    }

    pub fn get_selected_category(&self, single_selection: bool) -> Option<Vec<Category>>{
        let selected_categories = self.categories_search.iter()
            .filter(|f| f.1)
            .map(|f| f.0.clone()).collect::<Vec<Category>>();

        if selected_categories.is_empty() || single_selection{
            let selected = self.categories_list_state.selected();
            let id = self.categories_search.get(selected?)?.0.category_id;
            
            let mut category: Option<Vec<Category>> = None;
            for _category in &self.categories{
                if _category.category_id == id{
                    category = Some(vec![_category.clone()]);
                    break;
                }
            }
            return category;  
        }else{
            return Some(selected_categories);
        }
    }
}

impl MoveListSelection<Category> for CategoryListScreen {
    
    fn move_list_selection(&mut self, move_selection: MoveSelection) {
        if !self.categories_search.is_empty(){
            Self::move_list_selection_logic(move_selection,&mut self.categories_list_state, &self.categories);   
        }
    }
}

impl ListScreen for CategoryListScreen {
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