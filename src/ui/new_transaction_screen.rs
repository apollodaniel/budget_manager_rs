use std::error::Error;

use ratatui::{layout::{self, Constraint, Layout}, style::Stylize, widgets::{Block, Borders, Paragraph}};

use crate::{app::new_transaction::NewTransactionScreen, events::CrosstermTerminal};


pub fn draw(screen: &mut NewTransactionScreen, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    let block = Block::default().title("New transaction").title_alignment(ratatui::layout::Alignment::Center).borders(Borders::NONE);

    terminal.draw(|f|{        
        if let Some(error) = screen.error.clone() {
            
            let layout = Layout::new(
                layout::Direction::Vertical,
                [
                    Constraint::Min(1),
                    Constraint::Length(2),
                    Constraint::Min(1),
                ]
            ).split(f.size());

            f.render_widget(Paragraph::new(error).red().alignment(layout::Alignment::Center), layout[1]);

        }else{
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

            if let None = screen.date {
                f.render_widget(screen.date_text_area.widget(), layout[3]);
            }
        }
    })?;

    
    Ok(())
}