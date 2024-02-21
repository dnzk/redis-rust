use super::array::Array;

pub enum RespProtocol {
    Array(Array),
    SimpleError,
}

impl<'a> RespProtocol {
    pub fn from(source: &'a str) -> Self {
        let mut resp_type = RespProtocol::SimpleError;
        if let Some(data_type) = source.chars().nth(0) {
            match data_type {
                '*' => resp_type = RespProtocol::Array(Array::from(&source[1..])),
                _ => (),
            }
        }
        resp_type
    }
}

#[cfg(test)]
mod resp_protocol_tests {
    use super::*;

    #[test]
    fn creates_array_type() {
        let resp = RespProtocol::from("*2\r\n$4\r\necho\r\n$3\r\nhey\r\n");
        assert!(matches!(resp, RespProtocol::Array(_)));
    }
}
