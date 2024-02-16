use std::{error::Error, sync::mpsc::Sender};
use tui_textarea::{Input, Key};

use crate::{app::{new_transaction::{NewTransactionScreen, NewTransactionScreenFocus, ParentScreen}, App, StringToFloat}, events::Event, manager::{command_processing::process, Transaction}};


pub fn update(screen: &mut NewTransactionScreen, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    
    match input {
        Input { key: Key::Up, .. } =>{
            let new_focus = NewTransactionScreenFocus::get_new_focus(
                screen.focus.clone(),
                crate::app::MoveSelection::Up,
                screen.check_contains_date()
            );
            focus_on(new_focus, screen);
        },
        Input { key: Key::Down, .. } =>{
            let new_focus = NewTransactionScreenFocus::get_new_focus(
                screen.focus.clone(),
                crate::app::MoveSelection::Down,
                screen.check_contains_date()
            );
            focus_on(new_focus, screen);
        },
        Input { key: Key::Esc, .. }=>{
            match screen.error {
                Some(_)=>{
                    screen.error = None;
                },
                None=>{
                    send_exit_event(screen, sender)?;
                }
            }
        },
        Input { key: Key::Enter, ..} =>{
            let description: String = screen.description_text_area.lines().first().unwrap().to_string();
            let amount: String = screen.amount_text_area.lines().first().unwrap().to_string();
            let date: String = if let Some(date) = screen.date.clone() {date}else{screen.date_text_area.lines().first().unwrap().to_string()};

            let date_timestamp = date_to_timestamp(date.clone());

            match date_timestamp {
                Ok(timestamp) => {
                    if !description.is_empty() {
                        //let transaction = Transaction::new(amount, category_id, description, date)
                        if !amount.is_empty() {
                            //let transaction = Transaction::new(amount, category_id, description, date)
                            let amount_parsed = amount.to_float();
                            if let Ok(amount) = amount_parsed {
                                match &mut screen.parent {
                                    ParentScreen::DateList(e) => {
                                        create_transaction(amount, description, timestamp, e.category.category_id)?;
                                        e.update_dates()?;
                                    },
                                    ParentScreen::TransactionsList(e) => {
                                        create_transaction(amount, description, timestamp, e.category.category_id)?;
                                        e.update_transactions()?;
                                    },
                                };
                                send_exit_event(screen, sender)?;

                            }else{
                                screen.error = Some(String::from("Valor inválido.\nTente algo como: 12,50"));
                            }
                        }else{
                            screen.error = Some(String::from("O valor da transação não pode estar vázio"));
                        }
                    }else{
                        screen.error = Some(String::from("A descrição não pode estar vázia"));
                    }
                },
                Err(e)=>{
                    screen.error = Some(format!("Data inválida: {}.\n{}", date, e.to_string()));
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

pub fn send_exit_event(screen: &mut NewTransactionScreen, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    match &screen.parent {
        ParentScreen::DateList(e) => {
            sender.send(Event::ChangeAppState(crate::app::AppState::DateList(e.clone())))?;
        },
        ParentScreen::TransactionsList(e) => {
            sender.send(Event::ChangeAppState(crate::app::AppState::TransactionsList(e.clone())))?;
        },
    }
    Ok(())
}

pub fn create_transaction(amount: f64, description: String, timestamp: i64,category_id: u32)->Result<(), Box<(dyn Error)>>{
    let transaction = Transaction::new(amount, category_id, description, Some(timestamp))?;
    process(crate::manager::BudgetCommand::CreateTransaction(transaction))?;
    Ok(())
}

pub fn date_to_timestamp(date: String) -> Result<i64, Box<(dyn Error)>>{
    let format_str = "%d/%m/%Y";
    let named_format_str = "%e %b %Y";

    let datetime = chrono::NaiveDate::parse_from_str(&date, format_str);
    
    let datetime = match datetime {
        Ok(e) => {e},
        Err(_) => chrono::NaiveDate::parse_from_str(&date, named_format_str)?
    };
    
    let datetime = datetime.and_hms_opt(0, 0, 0).unwrap();

    Ok(datetime.timestamp_millis())
}
pub fn unfocus_all(screen: &mut NewTransactionScreen,){
    screen.date_text_area = App::get_new_text_area("Data dd/mm/yyyy", screen.date_text_area.lines().first().unwrap());
    screen.description_text_area = App::get_new_text_area("Descrição", screen.description_text_area.lines().first().unwrap());
    screen.amount_text_area = App::get_new_text_area("Valor", screen.amount_text_area.lines().first().unwrap());
}

pub fn focus_on(focus: NewTransactionScreenFocus,screen: &mut NewTransactionScreen){
    unfocus_all(screen);
    match focus {
        NewTransactionScreenFocus::DescriptionInput=>{
            screen.description_text_area = App::get_new_focused_text_area("Descrição", screen.description_text_area.lines().first().unwrap());
        },
        NewTransactionScreenFocus::DateInput=>{
            screen.date_text_area = App::get_new_focused_text_area("Data dd/mm/yyyy", screen.date_text_area.lines().first().unwrap());
        },
        NewTransactionScreenFocus::AmountInput =>{
            screen.amount_text_area = App::get_new_focused_text_area("Valor", screen.amount_text_area.lines().first().unwrap());
        }
    }
    screen.focus = focus;
}
