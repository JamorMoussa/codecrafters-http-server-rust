#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

mod server;
mod request;
mod response;

use server::HTTPServer;
use crate::response::HTTPResponse;

fn main() {

    env_logger::init();

    let mut http_server = HTTPServer::new("127.0.0.1", 4221);

    http_server.get(
        "/index.html", 
        |_request| {
            HTTPResponse::new(200, "Ok")
        }
    );

    http_server.get(
        "/", 
        |_request| {
            HTTPResponse::new(200, "Ok")
        }
    );
    
    http_server.start();


}
