use std::{env};
use std::process::Command;

fn main() {
	// Run `npm run build frontend`
	let status = Command::new("npm")
		.arg("run")
		.arg("build frontend")
		.current_dir(env::current_dir().unwrap())
		.status()
		.expect("Failed to execute npm 'build frontend'");

	if !status.success() {
		panic!("npm 'build frontend' failed!");
	}
}