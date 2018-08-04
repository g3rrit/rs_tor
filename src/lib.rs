mod socks;

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

    #[test]
    fn msocks_works()  {
        let stream = socks::get(String::from("www.web.de"), 80);

        match stream {
            Ok(_socket) => println!("connected via socks"),
            Err(_err) => panic!("failed")
        }
    }
}
