use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::{category_selection::CategorySelectionScreen, date_list::DateListScreen, transactions_list::TransactionListScreen, ListScreen, ListingState, MoveListSelection}, events::Event, manager::{command_processing::process, Category, Transaction}};



pub fn update(screen: &mut CategorySelectionScreen, input: &Input, sender: Sender<Event>)
->Result<(), Box<(dyn Error)>>{
    match &screen.listing_state {
        crate::app::ListingState::List => {

            match &input {
                Input { key: Key::Char('l'), ctrl: true, .. } => {
                    screen.clear_input();
                    screen.search_category();
                },
                Input { key: Key::Up, .. }=> screen.move_list_selection(crate::app::MoveSelection::Up),
                Input { key: Key::Down, .. }=>screen.move_list_selection(crate::app::MoveSelection::Down),
                Input { key: Key::Enter,.. } => {
                    if !screen.categories_search.is_empty(){
                        if let Some(t) = screen.get_selected_category() {
                            for transaction in screen.transactions.iter_mut(){
                                process(crate::manager::BudgetCommand::UpdateTransaction(transaction, &Transaction{category_id: t.category_id, ..transaction.clone()}))?;
                            }
                            match &screen.parent {
                                crate::app::new_transaction::ParentScreen::TransactionsList(e)=>{
                                    sender.send(Event::ChangeAppState(crate::app::AppState::TransactionsList(TransactionListScreen::new(&t, &e.current_date)?)))?;
                                },
                                crate::app::new_transaction::ParentScreen::DateList(_) => {
                                    sender.send(Event::ChangeAppState(crate::app::AppState::DateList(DateListScreen::new(t)?)))?;
                                }
                            }
                        }
                    }
                },
                Input { key: Key::Char('a'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Add);
                },
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Search);
                },
                Input { key: Key::Char('d'), ctrl: true, ..} => {
                    // let selected_category = screen.get_selected_category();
                    // if let Some(category) = selected_category {
                    //     // delete associated transactions
                    //     let transactions = list_transaction()?;
                    //     transactions.iter().for_each(|f| {
                    //         if f.category_id == category.category_id{
                    //             process(crate::manager::BudgetCommand::DeleteTransaction(f.clone())).expect("unable to delete transaction");
                    //         }
                    //     });
                    //     // delete category
                    //     process(crate::manager::BudgetCommand::DeleteCategory(category))?;
                    //     screen.update_categories()?;
                    // }
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
                    screen.search_category();
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
                            screen.update_categories()?;
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