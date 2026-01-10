use std::{collections::HashMap, io::{Read, Write}};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};

const CRLF: &str = "\r\n";

struct Headers {
    headers: HashMap<String, String>
}
`
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

        let mut buf = String::new();

        let _ = stream.read_to_string(&mut buf);

        let request = buf.split(CRLF).collect::<Vec<&str>>();

        // request line 
        let request_line = request[0].split_whitespace().collect::<Vec<&str>>();

        let method = request_line[0].to_string();
        let path = request_line[1].to_string();
        let http_version = request_line[2].to_string();

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

                if request.path != "/index.html" {
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
