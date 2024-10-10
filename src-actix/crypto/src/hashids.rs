use crate::salt::get_salt;
use hash_ids::HashIds;
use log::debug;

/// Decodes a given hash string into a vector of `u64` integers.
///
/// # Arguments
///
/// * `hash` - A string slice that holds the hash to be decoded.
///
/// # Returns
///
/// A vector of `u64` integers that were encoded in the given hash string.
pub fn decode(hash: &str) -> Result<Vec<u64>, String> {
    let hash_ids = hashids();
    let decode = match hash_ids.decode(hash) {
        Ok(d) => d,
        Err(e) => return Err(format!("{}", e)),
    };
    debug!("Decoding: {} -> {:?}", hash, decode);
    Ok(decode)
}

/// Encodes a slice of `u64` integers into a hash string.
///
/// # Arguments
///
/// * `data` - A slice of `u64` integers to be encoded.
///
/// # Returns
///
/// A string that represents the encoded hash of the input data.
pub fn encode(data: &[u64]) -> String {
    let hash_ids = hashids();
    let encode = hash_ids.encode(data);
    debug!("Encoding: {:?} -> {}", data, encode);
    encode
}

fn hashids() -> HashIds {
    HashIds::builder()
        .with_salt(get_salt().as_str())
        .with_min_length(16)
        .finish()
}
