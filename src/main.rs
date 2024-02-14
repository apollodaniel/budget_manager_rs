use std::{error::Error, io::stdout};

use app::App;
use events::{CrosstermTerminal, Event, EventHandler};
use ratatui::backend::CrosstermBackend;
use tui::Tui;
use ui::draw;
use update::update;


pub mod app;
pub mod events;
pub mod tui;
pub mod ui;
pub mod update;
pub mod manager;

fn main() -> Result<(), Box<(dyn Error)>> {
    
    Tui::enter()?;
    
    let mut app = App::new()?;
    let terminal = CrosstermTerminal::new(CrosstermBackend::new(stdout()))?;
    let mut tui = Tui::new(terminal, EventHandler::new(250));
    
    while !app.should_quit{

        match tui.events.next()? {
            Event::Key(e) => update(&mut app, &e, tui.events.get_sender())?,
            Event::Tick => draw(&mut tui.terminal, &mut app)?,
            Event::ChangeAppState(app_state)=>{
                app.change_app_state(app_state);
            },
            _=>{}
        }
    }

    Tui::reset()?;

    Ok(())
}
