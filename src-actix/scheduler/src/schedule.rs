use crate::duration::{Duration, SystemTimeTrait};
use std::time::SystemTime;

pub struct Schedule {
    pub id: u64,
    pub duration: Duration,
    pub active: bool,
    reoccurring: bool,
    end_time: SystemTime,
    action: fn(),
}

impl Schedule {
    pub fn new(id: u64, duration: Duration, reoccurring:bool, action: fn()) -> Schedule {
        Schedule {
            id,
            duration: Duration::new(),
            active: true,
            reoccurring,
            end_time: SystemTime::now(),
            action,
        }
    }

    pub fn get_end_time(&self) -> SystemTime {
        self.end_time
    }
    
    pub fn activate(&mut self) {
        self.active = true;
        self.end_time = self.duration.to_system_time();
    }
    
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    
    
    pub fn tick(&mut self) {
        if self.active && SystemTime::now() >= self.end_time {
            (self.action)();
            self.active = self.reoccurring;
            if self.active {
                self.end_time = self.duration.to_system_time();
            }
        }
    }
}
