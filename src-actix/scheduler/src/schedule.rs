use crate::duration::{Duration, SystemTimeTrait};
use std::time::SystemTime;

pub struct Schedule {
    pub id: u64,
    pub duration: Duration,
    pub active: bool,
    reoccurring: bool,
    end_time: SystemTime,
    pub(crate) action: Box<dyn Fn(&Self) + Send + Sync>,
}

impl Schedule {
    pub fn new<F>(id: u64, duration: Duration, reoccurring: bool, action: F) -> Schedule
                  where
                      F: Fn(&Self) + 'static + Send + Sync,
    {
        Schedule {
            id,
            duration,
            active: true,
            reoccurring,
            end_time: duration.add_duration_to_now(),
            action: Box::new(action),
        }
    }

    pub fn get_end_time(&self) -> SystemTime {
        self.end_time
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.end_time = self.duration.add_duration_to_now();
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn tick(&mut self) {
        if self.active && SystemTime::now() >= self.end_time {
            (self.action)(self);
            self.active = self.reoccurring;
            if self.active {
                self.end_time = self.duration.add_duration_to_now();
            }
        }
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait EqualsId {
    fn equals_id(&self, id: u64) -> bool;
}

impl EqualsId for Schedule {
    fn equals_id(&self, id: u64) -> bool {
        self.id == id
    }
}