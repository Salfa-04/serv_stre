mod sal_server;

use sal_server::SalServer;
use std::collections::HashMap;
use std::env::var;

fn main() {

    let port = var("PORT").unwrap_or(String::from("8888")).parse();
    SalServer::new(("0.0.0.0", port.unwrap_or(8888)), 8).route_http(route);

}

fn route(http_line: (&str, &str), head: HashMap<&str, &str>, body: &str) -> (Vec<u8>, bool) {

    let val = if body.is_empty() {
        format!("Http Line: {:?}\r\nHead: {:#?}\r\n", http_line, head)
    } else {
        format!(
            "Http Line: {:?}\r\nHead: {:#?}\r\nBody: {}\r\n",
            http_line, head, body
        )
    };

    let mut buf = Vec::from(format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\
        Content-Length: {}\r\n\r\n", val.len()
    ));

    buf.extend(Vec::from(val));

    if let Some(live) = head.get("Connection") {
        if live == &"close" {
            return (buf, false);
        };
    };

    (buf, true)

}
