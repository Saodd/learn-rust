use std::net::TcpListener;

pub struct Server {}

impl Server {
    pub fn run(addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for incoming in listener.incoming() {
            let stream = incoming.unwrap();

            dbg!(stream);
        }
    }
}
