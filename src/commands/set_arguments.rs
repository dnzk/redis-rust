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
    pub expiration: Option<SetExpiration>,
    pub condition: Option<SetCondition>,
    pub return_value: bool,
}

impl<'a> SetArguments {
    pub fn from(source: Vec<String>) -> Self {
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

#[cfg(test)]
mod set_arguments_tests {
    use super::*;
    use crate::vec_of_strings;

    #[test]
    fn parses_ex() {
        let args = SetArguments::from(vec_of_strings!["ex", "100"]);
        if let SetExpiration::Ex(ex) = args.expiration.unwrap() {
            assert_eq!(ex, 100);
        } else {
            panic!("Incorrect ex parsing");
        }
        assert!(args.condition.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_px() {
        let args = SetArguments::from(vec_of_strings!["px", "500"]);
        if let SetExpiration::Px(px) = args.expiration.unwrap() {
            assert_eq!(px, 500);
        } else {
            panic!("Incorrect px parsing");
        }
        assert!(args.condition.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_exat() {
        let args = SetArguments::from(vec_of_strings!["exat", "300"]);
        if let SetExpiration::ExAt(exat) = args.expiration.unwrap() {
            assert_eq!(exat, 300);
        } else {
            panic!("Incorect exat parsing");
        }
        assert!(args.condition.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_pxat() {
        let args = SetArguments::from(vec_of_strings!["pxat", "500"]);
        if let SetExpiration::PxAt(pxat) = args.expiration.unwrap() {
            assert_eq!(pxat, 500);
        } else {
            panic!("Incorrect pxat parsing");
        }
        assert!(args.condition.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_keepttl() {
        let args = SetArguments::from(vec_of_strings!["keepttl"]);
        assert!(matches!(args.expiration.unwrap(), SetExpiration::KeepTtl));
        assert!(args.condition.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_nx() {
        let args = SetArguments::from(vec_of_strings!["nx"]);
        assert!(matches!(args.condition.unwrap(), SetCondition::Nx));
        assert!(args.expiration.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_xx() {
        let args = SetArguments::from(vec_of_strings!["xx"]);
        assert!(matches!(args.condition.unwrap(), SetCondition::Xx));
        assert!(args.expiration.is_none());
        assert!(!args.return_value);
    }

    #[test]
    fn parses_get() {
        let args = SetArguments::from(vec_of_strings!["get"]);
        assert!(args.return_value);
        assert!(args.condition.is_none());
        assert!(args.expiration.is_none());
    }

    #[test]
    fn parses_multiple_args() {
        let args = SetArguments::from(vec_of_strings!["get", "pxat", "100", "nx"]);
        assert!(args.return_value);
        if let SetExpiration::PxAt(pxat) = args.expiration.unwrap() {
            assert_eq!(pxat, 100);
        } else {
            panic!("Incorrect pxat parsing");
        }
        assert!(matches!(args.condition.unwrap(), SetCondition::Nx));
    }

    #[test]
    fn case_insensitive() {
        let args = SetArguments::from(vec_of_strings!["GeT", "eX", "100", "NX"]);
        assert!(args.return_value);
        assert!(matches!(args.expiration.unwrap(), SetExpiration::Ex(_)));
        assert!(matches!(args.condition.unwrap(), SetCondition::Nx));
    }
}
