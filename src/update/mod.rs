use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::App, events::Event};

pub mod categories_list_screen;

// pub fn update_transactions_date_list(app: &mut App, input: &Input, category: Category)->Result<(), Box<(dyn Error)>>{
//     match &app.listing_state {
//         crate::app::ListingState::List => {

//             match &input {
//                 Input { key: Key::Esc, .. } => {
//                     app.change_app_state(crate::app::AppState::CategoriesList);
//                 },
//                 Input { key: Key::Up, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Up),
//                 Input { key: Key::Down, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Down),
//                 Input { key: Key::Char('a'), ctrl: true, ..} => {
//                     app.change_listing_state(ListingState::Add);
//                 },
//                 Input { key: Key::Char('f'), ctrl: true, ..} => {
//                     app.change_listing_state(ListingState::Search);
//                 },
//                 Input { key: Key::Char('d'), ctrl: true, ..} => {
//                     let selected_category = app.get_selected_category();
//                     if let Some(category) = selected_category {
//                         process(crate::manager::BudgetCommand::DeleteCategory(category))?;
//                         app.update_categories()?;
//                     }
//                 },
//                 _=>{}
//             }

//         },

//         crate::app::ListingState::Search=>{
//             match input {
//                 Input { key: Key::Esc, .. }=>{
//                     app.clear_input();
//                     app.change_listing_state(ListingState::List)
//                 },
//                 Input { key: Key::Enter, .. }=>{
//                     app.change_listing_state(ListingState::List)
//                 },
//                 input => {
//                     app.search_text_area.input(input.clone());
//                     app.search_category();
//                 }
//             }
//         },

//         crate::app::ListingState::Add => {
//             match input {
//                 Input { key: Key::Esc, .. }=>{
//                     app.clear_input();
//                     app.change_listing_state(ListingState::List)
//                 },
//                 Input { key: Key::Enter, .. }=>{
//                     if let Some(input) = app.add_text_area.lines().first(){
//                         if !input.is_empty(){
//                             process(
//                                 crate::manager::BudgetCommand::CreateCategory(
//                                     Category::new(
//                                         app.add_text_area.lines().first().unwrap_or(&String::new()).to_string()
//                                     )?
//                                 )
//                             )?;
//                             app.update_categories()?;
//                             app.clear_input();
//                         }
//                     }
//                     app.change_listing_state(ListingState::List);
//                 },
//                 input => {
//                     app.add_text_area.input(input.clone());
//                 }
//             }
//         }
//     }
    
//     Ok(())
// }

// pub fn update_categories_list(app: &mut App, input: &Input)->Result<(), Box<(dyn Error)>>{
//     match &app.listing_state {
//         crate::app::ListingState::List => {

//             match &input {
//                 Input { key: Key::Esc, .. } => {
//                     app.clear_input();
//                 },
//                 Input { key: Key::Up, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Up),
//                 Input { key: Key::Down, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Down),
//                 Input { key: Key::Enter,.. } => {
//                     if let Some(t) = app.get_selected_category() {
                        
//                         app.change_app_state(crate::app::AppState::DateList(t));
//                     }
//                 },
//                 Input { key: Key::Char('a'), ctrl: true, ..} => {
//                     app.change_listing_state(ListingState::Add);
//                 },
//                 Input { key: Key::Char('f'), ctrl: true, ..} => {
//                     app.change_listing_state(ListingState::Search);
//                 },
//                 Input { key: Key::Char('d'), ctrl: true, ..} => {
//                     let selected_category = app.get_selected_category();
//                     if let Some(category) = selected_category {
//                         process(crate::manager::BudgetCommand::DeleteCategory(category))?;
//                         app.update_categories()?;
//                     }
//                 },
//                 _=>{}
//             }

//         },

//         crate::app::ListingState::Search=>{
//             match input {
//                 Input { key: Key::Esc, .. }=>{
//                     app.clear_input();
//                     app.change_listing_state(ListingState::List)
//                 },
//                 Input { key: Key::Enter, .. }=>{
//                     app.change_listing_state(ListingState::List)
//                 },
//                 input => {
//                     app.search_text_area.input(input.clone());
//                     app.search_category();
//                 }
//             }
//         },

//         crate::app::ListingState::Add => {
//             match input {
//                 Input { key: Key::Esc, .. }=>{
//                     app.clear_input();
//                     app.change_listing_state(ListingState::List)
//                 },
//                 Input { key: Key::Enter, .. }=>{
//                     if let Some(input) = app.add_text_area.lines().first(){
//                         if !input.is_empty(){
//                             process(
//                                 crate::manager::BudgetCommand::CreateCategory(
//                                     Category::new(
//                                         app.add_text_area.lines().first().unwrap_or(&String::new()).to_string()
//                                     )?
//                                 )
//                             )?;
//                             app.update_categories()?;
//                             app.clear_input();
//                         }
//                     }
//                     app.change_listing_state(ListingState::List);
//                 },
//                 input => {
//                     app.add_text_area.input(input.clone());
//                 }
//             }
//         }
//     }
    
//     Ok(())
// }

pub fn update(app: &mut App, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    match &input {
        Input { key: Key::Char('x'), ctrl: true, .. }=>app.quit(),
        _=>{}
    }


    match &mut app.app_state {
        crate::app::AppState::CategoriesList(s) => categories_list_screen::update(s,input, sender)?,
        //crate::app::AppState::DateList(c) => update_transactions_date_list(app,input, c.clone())?,
        _=>{}
    }

    Ok(())
}