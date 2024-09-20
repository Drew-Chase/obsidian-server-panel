use crate::salt::get_salt;

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
    let hash_ids = hash_ids::HashIds::builder()
        .with_salt(get_salt().as_str())
        .finish();
    hash_ids.decode(hash)
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
    let hash_ids = hash_ids::HashIds::builder()
        .with_salt(get_salt().as_str())
        .finish();
    hash_ids.encode(data)
}
