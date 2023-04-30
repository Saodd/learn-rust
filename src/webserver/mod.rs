use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

type Handler = Box<dyn Fn(&mut Context) -> ()>;

#[derive(Debug)]
pub struct Context<'a> {
    stream: &'a mut TcpStream,
}

impl Context<'_> {
    pub fn response(&mut self, data: &[u8]) {
        self.stream.write_all(data).unwrap();
    }
}

pub struct Server {
    get_handlers: HashMap<String, Handler>,
}

impl Server {
    pub fn new() -> Self {
        return Self {
            get_handlers: HashMap::new(),
        };
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        self.get_handlers.insert(String::from(path), handler);
    }

    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();

        for incoming in listener.incoming() {
            let stream = incoming.unwrap();

            self.handle_stream(stream);
        }
    }

    fn handle_stream(&self, mut stream: TcpStream) {
        println!("新的stream耶！");
        loop {
            let buf = BufReader::new(&stream);
            let req_lines: Vec<String> = (buf)
                .lines()
                .map(|line| line.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            println!("Request: {:#?}", req_lines);

            let (method, path, http_version) = match req_lines.get(0) {
                Some(line) => {
                    let mut words: VecDeque<String> = line.split(' ').map(String::from).collect();
                    (
                        words.remove(0).unwrap(),
                        words.remove(0).unwrap(),
                        words.remove(0).unwrap(),
                    )
                }
                None => return,
            };

            match self.get_handlers.get(&path) {
                None => self.response_404(&mut stream),
                Some(handler) => handler(&mut Context {
                    stream: &mut stream,
                }),
            }
        }
    }

    fn response_404(&self, stream: &mut TcpStream) {
        let resp_body = "404 Not found~~";
        let resp_body_len = resp_body.len();
        let response =
            format!("HTTP/1.1 404 Not found\r\nContent-Length: {resp_body_len}\r\n\r\n{resp_body}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
