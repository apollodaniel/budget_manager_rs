use std::error::Error;

use crossterm::event::KeyEvent;
use ratatui::widgets::{Block, Borders};
use tui_textarea::{Input, Key, TextArea};

use crate::{app::{App, ListingState}, manager::{command_processing::process, Category}};


pub fn update_categories_list(app: &mut App, input: &Input)->Result<(), Box<(dyn Error)>>{
    match &app.listing_state {
        crate::app::ListingState::List => {

            match &input {
                Input { key: Key::Esc, .. } => {
                    app.clear_input();
                },
                Input { key: Key::Up, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Up),
                Input { key: Key::Down, .. }=>app.move_categories_list_selection(crate::app::MoveSelection::Down),
                Input { key: Key::Char('a'), ctrl: true, ..} => {
                    app.change_listing_state(ListingState::Add);
                },
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    app.change_listing_state(ListingState::Search);
                }
                _=>{}
            }

        },

        crate::app::ListingState::Search=>{
            match input {
                Input { key: Key::Esc, .. }=>{
                    app.clear_input();
                    app.change_listing_state(ListingState::List)
                },
                Input { key: Key::Enter, .. }=>{
                    app.change_listing_state(ListingState::List)
                },
                input => {
                    app.search_text_area.input(input.clone());
                    app.search_category();
                }
            }
        },

        crate::app::ListingState::Add => {
            match input {
                Input { key: Key::Esc, .. }=>{
                    app.clear_input();
                    app.change_listing_state(ListingState::List)
                },
                Input { key: Key::Enter, .. }=>{
                    process(
                        crate::manager::BudgetCommand::CreateCategory(
                            Category::new(
                                app.add_text_area.lines().first().unwrap_or(&String::new()).to_string()
                            )?
                        )
                    )?;
                    app.update_categories()?;
                    app.change_listing_state(ListingState::List);
                },
                input => {
                    app.add_text_area.input(input.clone());
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