use std::{ffi::CStr, io::Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use log::{debug, error, info};

const CRLF: &str = "\r\n";

struct HTTPResponse {
    http_version: String,
    status_code: i32,
    reason: String,
}

impl HTTPResponse {

    fn new(
        http_version: &str,
        status_code: i32,
        reason: &str
    ) -> Self {

        HTTPResponse {
            http_version: http_version.to_string(),
            status_code: status_code,
            reason: reason.to_string(),
        }
    }

    fn as_bytes(self) -> Vec<u8>{
        
        let HTTPResponse {http_version, status_code, reason} = self;

        let response = format!(
            "{} {} {}{}{}",
            http_version,
            status_code, 
            reason, CRLF, CRLF
        );

        response.into_bytes()
    }
}



fn main() {

    env_logger::init();

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    debug!("Logs from your program will appear here!");
    
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                info!("accepted new connection");
                
                let mut response = HTTPResponse::new(
                    "HTTP/1.1", 200, "OK",
                );
                stream.write(&mut response.as_bytes());
            }
            Err(e) => {
                error!("error: {}", e);
            }
        }
    }
}
