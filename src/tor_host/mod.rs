use std::string::String;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use std::io;
use std::sync::Arc;

pub enum State{
    ACTIVE,
    PAUSED,
    STOPPED,
}

pub struct Host {
    tx : Sender<State>,
    id : String,
}

impl Host {

    pub fn start<F>(handle_client : F) -> Host 
        where F : Fn(TcpStream) + Send + Sync + 'static {
        let (_tx, _rx) = channel();

        let handle_client = Arc::new(handle_client);

        thread::spawn(move || {
            let listener = match TcpListener::bind("127.0.0.1:80") {
                Ok(_listener) => _listener,
                Err(err) => return
            };
            listener.set_nonblocking(true).expect("Cannot set non-blocking");

            loop {                
                thread::sleep(time::Duration::from_millis(1000));
                let state = match _rx.try_recv() {
                    Ok(_state) => _state,
                    Err(ref e) => continue
                };

                match state {
                    State::PAUSED => continue,
                    State::STOPPED => break,
                    State::ACTIVE => {
                        for stream in listener.incoming() {
                            match stream {
                                Ok(s) => {
                                    println!("client connected");
                                    let handle_client = handle_client.clone();
                                    thread::spawn(move || {
                                        handle_client(s);
                                    });
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    continue;
                                }
                                Err(ref e) => panic!("encountered IO error: {}", e),
                            }
                        } 
                    },
                }
            }

        });

        return Host { tx : _tx, id : String::from("xxxxxxxxxxxxxxxx")}
    }

    pub fn set_state(&mut self, n_state : State) {
    }
}
