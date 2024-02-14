use std::{error::Error, sync::mpsc::Sender};

use tui_textarea::{Input, Key};

use crate::{app::{date_list::DateListScreen, ListScreen, ListingState, MoveListSelection, NewTransaction::{NewTransactionParent, NewTransactionScreen}}, events::Event};



pub fn update(screen: &mut DateListScreen, input: &Input, sender: Sender<Event>)->Result<(), Box<(dyn Error)>>{
    match &screen.listing_state {
        crate::app::ListingState::List => {

            match &input {
                Input { key: Key::Char('l'), ctrl: true, .. } => {
                    screen.clear_input();
                    screen.search_dates();
                },
                Input { key: Key::Esc, .. } => {
                    sender.send(Event::ChangeAppState(crate::app::AppState::CategoriesList(crate::app::categories_list::CategoryListScreen::new_with_selected(screen.category.clone())?)))?;
                },
                Input { key: Key::Up, .. }=>screen.move_list_selection(crate::app::MoveSelection::Up),
                Input { key: Key::Down, .. }=>screen.move_list_selection(crate::app::MoveSelection::Down),
                Input { key: Key::Char('a'), ctrl: true, ..} => {
                    //screen.change_listing_state(ListingState::Add);
                    sender.send(
                        Event::ChangeAppState(
                            crate::app::AppState::NewTransaction(
                                NewTransactionScreen::new(
                                NewTransactionParent::DateList(screen.clone()),
                                    None
                                )
                            )
                        )
                    )?;
                },
                Input { key: Key::Char('f'), ctrl: true, ..} => {
                    screen.change_listing_state(ListingState::Search);
                },
                Input { key: Key::Char('d'), ctrl: true, ..} => {
                    // loop for all transactions of this date and delete

                },
                Input { key: Key::Enter, .. }=>{
                    // screen.change_listing_state(ListingState::List)
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
                    screen.search_dates();
                }
            }
        },

        crate::app::ListingState::Add => {
            // match input {
            //     Input { key: Key::Esc, .. }=>{
            //         screen.clear_input();
            //         screen.change_listing_state(ListingState::List)
            //     },
            //     Input { key: Key::Enter, .. }=>{
            //         if let Some(input) = app.add_text_area.lines().first(){
            //             if !input.is_empty(){
            //                 process(
            //                     crate::manager::BudgetCommand::CreateCategory(
            //                         Category::new(
            //                             app.add_text_area.lines().first().unwrap_or(&String::new()).to_string()
            //                         )?
            //                     )
            //                 )?;
            //                 app.update_categories()?;
            //                 app.clear_input();
            //             }
            //         }
            //         app.change_listing_state(ListingState::List);
            //     },
            //     input => {
            //         app.add_text_area.input(input.clone());
            //     }
            // }
        }
    }
    
    Ok(())
}