use learn_rust::webserver::Server;

fn main() {
    let mut server = Server::new();
    server.get(
        "/",
        Box::new(|c| {
            dbg!(&c);
            let resp_body =
                "<HTML><head><script>fetch('/api').then(resp=>resp.data())</script></head><body>hello, world!</body></HTML>";
            let resp_body_len = resp_body.len();
            let response =
                format!("HTTP/1.1 200 OK\r\nContent-Length: {resp_body_len}\r\n\r\n{resp_body}");
            c.response(response.as_bytes())
        }),
    );

    server.run("0.0.0.0:8080");
}
