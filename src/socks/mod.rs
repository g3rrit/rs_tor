extern crate socks;

use std::string::String;
use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};

pub fn get(target: &String, port: u16) -> Option<TcpStream> {
    let local = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9050);
    let target = socks::TargetAddr::Domain(target, port); 

    let stream = socks::Socks5Stream::connect(local, target);
    match stream {
        Ok(socket) => Ok(socket.into_inner()),
        Err(err) => {
            println!("err: {}",err);
        }
    }
}
