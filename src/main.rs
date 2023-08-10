mod sal_server;

use std::{collections::HashMap, env};
use sal_server::SalServer;

fn main() {

    for (key, val) in env::vars() {
        println!("{key}: {val}");
    };

    let port = env::var("PORT").unwrap();
    let mut addr = String::from("0.0.0.0:");
    let _ = addr.push_str(port.as_str());

    let serv = SalServer::new(&addr, 8);

    serv.route_http(route);

}

fn route(http_line: (&str, &str), head: HashMap<&str, &str>, body: &str) -> (Vec<u8>, bool) {
    println!("HEADER: {:#?}", head);

    let mut buf = Vec::from("HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n");
    let buff = Vec::from(format!("Http Line: {:?}\r\nHead: {:#?}\r\nBody: {}\r\n", http_line, head, body));
    buf.extend(buff);
    return (buf, false)
}
