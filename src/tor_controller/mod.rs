use std::process::{Command, Child};
use std::string::String;
use std::net::{TcpStream, Shutdown};
use std::io::{Read, Write, Error};
use std::u8;
use std::{thread, time};
use std::str;

pub struct Controller {
    prog : Child,
    con : TcpStream,
}

impl Controller {
    pub fn new(path : String) -> Result<Controller, String> {
        let mut tor_exe : String = path.clone();
        tor_exe.push_str(&String::from("mac/tor"));
        let _prog =  match Command::new("tor")
            .args(&["-controlport", "9050"])
            .spawn() {
                Ok(ok) => ok,
                Err(err) => return Err(String::from("starting tor ".to_owned() + &err.to_string())),
        };

        //wait for tor to start
        thread::sleep(time::Duration::from_millis(5000));

        let mut _con = match TcpStream::connect(String::from("127.0.0.1:9050")) {
            Ok(ok) => ok,
            Err(err) => return Err(err.to_string()),
        };

        match _con.write(b"AUTHENTICATE \r\n") {
            Ok(ok) => (),
            Err(err) => return Err(String::from("sending authentication ".to_owned() + &err.to_string())),
        }
        match _con.flush() {
            Ok(ok) => (),
            Err(err) => return Err(String::from("flushing ".to_owned() + &err.to_string())),
        }

        let mut buffer : [u8; 256] = [0; 256];
        match _con.read(&mut buffer) {
            Ok(ok) => (),
            Err(err) => return Err(String::from("receiving auth response ".to_owned() + &err.to_string())),
        }

        match &buffer[0 .. 2] {
            err => return Err(String::from("invalid ret value: ".to_owned() + &str::from_utf8(&buffer).unwrap())),
            [50, 48, 48] => (),
        }

        Ok(Controller { prog : _prog, con : _con })
    }

    pub fn start_service(&mut self, port : u16) -> Result<String, String> {
        //let ret = self.con.read().unwrap();
        let command = format!("ADD_ONION NEW:BEST Flags=DiscardPK Port={}", port);
        match self.con.write(command.as_bytes()) {
            Ok(ok) => (),
            Err(err) => return Err(String::from("sent start service request ".to_owned() + &err.to_string())),
        }

        let mut buffer : [u8; 256] = [0; 256];
        match self.con.read(&mut buffer) {
            Ok(ok) => (),
            Err(err) => return Err(String::from("received start service response ".to_owned() + &err.to_string())),
        }

        Ok(String::from("xxxxxxxxxxxxxxxx"))
    }

    pub fn stop(&mut self) {
        match self.prog.kill() {
            Ok(ok) => (),
            Err(err) => (),
        }

        self.con.shutdown(Shutdown::Both).expect("error shutting down control connection");
    }
}