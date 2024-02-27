use std::sync::{Mutex, OnceLock};

type Kv = Vec<MetaData>;

pub enum MetaData {
    // master (address, port) in string
    Master((String, String)),
}

pub struct Meta {
    storage: &'static Mutex<Kv>,
}

impl Meta {
    pub fn new() -> Self {
        static STORAGE: OnceLock<Mutex<Kv>> = OnceLock::new();
        Meta {
            storage: STORAGE.get_or_init(|| Mutex::new(vec![])),
        }
    }

    pub fn storage(&self) -> &'static Mutex<Kv> {
        self.storage
    }

    pub fn set(&self, data: MetaData) {
        match self.storage.lock() {
            Ok(mut storage) => storage.push(data),
            Err(_) => (),
        }
    }

    pub fn get(&self, key: &str) -> Option<MetaData> {
        match self.storage.lock() {
            Ok(storage) => match key {
                "master" => {
                    for t in storage.iter() {
                        match t {
                            MetaData::Master((l, p)) => {
                                return Some(MetaData::Master((l.to_string(), p.to_string())))
                            }
                        }
                    }
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }
}
