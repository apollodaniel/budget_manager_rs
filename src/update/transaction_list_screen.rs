use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::{category_selection::CategorySelectionScreen, date_list::DateListScreen, new_transaction::{ParentScreen, NewTransactionScreen}, transactions_list::TransactionListScreen, ListScreen, ListingState, MoveListSelection}, events::Event, manager::{command_processing::process, Category}};



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
                    let selected_transaction = screen.get_selected_index();
                    if let Some(transaction) = selected_transaction {
                        screen.transactions_search[transaction].1 = !screen.transactions_search[transaction].1;
                    }

                },
                Input { key: Key::Char('a'), ctrl: true, ..} => {
                    //screen.change_listing_state(ListingState::Add);
                    sender.send(
                        Event::ChangeAppState(
                            crate::app::AppState::NewTransaction(
                                NewTransactionScreen::new(
                                ParentScreen::TransactionsList(screen.clone()),
                                    Some(screen.current_date.clone())
                                )
                            )
                        )
                    )?;
                },
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Search);
                },
                Input { key: Key::Char('c'), ctrl: true, ..} => {
                    let selected_transaction = screen.get_selected_transaction(false);
                    if let Some(transaction) = selected_transaction {
                        
                        sender.send(Event::ChangeAppState(
                            crate::app::AppState::ChangeCategory(
                                CategorySelectionScreen::new_with_selected(
                                    screen.category.clone(),
                                    transaction,
                                    ParentScreen::TransactionsList(screen.clone())
                                )?
                            ))
                        )?;

                    }
                },
                Input { key: Key::Char('d'), ctrl: true, ..} => {
                    let selected_transaction = screen.get_selected_transaction(false);
                    if let Some(transactions) = selected_transaction {
                        for transaction in transactions {
                            process(crate::manager::BudgetCommand::DeleteTransaction(transaction))?;
                        }
                        screen.update_transactions()?;
                        let last_transaction = screen.transactions.is_empty();

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