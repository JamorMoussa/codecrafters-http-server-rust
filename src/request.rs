use std::{collections::HashMap, io::{BufRead}};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;

pub struct Headers {
    pub headers: HashMap<String, String>
}

pub struct HTTPRequest{
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: Headers
}

impl HTTPRequest {

    pub fn parse(stream: &mut TcpStream) -> Self{

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