use std::{io::Stdout, sync::mpsc::{channel, Receiver, Sender}, thread, time::{Duration, Instant}};

use crossterm::event;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui_textarea::Input;

use crate::app::AppState;

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

pub enum Event{
    Tick,
    Key(Input),
    ChangeAppState(AppState),
    UpdateTransactions,
    UpdateCategories,
    UpdateAll
}

pub struct EventHandler{
    receiver: Receiver<Event>,
    sender: Sender<Event>
}

impl EventHandler {
    
    pub fn new(tick_rate: u64) -> EventHandler{

        let tick_rate = Duration::from_millis(tick_rate);

        let (sender, receiver): (Sender<Event>, Receiver<Event>) = channel();

        // key & tick events
        {
            let sender = sender.clone();
            thread::spawn(move||{
                let mut last_time = Instant::now();
                loop {
                    let timeout = tick_rate.checked_sub(last_time.elapsed()).unwrap_or(tick_rate);
                
                    sender.send(Event::Tick).expect("unable to send tick event");
                    if event::poll(timeout).expect("unable to pool events"){
                        if let crossterm::event::Event::Key(e) = event::read().expect("unable to read events"){
                            sender.send(Event::Key(e.into())).expect("unable to send key event");
                        }
                    }

                    if last_time.elapsed() >= tick_rate{
                        last_time = Instant::now();
                    }
                }

            });
        }

        Self{receiver: receiver, sender: sender.clone()}
    }

    pub fn next(&self) -> Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }

    pub fn get_sender(&self) -> Sender<Event>{
        self.sender.clone()
    }
}
