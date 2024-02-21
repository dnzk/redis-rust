use crate::command::Command;

pub struct Request {
    command: Command,
}

impl<'a> Request {
    pub fn from(buf: &'a [u8]) -> Self {
        let mut v: Vec<u8> = vec![];
        let zero: u8 = 0;
        for b in buf.into_iter() {
            if *b != zero {
                v.push(*b);
            }
        }
        let source = String::from_utf8(v).unwrap_or("".to_string());
        Request {
            command: Command::from(&source),
        }
    }

    pub fn command(self) -> Command {
        self.command
    }
}
