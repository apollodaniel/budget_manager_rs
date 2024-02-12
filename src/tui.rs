use std::{error::Error, panic};

use crossterm::{execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};

use crate::events::{CrosstermTerminal, EventHandler};


pub struct Tui{
    pub terminal: CrosstermTerminal,
    pub events: EventHandler
}

impl Tui {
    
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self{
        Self { terminal: terminal, events: events }
    }

    pub fn enter()->Result<(), Box<(dyn Error)>>{
        terminal::enable_raw_mode()?;
        execute!(std::io::stdout(), EnterAlternateScreen)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move|panic|{
            Self::reset().expect("unable to reset terminal state");
            panic_hook(panic)
        }));

        Ok(())
    }

    pub fn reset() -> Result<(), Box<(dyn Error)>>{
        terminal::disable_raw_mode()?;
        execute!(std::io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }


}
