use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

const TICK_INTERVAL: Duration = Duration::from_millis(500);

pub enum Event {
    Tick,
    Input(Key),
}

pub fn receiver() -> mpsc::Receiver<Event> {
    let (timer_tx, event) = mpsc::channel();
    let input_tx = timer_tx.clone();
    thread::spawn(move || loop {
        thread::sleep(TICK_INTERVAL);
        timer_tx.send(Event::Tick).expect("sending tick failed");
    });
    thread::spawn(move || {
        let stdin = std::io::stdin();
        for c in stdin.keys() {
            let key = c.unwrap_or(Key::Null);
            input_tx
                .send(Event::Input(key))
                .expect("sending input failed");
        }
    });
    event
}
