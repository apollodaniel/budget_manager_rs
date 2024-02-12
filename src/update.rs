use std::error::Error;

use crossterm::event::KeyEvent;

use crate::app::App;


pub fn update_categories_list(app: &mut App, key: &KeyEvent)->Result<(), Box<(dyn Error)>>{
    if let crate::app::AppState::CategoriesList(e) = &app.app_state {
        match e {
            crate::app::ListingState::Listing => {

                match key.code {
                    crossterm::event::KeyCode::Up=>app.move_categories_list_selection(crate::app::MoveSelection::Up),
                    crossterm::event::KeyCode::Down=>app.move_categories_list_selection(crate::app::MoveSelection::Down),
                    _=>{}
                }

            },
            crate::app::ListingState::Searching=>{

            }
        }
    }
    
    Ok(())
}

pub fn update(app: &mut App, key: &KeyEvent)->Result<(), Box<(dyn Error)>>{
    match &key.code {
        crossterm::event::KeyCode::Esc=>app.quit(),
        _=>{}
    }

    match &app.app_state {
        crate::app::AppState::CategoriesList(_) => update_categories_list(app,key)?,
        _=>{}
    }

    Ok(())
}