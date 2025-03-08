use std::fmt::Display;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    UNKNOWN,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    Status200,
    Status404,
    Status500,
    Undefined,
}

impl From<&str> for HttpStatus {
    fn from(value: &str) -> Self {
        match value {
            "200 OK" => Self::Status200,
            "404 Not Found" => Self::Status404,
            "500 Internal Server Error" => Self::Status500,
            _ => Self::Undefined,
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status200 => write!(f, "200 OK"),
            Self::Status404 => write!(f, "404 Not Found"),
            Self::Status500 => write!(f, "500 Internal Server Error"),
            _ => write!(f, "Status Unknown"),
        }
    }
}
