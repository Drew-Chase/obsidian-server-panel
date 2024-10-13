use npm_rs::{NodeEnv, NpmEnv};
use std::fs;
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
        }
    }
    fs::create_dir_all("target/dev-env").expect("failed to create target directory");
    fs::create_dir_all("target/wwwroot").expect("failed to create wwwroot directory");
    println!("Building frontend...");
    // Run `npm run build frontend`
    NpmEnv::default()
        .with_node_env(&NodeEnv::Production)
        .init_env()
        .run("build frontend")
        .install(None)
        .exec()
        .expect("Failed to execute npm run 'build frontend'");
}
