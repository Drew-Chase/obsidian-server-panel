use npm_rs::{NodeEnv, NpmEnv};
use std::fs;

fn main() {
	fs::create_dir_all("target/dev-env").expect("failed to create target directory");
	// Run `npm run build frontend`
	NpmEnv::default()
		.with_node_env(&NodeEnv::Production)
		.init_env()
		.run("build frontend")
		.exec()
		.expect("Failed to execute npm run 'build frontend'");
}