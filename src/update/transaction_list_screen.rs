use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::{date_list::DateListScreen, transactions_list::TransactionListScreen, ListScreen, ListingState, MoveListSelection}, events::Event, manager::{command_processing::process, Category}};



pub fn update(screen: &mut TransactionListScreen, input: &Input, sender: Sender<Event>)
->Result<(), Box<(dyn Error)>>{

    match &screen.listing_state {
        crate::app::ListingState::List => {

            match &input {
                Input { key: Key::Esc, .. } => {
                    sender.send(
                        Event::ChangeAppState(
                            crate::app::AppState::DateList(DateListScreen::new_with_selected(screen.category.clone(), screen.current_date.clone())?)
                        )
                    )?;
                }
                Input { key: Key::Char('l'), ctrl: true, .. } => {
                    screen.clear_input();
                    screen.search_transactions();
                },
                Input { key: Key::Up, .. }=> screen.move_list_selection(crate::app::MoveSelection::Up),
                Input { key: Key::Down, .. }=>screen.move_list_selection(crate::app::MoveSelection::Down),
                Input { key: Key::Enter,.. } => {
                    
                },
                Input { key: Key::Char('a'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Add);
                },
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Search);
                },
                Input { key: Key::Char('d'), ctrl: true, ..} => {
                    let selected_transaction = screen.get_selected_transaction();
                    if let Some(transaction) = selected_transaction {
                        process(crate::manager::BudgetCommand::DeleteTransaction(transaction))?;
                        let last_transaction = screen.transactions.len()==1;
                        screen.update_transactions()?;
                        if last_transaction{
                            sender.send(
                                Event::ChangeAppState(
                                    crate::app::AppState::DateList(DateListScreen::new(screen.category.clone())?)
                                )
                            )?;
                        }
                    }
                },
                _=>{}
            }

        },

        crate::app::ListingState::Search=>{
            match input {
                Input { key: Key::Esc, .. }=>{
                    screen.clear_input();
                    screen.change_listing_state(ListingState::List)
                },
                Input { key: Key::Enter, .. }=>{
                    screen.change_listing_state(ListingState::List)
                },
                input => {
                    screen.search_text_area.input(input.clone());
                    screen.search_transactions();
                }
            }
        },

        crate::app::ListingState::Add => {
            match input {
                Input { key: Key::Esc, .. }=>{
                    screen.clear_input();
                    screen.change_listing_state(ListingState::List)
                },
                Input { key: Key::Enter, .. }=>{
                    if let Some(input) = screen.add_text_area.lines().first(){
                        if !input.is_empty(){
                            process(
                                crate::manager::BudgetCommand::CreateCategory(
                                    Category::new(
                                        screen.add_text_area.lines().first().unwrap_or(&String::new()).to_string()
                                    )?
                                )
                            )?;
                            screen.update_transactions()?;
                            screen.clear_input();
                        }
                    }
                    screen.change_listing_state(ListingState::List);
                },
                input => {
                    screen.add_text_area.input(input.clone());
                }
            }
        }
    }
    

    Ok(())
}