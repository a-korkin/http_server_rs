use crate::utils::http;
use std::{collections::HashMap, fmt::Display};

pub struct Response<'a> {
    pub status: http::HttpStatus,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: &'a str,
}

impl<'a> Response<'a> {
    pub fn new(status: http::HttpStatus, body: &'a str) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }
}

impl<'a> Display for Response<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        write!(
            f,
            "HTTP/1.1 {}\r\n{}\r\n\r\n{}",
            self.status, headers, self.body
        )
    }
}
