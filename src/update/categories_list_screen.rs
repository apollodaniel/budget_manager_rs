use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::{categories_list::CategoryListScreen, date_list::DateListScreen, ListScreen, ListingState, MoveListSelection}, events::Event, manager::{command_processing::{list_transaction, process}, Category}};



pub fn update(screen: &mut CategoryListScreen, input: &Input, sender: Sender<Event>)
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
                        if let Some(categories) = screen.get_selected_category(true) {
                            sender.send(Event::ChangeAppState(crate::app::AppState::DateList(DateListScreen::new(categories.first().expect("unable to get selected category").clone())?)))?;
                        }
                    }
                },
                Input { key: Key::Char(' '), .. }=>{
                    if !screen.categories_search.is_empty(){
                        let selected_index = screen.get_selected_category_index();
                        if let Some(index) = selected_index {
                            screen.categories_search[index].1 = !screen.categories_search[index].1;
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
                    let selected_categories = screen.get_selected_category(false);
                    if let Some(categories) = selected_categories {
                        // delete associated transactions
                        let transactions = list_transaction()?;
                        
                        for category in categories{
                            transactions.iter().for_each(|f| {
                                if f.category_id == category.category_id{
                                    process(crate::manager::BudgetCommand::DeleteTransaction(f.clone())).expect("unable to delete transaction");
                                }
                            });
                            // delete category
                            process(crate::manager::BudgetCommand::DeleteCategory(category))?;
                        }
                        screen.update_categories()?;
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