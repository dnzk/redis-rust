use std::sync::{Mutex, MutexGuard, OnceLock};

type Kv = Vec<(String, String)>;

pub struct KvStore {
    db: &'static Mutex<Kv>,
}

impl KvStore {
    pub fn new() -> Self {
        static KV: OnceLock<Mutex<Kv>> = OnceLock::new();
        KvStore {
            db: KV.get_or_init(|| Mutex::new(vec![])),
        }
    }

    pub fn db(&self) -> &'static Mutex<Kv> {
        self.db
    }

    pub fn set(&self, key: String, value: String) {
        match self.db.lock() {
            Ok(mut db) => {
                if let Some((index, value)) = find(&db, &key) {
                    db[index] = (key, value);
                } else {
                    db.push((key, value));
                }
            }
            Err(_) => {}
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.db.lock() {
            Ok(db) => {
                if let Some((_, value)) = find(&db, &key) {
                    return Some(value);
                }
                None
            }
            _ => None,
        }
    }

    pub fn remove(db: &Mutex<Kv>, key: &str) {
        if let Ok(mut db) = db.lock() {
            for (i, (k, _)) in db.clone().iter().enumerate() {
                if k == key {
                    db.remove(i);
                }
            }
        }
    }
}

fn find(db: &MutexGuard<'static, Kv>, key: &str) -> Option<(usize, String)> {
    for (i, kv) in db.iter().enumerate() {
        if &kv.0 == key {
            return Some((i, kv.1.to_string()));
        }
    }
    None
}
