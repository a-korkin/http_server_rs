use crate::http;
use chrono::Local;
use std::{collections::HashMap, fmt::Display};

pub struct Response<'a> {
    pub status: http::HttpStatus,
    pub headers: HashMap<&'a str, String>,
    pub body: &'a str,
}

impl<'a> Response<'a> {
    pub fn new(status: http::HttpStatus, body: &'a str) -> Self {
        let now = Local::now().to_utc();
        let date = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let headers = HashMap::from([
            ("Date", date),
            ("Content-Length", body.len().to_string()),
            ("Content-Type", String::from("text/plain")),
        ]);
        Self {
            status,
            headers,
            body,
        }
    }
}

impl<'a> Display for Response<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}\r\n", k, v))
            .collect();
        write!(
            f,
            "HTTP/1.1 {}\r\n{}\r\n{}",
            self.status, headers, self.body
        )
    }
}
