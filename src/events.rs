use std::{io::Stdout, sync::mpsc::{channel, Receiver, Sender}, thread, time::{Duration, Instant}};

use crossterm::event::{self, KeyEvent};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

pub enum Event{
    Tick,
    Key(KeyEvent)
}

pub struct EventHandler{
    receiver: Receiver<Event>
}

impl EventHandler {
    
    pub fn new(tick_rate: u64) -> Self{

        let tick_rate = Duration::from_millis(tick_rate);

        let (sender, receiver): (Sender<Event>, Receiver<Event>) = channel();

        // key & tick events
        {
            thread::spawn(move||{
                let sender = sender.clone();
                let mut last_time = Instant::now();
                loop {
                    let timeout = tick_rate.checked_sub(last_time.elapsed()).unwrap_or(tick_rate);
                
                    sender.send(Event::Tick).expect("unable to send tick event");
                    if event::poll(timeout).expect("unable to pool events"){
                        if let crossterm::event::Event::Key(e) = event::read().expect("unable to read events"){
                            sender.send(Event::Key(e)).expect("unable to send key event");
                        }
                    }

                    if last_time.elapsed() >= tick_rate{
                        last_time = Instant::now();
                    }
                }

            });
        }

        Self{receiver: receiver}
    }

    pub fn next(&self) -> Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }
}
