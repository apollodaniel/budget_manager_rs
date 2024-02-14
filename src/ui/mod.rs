pub mod categories_list_screen;
pub mod date_list_screen;
pub mod new_transaction_screen;

use std::error::Error;


use crate::{app::App, events::CrosstermTerminal};


pub fn draw(terminal: &mut CrosstermTerminal, app: &mut App)->Result<(), Box<(dyn Error)>>{
    match &mut app.app_state {
        crate::app::AppState::CategoriesList(e) => categories_list_screen::draw(e,terminal)?,
        crate::app::AppState::DateList(e) => date_list_screen::draw(e,terminal)?,
        crate::app::AppState::NewTransaction(e) => new_transaction_screen::draw(e,terminal)?,
        //crate::app::AppState::DateList(_) => draw_date_list(app,terminal)?,
        _=>{}
    }

    Ok(())

}