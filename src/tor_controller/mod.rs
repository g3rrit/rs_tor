use std::process::{Command, Child};
use std::string::String;
use std::net::{TcpStream, Shutdown};
use std::io::*;
use std::u8;
use std::{thread, time};
use std::str;

pub struct Controller {
    prog : Child,
    con : TcpStream,
}

impl Controller {
    pub fn new(path : String) -> Result<Controller> {
        let mut tor_exe : String = path.clone();
        tor_exe.push_str(&String::from("mac/tor"));
        let _prog = Command::new("tor")
            .args(&["-controlport", "9050"])
            .spawn()?;

        //wait for tor to start
        thread::sleep(time::Duration::from_millis(5000));

        let mut _con = TcpStream::connect(String::from("127.0.0.1:9050"))?;

        _con.write(b"AUTHENTICATE \r\n")?;
        _con.flush()?;

        let mut buffer : [u8; 256] = [0; 256];
        _con.read(&mut buffer)?;

        match &buffer[0 .. 2] {
            err => return Err(Error::new(ErrorKind::Other, String::from("invalid ret value: ".to_owned() + &str::from_utf8(&buffer).unwrap()))),
            [50, 48, 48] => (),
        }

        Ok(Controller { prog : _prog, con : _con })
    }

    pub fn start_service(&mut self, port : u16) -> Result<String> {
        //let ret = self.con.read().unwrap();
        let command = format!("ADD_ONION NEW:BEST Flags=DiscardPK Port={}", port);
        self.con.write(command.as_bytes())?;

        let mut buffer : [u8; 256] = [0; 256];
        self.con.read(&mut buffer)?;

        Ok(String::from("xxxxxxxxxxxxxxxx"))
    }

    pub fn stop(&mut self) -> Result<()> {
        match self.prog.kill() {
            Ok(ok) => (),
            Err(err) => (),
        }

        self.con.shutdown(Shutdown::Both)
    }
}