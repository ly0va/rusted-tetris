use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use console::*;

pub enum Event {
    Tick, Input(char)
}

pub fn receiver() -> mpsc::Receiver<Event> {
    let (timer_tx, event) = mpsc::channel();
    let input_tx = timer_tx.clone();
    thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(500));
        timer_tx.send(Event::Tick);
    });
    thread::spawn(move || {
        let term = Term::buffered_stdout();
        loop {
            if let Ok(key) = term.read_char() {
                input_tx.send(Event::Input(key));
            }
        }
    });
    event
}
