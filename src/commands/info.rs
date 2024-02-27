use crate::Meta;

#[derive(Debug)]
pub enum Replication {
    Master,
    Slave,
}

/// Info sections per https://redis.io/commands/info/
#[derive(Debug)]
pub enum InfoSection {
    // Server,
    // Clients,
    // Memory,
    // Persistence,
    // Stats,
    Replication(Replication),
    // Cpu,
    // CommandStats,
    // LatencyStats,
    // Sentinel,
    // Cluster,
    // Modules,
    // KeySpace,
    // ErrorStats,
    // All,
    Default,
    // Everything,
}

#[derive(Debug)]
pub struct Info {
    section: InfoSection,
    info: String,
}

impl<'a> Info {
    pub fn from(source: &Vec<String>, db: &Meta) -> Self {
        let mut section = InfoSection::Default;
        for s in source {
            match s.as_str() {
                "replication" => {
                    if let Some(master) = db.get("master") {
                        section = InfoSection::Replication(Replication::Slave);
                    } else {
                        section = InfoSection::Replication(Replication::Master);
                    }
                }
                _ => (),
            }
        }
        let info = match &section {
            InfoSection::Replication(replication) => match replication {
                Replication::Master => "role:master".to_string(),
                Replication::Slave => "role:slave".to_string(),
            },
            _ => "".to_string(),
        };
        Info { section, info }
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }
}
