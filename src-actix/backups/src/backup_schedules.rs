use crate::backup_item::BackupType;
use std::time::{Duration, SystemTime};

pub struct BackupSchedule {
    pub id: u32,
    pub server: u32,
    pub backup_type: BackupType,
    pub interval: u32,
    pub exec_if_empty: bool,
    pub exec_if_offline: bool,
    pub last_exec: Option<SystemTime>,
    pub next_exec: Option<SystemTime>,
}
impl BackupSchedule {
    pub fn new(
        id: u32,
        server: u32,
        backup_type: BackupType,
        interval: u32,
        exec_if_empty: bool,
        exec_if_offline: bool,
    ) -> Self {
        Self {
            id,
            server,
            backup_type,
            interval,
            exec_if_empty,
            exec_if_offline,
            last_exec: None,
            next_exec: None,
        }
    }
    pub fn update_next_exec(&mut self) {
        let now = SystemTime::now();
        self.next_exec = Some(if let Some(last_exec) = self.last_exec {
            last_exec + Duration::from_secs(self.interval as u64 * 60)
        } else {
            now + Duration::from_secs(self.interval as u64 * 60)
        });
    }

    pub async fn execute(&mut self) {
        //		BackupItem::create_backup();
        //		self.last_exec = Some(SystemTime::now());
        //		self.update_next_exec();
    }
}
