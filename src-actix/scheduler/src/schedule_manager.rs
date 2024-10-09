use crate::duration::Duration;
use crate::schedule::Schedule;
use std::ops::{Add, AddAssign, Sub};

pub struct ScheduleManager {
	schedules: Vec<Schedule>,
	next_id: u64,
}

impl ScheduleManager {
	pub fn new() -> ScheduleManager {
		ScheduleManager {
			schedules: Vec::new(),
			next_id: 0,
		}
	}

	pub fn add_schedule(&mut self, duration: Duration, reoccurring: bool, action: fn()) -> u64 {
		let id = self.next_id;
		self.next_id += 1;
		let schedule = Schedule::new(id, duration, reoccurring, action);

		self.schedules.push(schedule);
		id
	}

	pub fn remove_schedule(&mut self, id: u64) {
		self.schedules.retain(|schedule| schedule.id != id);
	}

	pub fn tick(&mut self) {
		for schedule in self.schedules.iter_mut() {
			schedule.tick();
		}
	}
}

impl Add for ScheduleManager {
	type Output = ScheduleManager;

	fn add(&mut self, other: Schedule) -> &ScheduleManager {
		self.schedules.push(other);
		self
	}
}

impl Sub for ScheduleManager {
	type Output = ScheduleManager;

	fn sub(&mut self, other: Schedule) -> &ScheduleManager {
		self.schedules.retain(|schedule| schedule.id != other.id);
		self
	}
}

impl AddAssign<Schedule> for ScheduleManager {
	fn add_assign(&mut self, other: Schedule) {
		self.schedules.push(other);
	}
}