use super::set_arguments::{SetArguments, SetCondition, SetExpiration};
use crate::KvStore;
use std::{thread, time};

#[derive(Debug)]
pub struct Set {
    key: String,
    value: Option<String>,
    arguments: Option<SetArguments>,
}

impl<'a> Set {
    pub fn from(args: &'a Vec<String>) -> Self {
        let mut arguments = None;
        if args.len() > 2 {
            arguments = Some(SetArguments::from(args[2..].to_vec()))
        }
        Set {
            key: args[0].to_string(),
            value: Some(args[1].to_string()),
            arguments,
        }
    }

    pub fn save(args: &'a Vec<String>, db: &KvStore) -> (String, Option<String>) {
        let s = Self::from(args);
        let mut previous_value: Option<String> = None;
        let value = s.value.as_ref().unwrap();
        if let Some(arguments) = s.arguments.as_ref() {
            if let Some(condition) = &arguments.condition {
                match condition {
                    SetCondition::Nx => {
                        if db.get(&s.key).is_none() {
                            db.set(s.key(), value.to_owned());
                        }
                    }
                    SetCondition::Xx => {
                        if db.get(&s.key).is_some() {
                            previous_value = db.get(&s.key);
                            db.set(s.key(), value.to_owned());
                        }
                    }
                }
            } else {
                db.set(s.key(), value.to_owned());
            }

            if let Some(expiration) = &arguments.expiration {
                match expiration {
                    SetExpiration::Ex(_ex) => {
                        unimplemented!();
                    }
                    SetExpiration::Px(px) => {
                        let px = *px as u64;
                        let key = s.key.clone();
                        let db = db.db();
                        thread::spawn(move || {
                            thread::sleep(time::Duration::from_millis(px));
                            KvStore::remove(db, &key);
                        });
                    }
                    SetExpiration::ExAt(_exat) => {
                        unimplemented!();
                    }
                    SetExpiration::PxAt(_pxat) => {
                        unimplemented!();
                    }
                    SetExpiration::KeepTtl => {
                        unimplemented!();
                    }
                }
            }

            if arguments.return_value {
                return (s.key().clone(), previous_value);
            }
            return (s.key().clone(), None);
        }
        db.set(s.key(), value.to_owned());
        (s.key().clone(), None)
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}
