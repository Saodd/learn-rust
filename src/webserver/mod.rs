use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

pub struct Server {}

impl Server {
    pub fn run(addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for incoming in listener.incoming() {
            let stream = incoming.unwrap();

            Server::handle_stream(stream);
        }
    }

    fn handle_stream(stream: TcpStream) {
        let buf = BufReader::new(stream);

        let req: Vec<String> = buf
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {:#?}", req);
    }
}
