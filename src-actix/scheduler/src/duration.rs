use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::time::{Duration as StdDuration, SystemTime};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Duration {
	pub seconds: u64,
	pub minutes: u64,
	pub hours: u64,
	pub days: u64,
}

impl Duration {
	pub fn new() -> Duration {
		Duration {
			seconds: 0,
			minutes: 0,
			hours: 0,
			days: 0,
		}
	}
	pub fn set_seconds(&mut self, seconds: u64) {
		self.seconds = seconds;
	}
	pub fn set_minutes(&mut self, minutes: u64) {
		self.minutes = minutes;
	}
	pub fn set_hours(&mut self, hours: u64) {
		self.hours = hours;
	}
	pub fn set_days(&mut self, days: u64) {
		self.days = days;
	}
	pub fn get_seconds(&self) -> u64 {
		self.seconds
	}
	pub fn get_minutes(&self) -> u64 {
		self.minutes
	}
	pub fn get_hours(&self) -> u64 {
		self.hours
	}
	pub fn get_days(&self) -> u64 {
		self.days
	}
	pub fn add_seconds(&mut self, seconds: u64) {
		self.seconds += seconds;
	}
	pub fn add_minutes(&mut self, minutes: u64) {
		self.minutes += minutes;
	}
	pub fn add_hours(&mut self, hours: u64) {
		self.hours += hours;
	}
	pub fn add_days(&mut self, days: u64) {
		self.days += days;
	}
}

impl Add for Duration {
	type Output = Duration;

	fn add(self, other: Duration) -> Duration {
		let mut result = Duration::new();
		result.add_seconds(self.seconds + other.seconds);
		result.add_minutes(self.minutes + other.minutes);
		result.add_hours(self.hours + other.hours);
		result.add_days(self.days + other.days);
		result
	}
}

impl AddAssign for Duration {
	fn add_assign(&mut self, other: Duration) {
		self.add_seconds(other.seconds);
		self.add_minutes(other.minutes);
		self.add_hours(other.hours);
		self.add_days(other.days);
	}
}

impl Sub for Duration {
	type Output = Duration;

	fn sub(self, other: Duration) -> Duration {
		let mut result = Duration::new();
		result.add_seconds(self.seconds - other.seconds);
		result.add_minutes(self.minutes - other.minutes);
		result.add_hours(self.hours - other.hours);
		result.add_days(self.days - other.days);
		result
	}
}

impl Mul for Duration {
	type Output = Duration;

	fn mul(self, other: Duration) -> Duration {
		let mut result = Duration::new();
		result.add_seconds(self.seconds * other.seconds);
		result.add_minutes(self.minutes * other.minutes);
		result.add_hours(self.hours * other.hours);
		result.add_days(self.days * other.days);
		result
	}
}

impl Div for Duration {
	type Output = Duration;

	fn div(self, other: Duration) -> Duration {
		let mut result = Duration::new();
		result.add_seconds(self.seconds / other.seconds);
		result.add_minutes(self.minutes / other.minutes);
		result.add_hours(self.hours / other.hours);
		result.add_days(self.days / other.days);
		result
	}
}

impl PartialEq for Duration {
	fn eq(&self, other: &Duration) -> bool {
		self.seconds == other.seconds
			&& self.minutes == other.minutes
			&& self.hours == other.hours
			&& self.days == other.days
	}
}

impl Display for Duration {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{} days, {} hours, {} minutes, {} seconds",
			self.days, self.hours, self.minutes, self.seconds
		)
	}
}

pub trait SystemTimeTrait {
	fn from_system_time(system_time: SystemTime) -> Self;
	fn to_system_time(&self) -> SystemTime;
}

impl SystemTimeTrait for Duration {
	fn from_system_time(system_time: SystemTime) -> Duration {
		let duration = system_time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
		let total_seconds = duration.as_secs();
		let days = total_seconds / 86400;
		let hours = (total_seconds % 86400) / 3600;
		let minutes = (total_seconds % 3600) / 60;
		let seconds = total_seconds % 60;
		Duration {
			seconds,
			minutes,
			hours,
			days,
		}
	}

	fn to_system_time(&self) -> SystemTime {
		let total_seconds =
			self.seconds + self.minutes * 60 + self.hours * 3600 + self.days * 86400;
		SystemTime::now() + StdDuration::from_secs(total_seconds)
	}
}
