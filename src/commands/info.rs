/// Info sections per https://redis.io/commands/info/
#[derive(Debug)]
pub enum InfoSection {
    // Server,
    // Clients,
    // Memory,
    // Persistence,
    // Stats,
    Replication,
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
    pub fn from(source: &Vec<String>) -> Self {
        let mut section = InfoSection::Default;
        for s in source {
            match s.as_str() {
                "replication" => section = InfoSection::Replication,
                _ => (),
            }
        }
        Info {
            section,
            info: "role:master".to_string(),
        }
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }
}
