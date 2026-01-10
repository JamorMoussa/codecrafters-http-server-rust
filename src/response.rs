use std::{collections::HashMap, io::{Write}};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};

const CRLF: &str = "\r\n";

pub struct HTTPResponse {
    status_code: i32,
    reason: String,
}

impl HTTPResponse {

    pub fn new(
        status_code: i32,
        reason: &str
    ) -> Self {

        HTTPResponse {
            status_code: status_code,
            reason: reason.to_string(),
        }
    }

    pub fn as_bytes(self) -> Vec<u8>{
        
        let HTTPResponse {status_code, reason} = self;

        let response = format!(
            "HTTP/1.1 {} {}{}{}",
            status_code, reason, CRLF, CRLF
        );

        response.into_bytes()
    }
}