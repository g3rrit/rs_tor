pub mod tor_socks;
pub mod tor_host;

#[cfg(test)]
mod tests {
    extern crate socks;

    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::string::String;

    #[test]
    fn socks_works() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9050);
        let target = socks::TargetAddr::Domain(String::from("www.web.de"), 80);

        let stream = socks::Socks5Stream::connect(addr, target);

        match stream {
            Ok(_socket) => {
                println!("connected via socks");
            }
            Err(_er) => {
                assert!(false);
            }
        }
    }

    use tor_socks;

    #[test]
    fn msocks_works()  {
        let stream = tor_socks::get(String::from("www.web.de"), 80);

        let socket = match stream {
            Ok(_socket) => _socket,
            Err(_err) => panic!("failed")
        };
    }

    use tor_host::Host;
    use tor_host::State;
    use std::{thread, time};
    #[test]
    fn host_works() {
        let mut host = Host::start(|s| {
            println!("hello");
        });

        host.set_state(State::PAUSED);

    thread::sleep(time::Duration::from_millis(10000));
    }
}
