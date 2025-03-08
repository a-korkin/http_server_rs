use crate::http::HttpMethod;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Request<'a> {
    pub request_line: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: &'a str,
    pub method: HttpMethod,
    pub target: &'a str,
    pub protocol: &'a str,
}

impl<'a> From<&'a str> for Request<'a> {
    fn from(value: &'a str) -> Self {
        let lines: Vec<&'a str> = value.split("\r\n").collect();
        let mut headers: HashMap<&'a str, &'a str> = HashMap::new();
        let mut count_headers = 0;
        for header in &lines[1..] {
            count_headers += 1;
            if *header == "" {
                break;
            }
            if let Some(op) = header.split_once(":") {
                headers.insert(op.0.trim(), op.1.trim());
            };
        }
        let request_line = lines[0];
        let rq_tokens: Vec<&str> = request_line.split_whitespace().collect();
        Self {
            request_line,
            headers,
            body: lines[count_headers + 1..][0],
            method: HttpMethod::from(rq_tokens[0]),
            target: rq_tokens[1],
            protocol: rq_tokens[2],
        }
    }
}
