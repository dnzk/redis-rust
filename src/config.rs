use std::net::SocketAddr;

const PORT: &str = "--port";
const REPLICA_OF: &str = "--replicaof";

const BASE: [u8; 4] = [127, 0, 0, 1];
const DEFAULT_PORT: u16 = 6379;

pub struct Config {
    port: Option<u16>,
    master: Option<(String, String)>,
}

impl<'a> Config {
    pub fn from(source: &'a Vec<String>) -> Self {
        let mut current = 0;
        let mut port = None;
        let mut master = None;
        while current < source.len() {
            match source[current].as_str() {
                PORT => {
                    if let Some(thing) = source.get(current + 1) {
                        let p = thing.parse::<u16>();
                        if let Ok(p) = p {
                            port = Some(p);
                            current += 2;
                        } else {
                            current += 1;
                        }
                    } else {
                        current += 1;
                    }
                }
                REPLICA_OF => {
                    let replica_params = &source[current + 1..current + 3];
                    current += 3;
                    master = Some((replica_params[0].to_owned(), replica_params[1].to_owned()));
                }
                _ => current += 1,
            }
        }
        Config { port, master }
    }

    pub fn address(&self) -> SocketAddr {
        if let Some(port) = self.port {
            return SocketAddr::from((BASE, port));
        }
        SocketAddr::from((BASE, DEFAULT_PORT))
    }

    pub fn master(&self) -> Option<(String, String)> {
        self.master.clone()
    }
}

#[cfg(test)]
mod config_tests {
    use std::net::Ipv4Addr;

    use super::*;
    use crate::vec_of_strings;

    #[test]
    fn parses_port() {
        let c = Config::from(&vec_of_strings!["program", "--port", "8080"]);
        assert_eq!(c.port.unwrap(), 8080);
    }

    #[test]
    fn generates_address_from_port() {
        let c = Config::from(&vec_of_strings!["program_name", "--port", "4000"]);
        if let SocketAddr::V4(address) = c.address() {
            assert_eq!(address.ip(), &Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(address.port(), 4000);
        } else {
            panic!("Error parsing address");
        }
    }

    #[test]
    fn generates_address_with_default_port() {
        let c = Config::from(&vec_of_strings!["program"]);
        if let SocketAddr::V4(address) = c.address() {
            assert_eq!(address.ip(), &Ipv4Addr::new(127, 0, 0, 1));
            assert_eq!(address.port(), 6379);
        } else {
            panic!("Error parsing address");
        }
    }
}
