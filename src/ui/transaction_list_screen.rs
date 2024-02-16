use std::error::Error;

use ratatui::{layout::{Constraint, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection}};

use crate::{app::{self, transactions_list::TransactionListScreen}, events::CrosstermTerminal};



pub fn draw(screen: &mut TransactionListScreen, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    let block = Block::default().title("Transações").borders(Borders::ALL);
    let list = List::new(
        screen.transactions_search.iter().map(
            |f|format!(
                "{} - {} - {} {}",
                f.0.description.clone(),
                f.0.get_amount_formatted(),
                f.0.get_date_formatted().unwrap_or("".to_string()),
                if f.1 {"[X]"}else{"[ ]"}
        )
    ).collect::<Vec<String>>())
        .block(block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    match &screen.listing_state {
        app::ListingState::List => {
            terminal.draw(|f|{
                f.render_stateful_widget(list, f.size(), &mut screen.transactions_list_state);
            })?;
        },
        app::ListingState::Search=>{
            terminal.draw(|f|{
                let layout = Layout::new(
                    ratatui::layout::Direction::Vertical,
                    [
                        Constraint::Min(1),
                        Constraint::Length(3)
                    ]
                ).split(f.size());
                f.render_stateful_widget(list, layout[0], &mut screen.transactions_list_state);
                f.render_widget(screen.search_text_area.widget(), layout[1]);
            })?;
        },
        _=>{
            
        }
        }
    Ok(())
}