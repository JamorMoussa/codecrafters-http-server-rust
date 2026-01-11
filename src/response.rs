use std::{collections::HashMap, io::{Write}};
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};

const CRLF: &str = "\r\n";
const HTTP_VERSION: &str = "HTTP/1.1";

pub struct HTTPResponse {
    content: String, 
    content_type: String,
    content_len: usize,
    status_code: i32,
    reason: String,
}

impl HTTPResponse {

    pub fn new(
        content: &str,
        content_type: &str,
        status_code: i32,
        reason: &str
    ) -> Self {

        HTTPResponse {
            content: content.to_string(),
            content_type: content_type.to_string(),
            content_len: content.len(),
            status_code: status_code,
            reason: reason.to_string(),
        }
    }

    pub fn as_bytes(self) -> Vec<u8>{
        
        let HTTPResponse {content, content_type,content_len, status_code, reason} = self;

        let response = format!(
            "{} {} {}{}Content-Type: {}{}Content-Length: {}{}{}{}",
            HTTP_VERSION, status_code, reason, CRLF, content_type, CRLF, content_len, CRLF, CRLF, content
        );

        response.into_bytes()
    }
}