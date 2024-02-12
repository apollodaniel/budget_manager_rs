use std::{borrow::BorrowMut, error::Error, rc::Rc};

use ratatui::{style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph}};

use crate::{app::{self, App}, events::CrosstermTerminal};



pub fn draw_categories_list(app: &mut App, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    if let app::AppState::CategoriesList(listing_state) = &app.app_state{
        
        match listing_state {
            app::ListingState::Listing => {
                terminal.draw(|f|{

                    let block = Block::default().title("Categories").borders(Borders::ALL);
                    let list = List::new(app.categories.iter().map(|f|f.name.clone()).collect::<Vec<String>>())
                        .block(block)
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                        .highlight_symbol(">> ")
                        .repeat_highlight_symbol(true)
                        .direction(ListDirection::TopToBottom);
                
                    f.render_stateful_widget(list, f.size(), &mut app.categories_list_state);
                })?;
            },
            app::ListingState::Searching=>{



            }
        }
    }
    Ok(())
}  

pub fn draw(terminal: &mut CrosstermTerminal, app: &mut App)->Result<(), Box<(dyn Error)>>{
    match &app.app_state {
        crate::app::AppState::CategoriesList(_) => draw_categories_list(app,terminal)?,
        _=>{}
    }

    Ok(())

}