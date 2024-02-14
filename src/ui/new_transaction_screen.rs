use std::error::Error;

use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Borders}};

use crate::{app::new_transaction::NewTransactionScreen, events::CrosstermTerminal};


pub fn draw(screen: &mut NewTransactionScreen, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    let block = Block::default().title("New transaction").title_alignment(ratatui::layout::Alignment::Center).borders(Borders::NONE);

    terminal.draw(|f|{
        
        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
        ).split(f.size());
            
            
        f.render_widget(block, f.size());

        f.render_widget(screen.description_text_area.widget(), layout[1]);
        f.render_widget(screen.amount_text_area.widget(), layout[2]);
        f.render_widget(screen.date_text_area.widget(), layout[3]);

        
    })?;

    
    Ok(())
}