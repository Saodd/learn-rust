use std::io::{BufRead, BufReader, Write};
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

    fn handle_stream(mut stream: TcpStream) {
        let buf = BufReader::new(&stream);

        let req: Vec<String> = buf
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        println!("Request: {:#?}", req);

        let resp_body = "hello, world!";
        let resp_body_len = resp_body.len();
        let response =
            format!("HTTP/1.1 200 OK\r\nContent-Length: {resp_body_len}\r\n\r\n{resp_body}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
