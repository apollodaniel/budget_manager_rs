use std::error::Error;

use ratatui::{layout::{Constraint, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List, ListDirection}};

use crate::{app::{self, categories_list::CategoryListScreen}, events::CrosstermTerminal};

pub fn draw(screen: &mut CategoryListScreen, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
        let block = Block::default().title("Categorias").borders(Borders::ALL);
        let list = List::new(screen.categories_search.iter().map(|f|format!("{} {}", f.0.name, if f.1 {"[x]"}else{"[ ]"})).collect::<Vec<String>>())
            .block(block)
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        match &screen.listing_state {
            app::ListingState::List => {
                terminal.draw(|f|{
                    f.render_stateful_widget(list, f.size(), &mut screen.categories_list_state);
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
                    f.render_stateful_widget(list, layout[0], &mut screen.categories_list_state);
                    f.render_widget(screen.search_text_area.widget(), layout[1]);
                })?;
            },
            app::ListingState::Add=>{
                terminal.draw(|f|{
                    let layout = Layout::new(
                        ratatui::layout::Direction::Vertical,
                        [
                            Constraint::Min(1),
                            Constraint::Length(3)
                        ]
                    ).split(f.size());
                    f.render_stateful_widget(list, layout[0], &mut screen.categories_list_state);
                    f.render_widget(screen.add_text_area.widget(), layout[1]);
                })?;
            }
            }
        Ok(())
    }  