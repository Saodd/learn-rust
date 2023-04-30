use learn_rust::webserver::{Context, Server};
use std::sync::Arc;

fn main() {
    let mut server = Server::new();
    let handler = (|c: &mut Context| {
        dbg!(&c);
        let resp_body =
            "<HTML><head><script>fetch('/api').then(resp=>resp.data())</script></head><body>hello, world!</body></HTML>";
        let resp_body_len = resp_body.len();
        let response =
            format!("HTTP/1.1 200 OK\r\nContent-Length: {resp_body_len}\r\n\r\n{resp_body}");
        c.response(response.as_bytes())
    });
    server.get("/", handler);
    // server.get("/test", handler);

    server.run("0.0.0.0:8080");
}
