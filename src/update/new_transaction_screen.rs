use std::{error::Error, str::FromStr, sync::mpsc::Sender};
use chrono::Utc;
use tui_textarea::{Input, Key};

use crate::{app::{App, new_transaction::{NewTransactionParent, NewTransactionScreen, NewTransactionScreenFocus}}, events::Event};


pub fn update(screen: &mut NewTransactionScreen, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    
    match input {
        Input { key: Key::Up, .. } =>{
            let new_focus = NewTransactionScreenFocus::get_new_focus(screen.focus.clone(), crate::app::MoveSelection::Up);
            focus_on(new_focus, screen);
        },
        Input { key: Key::Down, .. } =>{
            let new_focus = NewTransactionScreenFocus::get_new_focus(screen.focus.clone(), crate::app::MoveSelection::Down);
            focus_on(new_focus, screen);
        },
        Input { key: Key::Esc, .. }=>{
            match &screen.parent {
                NewTransactionParent::DateList(e) => {
                    sender.send(Event::ChangeAppState(crate::app::AppState::DateList(e.clone())))?;
                },
                NewTransactionParent::TransactionsList(e) => {
                    sender.send(Event::ChangeAppState(crate::app::AppState::TransactionsList(e.clone())))?;
                },
            }
        },
        Input { key: Key::Enter, ..} =>{
            let description: String = screen.description_text_area.lines().first().unwrap().to_string();
            let date: String = screen.date_text_area.lines().first().unwrap().to_string();
            let date_timestamp = date_to_timestamp(date.clone());
            if let Ok(_timestamp) = date_timestamp {
                if !description.is_empty(){
                    //let transaction = Transaction::new(amount, category_id, description, date)
                }
            }

        },
        e=>{
            match screen.focus {
                NewTransactionScreenFocus::DateInput => {
                    screen.date_text_area.input(e.clone());
                },
                NewTransactionScreenFocus::DescriptionInput => {
                    screen.description_text_area.input(e.clone());
                },
                NewTransactionScreenFocus::AmountInput =>{
                    screen.amount_text_area.input(e.clone());
                }
            }
        }
    }


    Ok(())
}

pub fn date_to_timestamp(date: String) -> Result<i64, Box<(dyn Error)>>{
    Ok(chrono::DateTime::<Utc>::from_str(&date)?.timestamp_millis())
}
pub fn unfocus_all(screen: &mut NewTransactionScreen,){
    screen.date_text_area = App::get_new_text_area("Date separated by /", screen.date_text_area.lines().first().unwrap());
    screen.description_text_area = App::get_new_text_area("Description", screen.description_text_area.lines().first().unwrap());
    screen.amount_text_area = App::get_new_text_area("Amount", screen.amount_text_area.lines().first().unwrap());
}

pub fn focus_on(focus: NewTransactionScreenFocus,screen: &mut NewTransactionScreen){
    unfocus_all(screen);
    match focus {
        NewTransactionScreenFocus::DescriptionInput=>{
            screen.description_text_area = App::get_new_focused_text_area("Description", screen.description_text_area.lines().first().unwrap());
        },
        NewTransactionScreenFocus::DateInput=>{
            screen.date_text_area = App::get_new_focused_text_area("Date separated by /", screen.date_text_area.lines().first().unwrap());
        },
        NewTransactionScreenFocus::AmountInput =>{
            screen.amount_text_area = App::get_new_focused_text_area("Amount", screen.amount_text_area.lines().first().unwrap());
        }
    }
    screen.focus = focus;
}
