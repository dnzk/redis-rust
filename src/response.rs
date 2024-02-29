use crate::Command;

#[derive(Debug)]
pub struct Response {
    reply: String,
}

impl<'a> Response {
    pub fn from(command: &'a Command) -> Self {
        match command {
            Command::Echo(s) => Response {
                reply: format!("${}\r\n{}\r\n", s.len(), s),
            },
            Command::Set(_k, value) => {
                let reply = match value {
                    Some(s) => format!("${}\r\n{}\r\n", s.len(), s),
                    None => "+OK\r\n".to_string(),
                };
                Response { reply }
            }
            Command::Get(_k, v) => Response {
                reply: match v {
                    Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                    None => "$-1\r\n".to_string(),
                },
            },
            Command::Info(info) => Response {
                reply: format!("${}\r\n{}\r\n", info.len(), info),
            },
            Command::Ping => Response {
                reply: "$4\r\nPONG\r\n".to_string(),
            },
            Command::ReplConf => Response {
                reply: "+OK\r\n".to_string(),
            },
            Command::Psync(id) => Response {
                reply: format!("+FULLRESYNC {} 0\r\n", id),
            },
        }
    }

    pub fn buf(&self) -> Vec<u8> {
        self.reply.clone().into_bytes()
    }
}

#[cfg(test)]
mod response_tests {
    use super::*;
    use crate::Command;

    #[test]
    fn replies_echo() {
        let r = Response::from(&Command::Echo("hi".to_string()));
        assert_eq!(r.reply, "$2\r\nhi\r\n");
    }

    #[test]
    fn replies_set() {
        let r = Response::from(&Command::Set("foo".to_string(), Some("bar".to_string())));
        assert_eq!(r.reply, "$3\r\nbar\r\n");
    }

    #[test]
    fn replies_set_none() {
        let r = Response::from(&Command::Set("foo".to_string(), None));
        assert_eq!(r.reply, "+OK\r\n");
    }

    #[test]
    fn replies_get() {
        let r = Response::from(&Command::Get("foo".to_string(), Some("bar".to_string())));
        assert_eq!(r.reply, "$3\r\nbar\r\n");
    }

    #[test]
    fn replies_get_when_none() {
        let r = Response::from(&Command::Get("foo".to_string(), None));
        assert_eq!(r.reply, "$-1\r\n");
    }

    #[test]
    fn replies_ping() {
        let r = Response::from(&Command::Ping);
        assert_eq!(r.reply, "$4\r\nPONG\r\n");
    }
}
