use npm_rs::{NodeEnv, NpmEnv};

fn main() {
	// Run `npm run build frontend`
	NpmEnv::default()
		.with_node_env(&NodeEnv::Production)
		.init_env()
		.run("build frontend")
		.exec()
		.expect("Failed to execute npm run 'build frontend'");
}