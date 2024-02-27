mod kv_store;
mod meta;

pub use kv_store::KvStore;
pub use meta::{Meta, MetaData};

pub struct Storage {
    pub kv_store: KvStore,
    pub meta: Meta,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            kv_store: KvStore::new(),
            meta: Meta::new(),
        }
    }
}
