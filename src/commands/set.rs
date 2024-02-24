use crate::KvStore;
use std::{thread, time};

#[derive(Debug)]
pub enum SetExpiration {
    Ex(usize),
    Px(usize),
    ExAt(usize),
    PxAt(usize),
    KeepTtl,
}

#[derive(Debug)]
pub enum SetCondition {
    Nx,
    Xx,
}

#[derive(Debug)]
pub struct SetArguments {
    expiration: Option<SetExpiration>,
    condition: Option<SetCondition>,
    return_value: bool,
}

impl<'a> SetArguments {
    fn from(source: Vec<String>) -> Self {
        let mut expiration = None;
        let mut condition = None;
        let mut return_value = false;

        let mut current = 0;

        for s in source.iter() {
            match s.to_lowercase().as_str() {
                "ex" => {
                    if let Some(next) = source.get(current + 1) {
                        if let Ok(ex) = next.parse::<usize>() {
                            expiration = Some(SetExpiration::Ex(ex));
                            current += 2;
                        }
                    } else {
                        expiration = None;
                        current += 1;
                    }
                }
                "px" => {
                    if let Some(next) = source.get(current + 1) {
                        if let Ok(px) = next.parse::<usize>() {
                            expiration = Some(SetExpiration::Px(px));
                            current += 2;
                        }
                    } else {
                        expiration = None;
                        current += 1;
                    }
                }
                "exat" => {
                    if let Some(next) = source.get(current + 1) {
                        if let Ok(exat) = next.parse::<usize>() {
                            expiration = Some(SetExpiration::ExAt(exat));
                            current += 2;
                        }
                    } else {
                        expiration = None;
                        current += 1;
                    }
                }
                "pxat" => {
                    if let Some(next) = source.get(current + 1) {
                        if let Ok(pxat) = next.parse::<usize>() {
                            expiration = Some(SetExpiration::PxAt(pxat));
                            current += 2;
                        }
                    } else {
                        expiration = None;
                        current += 1;
                    }
                }
                "keepttl" => {
                    expiration = Some(SetExpiration::KeepTtl);
                    current += 1;
                }
                "nx" => {
                    condition = Some(SetCondition::Nx);
                    current += 1;
                }
                "xx" => {
                    condition = Some(SetCondition::Xx);
                    current += 1;
                }
                "get" => {
                    return_value = true;
                    current += 1;
                }
                _ => current += 1,
            }
        }

        SetArguments {
            expiration,
            condition,
            return_value,
        }
    }
}

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
        if let Some(arguments) = s.arguments.as_ref() {
            if let Some(condition) = &arguments.condition {
                match condition {
                    SetCondition::Nx => {
                        if db.get(&s.key).is_none() {
                            db.set(s.key(), s.value.clone().unwrap());
                        }
                    }
                    SetCondition::Xx => {
                        if db.get(&s.key).is_some() {
                            previous_value = db.get(&s.key);
                            db.set(s.key(), s.value.clone().unwrap());
                        }
                    }
                }
            } else {
                db.set(s.key(), s.value.clone().unwrap());
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
        db.set(s.key(), s.value.as_ref().unwrap().clone());
        (s.key().clone(), None)
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }
}
