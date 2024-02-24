use crate::commands::Set;
use crate::resp::RespProtocol;
use crate::KvStore;

const PING: &str = "ping";
const ECHO: &str = "echo";
const SET: &str = "set";
const GET: &str = "get";

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, Option<String>),
    Get(String, Option<String>),
}

impl<'a> Command {
    pub fn from(source: &'a str, db: &KvStore) -> Self {
        let mut result = Command::Ping;
        match RespProtocol::from(source) {
            RespProtocol::Array(array) => match array.elements[0].to_lowercase().as_str() {
                PING => result = Command::Ping,
                ECHO => result = Command::Echo(array.elements[1..].join(" ")),
                SET => {
                    let (key, value) = Set::save(&array.elements[1..].to_vec(), db);
                    result = Command::Set(key, value);
                }
                GET => {
                    let value = db.get(&array.elements[1]);
                    result = Command::Get(array.elements[1].to_string(), value);
                }
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
        let r = Command::from("", &KvStore::new());
        assert!(matches!(r, Command::Ping));
    }

    #[test]
    fn from_echo() {
        let c = Command::from("*2\r\n$4\r\necho\r\n$3\r\nhey\r\n", &KvStore::new());
        assert!(matches!(c, Command::Echo(_)));
    }

    #[test]
    fn case_insensitive() {
        let r = Command::from("*2\r\n$4\r\nEchO\r\n$3\r\nhey\r\n", &KvStore::new());
        assert!(matches!(r, Command::Echo(_)));
    }

    #[test]
    fn optionless_set_get() {
        let r = Command::from(
            "*3\r\n$3\r\nset\r\n$3\r\nfoo\r\n$3\r\nbar\r\n",
            &KvStore::new(),
        );
        assert!(matches!(r, Command::Set(_, _)));

        let r = Command::from("*2\r\n$3\r\nget\r\n$3\r\nfoo\r\n", &KvStore::new());
        if let Command::Get(_k, v) = r {
            assert_eq!("bar".to_string(), v.unwrap());
        } else {
            panic!("Incorrect Command");
        }
    }
}
