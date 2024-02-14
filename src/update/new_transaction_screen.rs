use std::{error::Error, sync::mpsc::Sender};
use crossterm::style::style;
use ratatui::style::Style;
use tui_textarea::{Input, Key};

use crate::{app::{App, NewTransaction::{NewTransactionParent, NewTransactionScreen, NewTransactionScreenFocus}}, events::Event};


pub fn update(screen: &mut NewTransactionScreen, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    
    match input {
        Input { key: Key::Up, .. } | Input { key: Key::Down, .. } =>{
            change_focus(screen.focus.clone(), screen)
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
        e=>{
            match screen.focus {
                NewTransactionScreenFocus::DateInput => {
                    screen.date_text_area.input(e.clone());
                },
                NewTransactionScreenFocus::DescriptionInput => {
                    screen.description_text_area.input(e.clone());
                }
            }
        }
    }


    Ok(())
}

pub fn change_focus(focus: NewTransactionScreenFocus, screen: &mut NewTransactionScreen ){
    match focus {
        NewTransactionScreenFocus::DateInput=>{
            screen.focus = NewTransactionScreenFocus::DescriptionInput;
            screen.description_text_area = App::get_new_focused_text_area("Description", screen.description_text_area.lines().first().unwrap());
            screen.date_text_area = App::get_new_text_area("Date separated by /", screen.date_text_area.lines().first().unwrap());
        },
        NewTransactionScreenFocus::DescriptionInput=>{
            screen.focus = NewTransactionScreenFocus::DateInput;   

            screen.date_text_area = App::get_new_focused_text_area("Date separated by /", screen.date_text_area.lines().first().unwrap());
            screen.description_text_area = App::get_new_text_area("Description", screen.description_text_area.lines().first().unwrap());

        }
    }
}