pub mod categories_list_screen;

use std::error::Error;


use crate::{app::App, events::CrosstermTerminal};


// pub fn draw_date_list(app: &mut App, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
//     let block = Block::default().title("Date").borders(Borders::ALL);
//     let list = List::new(app.transactions_date_search.clone())
//         .block(block)
//         .style(Style::default().fg(Color::White))
//         .highlight_style(Style::default().add_modifier(Modifier::BOLD))
//         .highlight_symbol(">> ")
//         .repeat_highlight_symbol(true)
//         .direction(ListDirection::TopToBottom);

//     match &app.listing_state {
//         app::ListingState::List => {
//             terminal.draw(|f|{
//                 f.render_stateful_widget(list, f.size(), &mut app.date_list_state);
//             })?;
//         },
//         app::ListingState::Search=>{
//             terminal.draw(|f|{
//                 let layout = Layout::new(
//                     ratatui::layout::Direction::Vertical,
//                     [
//                         Constraint::Min(1),
//                         Constraint::Length(3)
//                     ]
//                 ).split(f.size());
//                 f.render_stateful_widget(list, layout[0], &mut app.date_list_state);
//                 f.render_widget(app.search_text_area.widget(), layout[1]);
//             })?;
//         },
//         _=>{
            
//         }
//         }
//     Ok(())
// }


pub fn draw(terminal: &mut CrosstermTerminal, app: &mut App)->Result<(), Box<(dyn Error)>>{
    match &mut app.app_state {
        crate::app::AppState::CategoriesList(e) => categories_list_screen::draw(e,terminal)?,
        //crate::app::AppState::DateList(_) => draw_date_list(app,terminal)?,
        _=>{}
    }

    Ok(())

}