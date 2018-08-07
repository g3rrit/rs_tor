use std::string::String;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, time};
use std::net::{TcpListener, TcpStream};

pub enum State{
    ACTIVE,
    PAUSED,
    STOPPED,
}

pub struct Host {
    tx : Sender<State>,
    rx : Receiver<State>,
    id : String,
    state : State
}

impl Host {

    pub fn new() -> Host {
        let (_tx, _rx) = channel();
        Host { tx : _tx, rx : _rx, id : String::from("xxxxxxxxxxxxxxx"), state : State::ACTIVE }
    }

    pub fn start(&self) {
        thread::spawn(move || {
            let listener = match TcpListener::bind("127.0.0.1:80") {
                Ok(_listener) => _listener,
                Err(err) => return
            };

            loop {                
                thread::sleep(time::Duration::from_millis(1000));
                let n_state = match self.rx.try_recv() {
                    Ok(_n_state) => _n_state,
                    Err(err) => continue
                };

                match n_state {
                    State::ACTIVE => {},
                    State::PAUSED => continue,
                    State::STOPPED => break
                }

                println!("loop");
            }

        });
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn set_state(&mut self, n_state : State) {
        self.state = n_state;
    }
}
