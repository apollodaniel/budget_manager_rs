use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::App, events::Event};

pub mod categories_list_screen;
pub mod date_list_screen;
pub mod new_transaction_screen;



pub fn update(app: &mut App, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    match &input {
        Input { key: Key::Char('x'), ctrl: true, .. }=>app.quit(),
        _=>{}
    }


    match &mut app.app_state {
        crate::app::AppState::CategoriesList(s) => categories_list_screen::update(s,input, sender)?,
        crate::app::AppState::DateList(s) => date_list_screen::update(s,input, sender)?,
        crate::app::AppState::NewTransaction(s) => new_transaction_screen::update(s,input, sender)?,
        //crate::app::AppState::DateList(c) => update_transactions_date_list(app,input, c.clone())?,
        _=>{}
    }

    Ok(())
}