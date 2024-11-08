use std::fs;

fn main() {
    fs::create_dir_all("target/wwwroot").unwrap();
    fs::create_dir_all("target/dev-env").unwrap();
}
