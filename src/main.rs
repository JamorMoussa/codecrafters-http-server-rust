#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

mod server;
mod request;
mod response;

use log::debug;
use server::HTTPServer;
use crate::response::HTTPResponse;

fn main() {

    env_logger::init();

    let mut http_server = HTTPServer::new("127.0.0.1", 4221);

    http_server.get(
        "/echo/abc", 
        |_request| {

            let content = _request.params.clone().unwrap();
            HTTPResponse::new(&content, "text/plain",200, "Ok")
        }
    );
    
    http_server.start();
}
