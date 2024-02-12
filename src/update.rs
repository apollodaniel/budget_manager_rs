use std::error::Error;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders};
use tui_textarea::{Input, Key, TextArea};

use crate::app::{App, ListingState};


pub fn update_categories_list(app: &mut App, input: &Input)->Result<(), Box<(dyn Error)>>{
    match &app.listing_state {
        crate::app::ListingState::Listing => {

            match &input {
                Input { key: Key::Esc, .. } => {
                    app.clear_input();
                    app.search_category();
                },
                Input { key: Key::Up, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Up),
                Input { key: Key::Down, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Down),
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    
                    app.change_listing_state(ListingState::Searching);
                }
                _=>{}
            }

        },
        crate::app::ListingState::Searching=>{
            match input {
                Input { key: Key::Esc, .. }=>{
                    app.clear_input();
                    app.search_category();
                    app.change_listing_state(ListingState::Listing)
                },
                Input { key: Key::Enter, .. }=>{
                    app.change_listing_state(ListingState::Listing)
                },
                input => {
                    app.text_area.input(input.clone());
                    app.search_category();
                }
            }
        }
    }
    
    Ok(())
}

pub fn update(app: &mut App, input: &Input)->Result<(), Box<(dyn Error)>>{
    match &input {
        Input { key: Key::Char('x'), ctrl: true, .. }=>app.quit(),
        _=>{}
    }

    match &app.app_state {
        crate::app::AppState::CategoriesList => update_categories_list(app,input)?,
        _=>{}
    }

    Ok(())
}