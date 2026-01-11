use std::net::{TcpListener};
use log::{debug, info, error};
use std::{io::{Write}};

use crate::{request::{self, HTTPRequest}, response};
use crate::response::HTTPResponse;

type HTTPFunc = fn(&HTTPRequest) -> HTTPResponse;

#[derive(Debug)]
struct Endpoint {
    method: String,
    path: String,
    func: HTTPFunc
}


#[derive(Debug)]
pub struct HTTPServer {
    host: String,
    port: i32,
    endpoints: Vec<Endpoint>
}

impl HTTPServer{

    pub fn new(
        host: &str, port: i32
    ) -> Self{
        HTTPServer {
            host: host.to_string(), port: port, endpoints: Vec::new()
        }
    }

    fn is_allowed_request(&self, e: &Endpoint, request: &HTTPRequest) -> bool {

        if e.path == request.path && request.method == e.method {
                return true;
        }
        return false
    }

    pub fn get(&mut self, path: &str, func: HTTPFunc){

        let (path, _) = path.rsplit_once("/").unwrap();

        self.endpoints.push(
            Endpoint { method: "GET".to_string(), path: path.to_string(), func: func}
        );
    }

    pub fn start(self){
        
        let listener = TcpListener::bind(
            format!("{}:{}", self.host, self.port)
        ).unwrap();
        
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("accepted new connection");
                    
                    let request = HTTPRequest::parse(&mut stream);
                    
                    let mut response = HTTPResponse::new("", "text/plain", 404, "Not Found");

                    for endpoint in &self.endpoints {

                        if self.is_allowed_request(endpoint, &request){
                            response = (endpoint.func)(&request);
                            break;
                        }
                    }
                    let _ = stream.write(&response.as_bytes());
                    stream.flush();
                }
                Err(e) => {
                    error!("error: {}", e);
                }
            }
        }
    }


}