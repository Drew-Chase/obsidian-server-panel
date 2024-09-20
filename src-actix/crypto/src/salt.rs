use log::debug;
use rand::distr::Alphanumeric;
use rand::Rng;

fn generate_salt() -> String {
    debug!("Generating salt");
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
fn write_salt_to_file(salt: &str) -> std::io::Result<()> {
    debug!("Writing salt to file");
    std::fs::write(".salt", salt)
}
pub fn get_salt() -> String {
    std::fs::read_to_string(".salt").unwrap_or_else(|_| {
        let salt = generate_salt();
        write_salt_to_file(&salt).expect("Failed to write salt to file");
        salt
    })
}
