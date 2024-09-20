use crate::salt::get_salt;
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
pub fn decode(hash: &str) -> Vec<u64> {
    let hash_ids = hashids();
    let decode = hash_ids.decode(hash);
    debug!("Decoding: {} -> {:?}", hash, decode);
    decode
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

fn hashids() -> hash_ids::HashIds {
    hash_ids::HashIds::builder()
        .with_salt(get_salt().as_str())
        .with_min_length(16)
        .finish()
}
