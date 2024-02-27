use crate::{Meta, MetaData};

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
        let mut info = String::new();
        for s in source {
            match s.as_str() {
                "replication" => {
                    if db.get("master").is_some() {
                        section = InfoSection::Replication(Replication::Slave);
                        info.push_str("role:slave");
                    } else {
                        section = InfoSection::Replication(Replication::Master);
                        info.push_str("role:master");
                    }
                    if let Some(MetaData::ReplicationId(id)) = db.get("replication_id") {
                        info.push_str(format!("\nmaster_replid:{}", id).as_str());
                    }
                    if let Some(MetaData::ReplicationOffset(o)) = db.get("replication_offset") {
                        info.push_str(format!("\nmaster_repl_offset:{}", o).as_str());
                    }
                }
                _ => (),
            }
        }
        Info { section, info }
    }

    pub fn info(&self) -> String {
        self.info.clone()
    }
}
