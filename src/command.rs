use crate::commands::{Info, Set};
use crate::resp::RespProtocol;
use crate::{MetaData, Storage};

const PING: &str = "ping";
const ECHO: &str = "echo";
const SET: &str = "set";
const GET: &str = "get";
const INFO: &str = "info";
const REPLCONF: &str = "replconf";
const PSYNC: &str = "psync";

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, Option<String>),
    Get(String, Option<String>),
    Info(String),
    ReplConf,
    Psync(String),
}

impl<'a> Command {
    pub fn from(source: &'a str, db: &Storage) -> Self {
        let mut result = Command::Ping;
        match RespProtocol::from(source) {
            RespProtocol::Array(array) => match array.elements[0].to_lowercase().as_str() {
                PING => result = Command::Ping,
                ECHO => result = Command::Echo(array.elements[1..].join(" ")),
                SET => {
                    let (key, value) = Set::save(&array.elements[1..].to_vec(), &db.kv_store);
                    result = Command::Set(key, value);
                }
                GET => {
                    let value = db.kv_store.get(&array.elements[1]);
                    result = Command::Get(array.elements[1].to_string(), value);
                }
                INFO => {
                    let info = Info::from(&array.elements[1..].to_vec(), &db.meta);
                    result = Command::Info(info.info());
                }
                REPLCONF => {
                    result = Command::ReplConf;
                }
                PSYNC => {
                    if let MetaData::ReplicationId(id) = db.meta.get("replication_id").unwrap() {
                        result = Command::Psync(id);
                    } else {
                        panic!("Missing replication id");
                    }
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
        let r = Command::from("", &Storage::new());
        assert!(matches!(r, Command::Ping));
    }

    #[test]
    fn from_echo() {
        let c = Command::from("*2\r\n$4\r\necho\r\n$3\r\nhey\r\n", &Storage::new());
        assert!(matches!(c, Command::Echo(_)));
    }

    #[test]
    fn case_insensitive() {
        let r = Command::from("*2\r\n$4\r\nEchO\r\n$3\r\nhey\r\n", &Storage::new());
        assert!(matches!(r, Command::Echo(_)));
    }

    #[test]
    fn optionless_set_get() {
        let r = Command::from(
            "*3\r\n$3\r\nset\r\n$3\r\nfoo\r\n$3\r\nbar\r\n",
            &Storage::new(),
        );
        assert!(matches!(r, Command::Set(_, _)));

        let r = Command::from("*2\r\n$3\r\nget\r\n$3\r\nfoo\r\n", &Storage::new());
        if let Command::Get(_k, v) = r {
            assert_eq!("bar".to_string(), v.unwrap());
        } else {
            panic!("Incorrect Command");
        }
    }
}
