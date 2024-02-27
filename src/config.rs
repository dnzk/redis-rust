use std::net::SocketAddr;

const PORT: &str = "--port";

const BASE: [u8; 4] = [127, 0, 0, 1];
const DEFAULT_PORT: u16 = 6379;

pub struct Config {
    port: Option<u16>,
}

impl<'a> Config {
    pub fn from(source: &'a Vec<String>) -> Self {
        let mut current = 0;
        let mut port = None;
        for s in source {
            match s.as_str() {
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
                _ => current += 1,
            }
        }
        Config { port }
    }

    pub fn address(&self) -> SocketAddr {
        if let Some(port) = self.port {
            return SocketAddr::from((BASE, port));
        }
        SocketAddr::from((BASE, DEFAULT_PORT))
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
