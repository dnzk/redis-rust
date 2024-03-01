const EMPTY_HEX: &str = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";

pub struct Rdb;

impl Rdb {
    pub fn empty() -> Vec<u8> {
        match decode_hex(EMPTY_HEX) {
            Ok(mut decoded) => {
                let mut head = format!("${}\r\n", decoded.len()).as_bytes().to_owned();
                head.append(&mut decoded);
                head
            }
            _ => panic!("Error while decoding"),
        }
    }
}

fn decode_hex(s: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
