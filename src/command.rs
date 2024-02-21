use crate::resp::RespProtocol;

const PING: &str = "ping";
const ECHO: &str = "echo";

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
}

impl<'a> Command {
    pub fn from(source: &'a str) -> Self {
        let mut result = Command::Ping;
        match RespProtocol::from(source) {
            RespProtocol::Array(array) => match array.elements[0].to_lowercase().as_str() {
                PING => result = Command::Ping,
                ECHO => result = Command::Echo(array.elements[1..].join(" ")),
                _ => (),
            },
            _ => (),
        }
        result
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn from_ping() {
        let r = Command::from("");
        assert!(matches!(r, Command::Ping));
    }

    #[test]
    fn from_echo() {
        let c = Command::from("*2\r\n$4\r\necho\r\n$3\r\nhey\r\n");
        assert!(matches!(c, Command::Echo(_)));
    }

    #[test]
    fn case_insensitive() {
        let r = Command::from("*2\r\n$4\r\nEchO\r\n$3\r\nhey\r\n");
        assert!(matches!(r, Command::Echo(_)));
    }
}
