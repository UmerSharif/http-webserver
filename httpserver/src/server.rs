use super::router::Router;
use http::httprequest::HttpRequest;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("connection established");
            let mut buffer = [0; 99];
            stream.read(&mut buffer).unwrap();
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}
