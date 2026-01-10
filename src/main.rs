use std::{collections::HashMap, io::{BufRead, Read, Write}};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};
use std::io::BufReader;

const CRLF: &str = "\r\n";

struct Headers {
    headers: HashMap<String, String>
}

struct HTTPResponse {
    status_code: i32,
    reason: String,
}

impl HTTPResponse {

    fn new(
        status_code: i32,
        reason: &str
    ) -> Self {

        HTTPResponse {
            status_code: status_code,
            reason: reason.to_string(),
        }
    }

    fn as_bytes(self) -> Vec<u8>{
        
        let HTTPResponse {status_code, reason} = self;

        let response = format!(
            "HTTP/1.1 {} {}{}{}",
            status_code, reason, CRLF, CRLF
        );

        response.into_bytes()
    }
}


struct HTTPRequest{
    method: String,
    path: String,
    http_version: String,
    headers: Headers
}

impl HTTPRequest {

    fn parse(stream: &mut TcpStream) -> Self{

        let mut reader = BufReader::new(stream);

        let mut request_line = String::new();

        reader.read_line(&mut request_line).unwrap();

        let parts: Vec<&str>= request_line.trim_end().split_whitespace().collect();

        let method = parts[0].to_string();
        let path = parts[1].to_string();
        let http_version = parts[2].to_string();

        HTTPRequest {
            method: method,
            path: path,
            http_version: http_version,
            headers: Headers { headers: HashMap::new()}
        }

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

                let request = HTTPRequest::parse(&mut stream);

                let (mut status_code, mut reason) = (200, "OK");

                debug!("{}",request.path);

                if request.path != "/index.html" && request.path != "/" {
                    (status_code, reason) = (404, "Not Found");
                    error!("Not Found");
                }
                
                let response = HTTPResponse::new(status_code, reason);
                
                let _ = stream.write(&response.as_bytes());
                stream.flush();
            }
            Err(e) => {
                error!("error: {}", e);
            }
        }
    }
}
