use std::net::{TcpListener};
use log::{debug, info, error};
use std::{io::{Write}};

use crate::{request::HTTPRequest, response};
use crate::response::HTTPResponse;

type HTTPFunc = fn(HTTPRequest) -> HTTPResponse;

#[derive(Debug)]
struct Endpoint {
    method: String,
    path: String,
    func: HTTPFunc
}


#[derive(Debug)]
pub struct HTTPServer {
    endpoints: Vec<Endpoint>
}

impl HTTPServer{

    pub fn new() -> Self{
        HTTPServer {
            endpoints: Vec::new()
        }
    }

    fn is_allowed_path(&self, path: &str) -> bool {
        self.endpoints.iter().any(|e| {e.path == path.to_string()})
    }

    pub fn get(&mut self, path: &str, func: HTTPFunc){

        self.endpoints.push(
            Endpoint { method: "GET".to_string(), path: path.to_string(), func: func}
        );
    }

    pub fn bind(
        self, host: &str, port: i32
    ){
        let listener = TcpListener::bind(
            format!("{}:{}", host, port)
        ).unwrap();
        
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("accepted new connection");

                    for endpoint in &self.endpoints {

                        let request = HTTPRequest::parse(&mut stream);

                        let mut response = HTTPResponse::new(404, "Not Found");

                        if self.is_allowed_path(&request.path) {
                            response = (endpoint.func)(request);
                        }
                        
                        let _ = stream.write(&response.as_bytes());
                        stream.flush();
                    }
                }
                Err(e) => {
                    error!("error: {}", e);
                }
            }
        }
    }


}