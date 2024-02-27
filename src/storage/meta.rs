use std::sync::{Mutex, OnceLock};

type Kv = Vec<MetaData>;

pub enum MetaData {
    // master (address, port) in string
    Master((String, String)),
    ReplicationId(String),
    ReplicationOffset(i32),
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
                "master" => storage.iter().find_map(|d| match d {
                    MetaData::Master((l, p)) => Some(MetaData::Master((l.clone(), p.clone()))),
                    _ => None,
                }),
                "replication_id" => storage.iter().find_map(|d| match d {
                    MetaData::ReplicationId(id) => Some(MetaData::ReplicationId(id.clone())),
                    _ => None,
                }),
                "replication_offset" => storage.iter().find_map(|d| match d {
                    MetaData::ReplicationOffset(o) => Some(MetaData::ReplicationOffset(o.clone())),
                    _ => None,
                }),
                _ => None,
            },
            _ => None,
        }
    }
}
