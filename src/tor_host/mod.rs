use std::string::String;
use std::sync::mpsc::channel;
use std::thread;

pub enum State{
    ACTIVE,
    PAUSED,
    STOPPED,
}


impl Host {

    pub fn new() -> Host {
        Host { 
    }

    pub fn start(&self) {
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn set_state(&mut self, n_state : State) {
        self.state = n_state;
    }
}
