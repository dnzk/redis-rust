use std::fmt;

use crate::Command;

pub struct Response<'a> {
    r: &'a str,
}

impl<'a> Response<'a> {
    pub fn new(source: &'a str) -> Self {
        Response { r: source }
    }

    fn format(&self) -> String {
        if self.r.is_empty() {
            return format!("$4\r\nPONG\r\n");
        }
        format!("${}\r\n{}\r\n", self.r.len(), self.r)
    }

    pub fn from(command: &'a Command) -> Self {
        match command {
            Command::Echo(s) => Response { r: s },
            _ => Response { r: "" },
        }
    }

    pub fn buf(&self) -> Vec<u8> {
        self.format().into_bytes()
    }
}

impl<'a> fmt::Display for Response<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod resp_tests {
    use super::*;

    #[test]
    fn formats_correctly() {
        let resp = Response::new("hey");
        assert_eq!(resp.format(), String::from("$3\r\nhey\r\n"));
    }

    #[test]
    fn is_printable() {
        let resp = Response::new("hello");
        assert_eq!(resp.to_string(), String::from("$5\r\nhello\r\n"));
    }
}
