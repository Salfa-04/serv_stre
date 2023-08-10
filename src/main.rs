mod sal_server;

use std::{collections::HashMap, env};
use sal_server::SalServer;

fn main() {

    let port = env::var("PORT").unwrap();
    let mut addr = String::from("0.0.0.0:");
    addr.push_str(port.as_str());

    SalServer::new(&addr, 8).route_http(route);

}

fn route(http_line: (&str, &str), head: HashMap<&str, &str>, body: &str) -> (Vec<u8>, bool) {

    if let Some(x) = head.get("Connection") {
        println!("Keep-Alive: {}", x);
    };

    let val = if body.is_empty() {
        format!("Http Line: {:?}\r\nHead: {:#?}\r\n", http_line, head)
    } else {
        format!("Http Line: {:?}\r\nHead: {:#?}\r\nBody: {}\r\n", http_line, head, body)
    };

    let mut buf = Vec::from(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\r\n"
    );
    buf.extend(Vec::from(val));
    return (buf, false)
}
