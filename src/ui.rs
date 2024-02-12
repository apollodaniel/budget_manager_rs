use std::{borrow::BorrowMut, error::Error, rc::Rc};

use ratatui::{layout::{Constraint, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection, ListState, Paragraph}};

use crate::{app::{self, App}, events::CrosstermTerminal};



pub fn draw_categories_list(app: &mut App, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    let block = Block::default().title("Categories").borders(Borders::ALL);
    let list = List::new(app.categories_search.clone())
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    match &app.listing_state {
        app::ListingState::Listing => {
            terminal.draw(|f|{
                f.render_stateful_widget(list, f.size(), &mut app.categories_list_state);
            })?;
        },
        app::ListingState::Searching=>{
            terminal.draw(|f|{
                let layout = Layout::new(
                    ratatui::layout::Direction::Vertical,
                    [
                        Constraint::Min(1),
                        Constraint::Length(3)
                    ]
                ).split(f.size());
                f.render_stateful_widget(list, layout[0], &mut app.categories_list_state);
                f.render_widget(app.text_area.widget(), layout[1]);
            })?;
        }
        }
    Ok(())
}  

pub fn draw(terminal: &mut CrosstermTerminal, app: &mut App)->Result<(), Box<(dyn Error)>>{
    match &app.app_state {
        crate::app::AppState::CategoriesList => draw_categories_list(app,terminal)?,
        _=>{}
    }

    Ok(())

}