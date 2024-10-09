use crate::duration::Duration;
use crate::schedule::Schedule;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCHEDULE_MANAGER_SINGLETON: Mutex<ScheduleManager> =
        Mutex::new(ScheduleManager::new());
}
pub struct ScheduleManager {
    schedules: Vec<Schedule>,
    pub next_id: u64,
    pub ticking: bool,
}

impl Default for ScheduleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ScheduleManager {
    pub fn new() -> ScheduleManager {
        ScheduleManager {
            schedules: Vec::new(),
            next_id: 0,
            ticking: false,
        }
    }

    pub fn add_schedule(
        &mut self,
        duration: Duration,
        reoccurring: bool,
        action: fn(&Schedule),
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let schedule = Schedule::new(id, duration, reoccurring, action);

        self.schedules.push(schedule);
        id
    }

    pub fn remove_schedule(&mut self, filter: fn(&Schedule) -> bool) {
        self.schedules.retain(|schedule| !filter(schedule));
    }

    pub fn tick(&mut self) {
        println!("Ticking schedules: {:?}", self.schedules.len());
        for schedule in self.schedules.iter_mut() {
            schedule.tick();
        }
    }
}

// Macro definitions
#[macro_export]
macro_rules! add_schedule {
    ($duration:expr, $reoccurring:expr, $action:expr) => {
        $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
            .lock()
            .unwrap()
            .add_schedule($duration, $reoccurring, $action)
    };
}

#[macro_export]
macro_rules! remove_schedule {
    ($filter:expr) => {
        $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
            .lock()
            .unwrap()
            .remove_schedule($filter)
    };
}

#[macro_export]
macro_rules! start_ticking_schedules {
    () => {
        $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
            .lock()
            .unwrap()
            .ticking = true;
        std::thread::spawn(move || {
            while $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
                .lock()
                .unwrap()
                .ticking
            {
                $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
                    .lock()
                    .unwrap()
                    .tick();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    };
}

#[macro_export]
macro_rules! stop_ticking_schedules {
    () => {
        $crate::schedule_manager::SCHEDULE_MANAGER_SINGLETON
            .lock()
            .unwrap()
            .ticking = false;
    };
}
