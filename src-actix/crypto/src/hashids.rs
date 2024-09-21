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

use std::collections::VecDeque;

const MIN_ALPHABET_LENGTH: usize = 16;
const SEPARATOR_DIV: f32 = 3.5;
const GUARD_DIV: f32 = 12.0;
const DEFAULT_SEPARATORS: &str = "cfhistuCFHISTU";
const DEFAULT_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

/// Error container for various errors
#[derive(Debug)]
pub enum Error {
    AlphabetTooSmall,
    ContainsSpace,
    AlphabetNotUnique,
    InvalidHash,
    MissingLotteryChar,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AlphabetTooSmall => "Alphabet must contain at least 16 unique characters".fmt(f),
            Error::ContainsSpace => "Alphabet may not contain spaces".fmt(f),
            Error::AlphabetNotUnique => "Alphabet must contain unique characters".fmt(f),
            Error::InvalidHash => "Invalid hash provided".fmt(f),
            Error::MissingLotteryChar => "Hash is missing the lottery character".fmt(f),
        }
    }
}

impl std::error::Error for Error {}

/// Builder for `HashIds`
#[derive(Debug)]
pub struct HashIdsBuilder {
    salt: Vec<char>,
    min_length: usize,
}

impl HashIdsBuilder {
    /// Creates a new `HashIdsBuilder` instance with default values.
    pub fn new() -> Self {
        Self {
            salt: vec![],
            min_length: 0,
        }
    }
}

/// Same as `HashIdsBuilder`, but with custom alphabet (which can fail)
#[derive(Debug)]
pub struct HashIdsBuilderWithCustomAlphabet {
    inner: HashIdsBuilder,
    alphabet: Vec<char>,
}

impl HashIdsBuilderWithCustomAlphabet {
    /// Set the salt (arbitrary string) for the `HashIds`
    pub fn with_salt(mut self, salt: &str) -> Self {
        self.inner = self.inner.with_salt(salt);
        self
    }

    /// Set the minimum length for the encoded string
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.inner = self.inner.with_min_length(min_length);
        self
    }

    /// Convert the builder to the finished `HashIds`
    ///
    /// Can fail if the custom alphabet won't work
    pub fn finish(self) -> Result<HashIds, Error> {
        let Self {
            inner: HashIdsBuilder { salt, min_length },
            mut alphabet,
        } = self;

        let separators = DEFAULT_SEPARATORS
            .chars()
            .filter(|x| alphabet.contains(x))
            .collect::<Vec<_>>();

        alphabet = alphabet
            .drain(..)
            .filter(|x| !separators.contains(x))
            .collect();

        alphabet = alphabet
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(i, c)| alphabet.iter().position(|a| a == c) == Some(*i))
            .map(|(_, c)| c)
            .collect();

        if alphabet.len() + separators.len() < MIN_ALPHABET_LENGTH {
            return Err(Error::AlphabetTooSmall);
        }

        if alphabet.contains(&' ') {
            return Err(Error::ContainsSpace);
        }

        if alphabet.len()
            != alphabet
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
        {
            return Err(Error::AlphabetNotUnique);
        }

        Ok(HashIds {
            salt,
            min_length,
            alphabet,
            separators,
            guards: Vec::new(),
        }
        .finish())
    }
}

impl HashIdsBuilder {
    /// Set the salt (arbitrary string) for the `HashIds`
    pub fn with_salt(mut self, salt: &str) -> Self {
        self.salt = salt.chars().collect();
        self
    }

    /// Set the minimum length for the encoded string
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }

    /// Set the custom alphabet to use for encoding
    pub fn with_alphabet(self, alphabet: &str) -> HashIdsBuilderWithCustomAlphabet {
        HashIdsBuilderWithCustomAlphabet {
            inner: self,
            alphabet: alphabet.chars().collect(),
        }
    }

    /// Convert the builder to the finished `HashIds`
    pub fn finish(self) -> HashIds {
        let Self { salt, min_length } = self;
        HashIds {
            salt,
            min_length,
            alphabet: DEFAULT_ALPHABET
                .chars()
                .filter(|x| !DEFAULT_SEPARATORS.contains(*x))
                .collect(),
            separators: DEFAULT_SEPARATORS.chars().collect(),
            guards: Vec::new(),
        }
        .finish()
    }
}

/// The encoder/decoder container
#[derive(Debug, Clone)]
pub struct HashIds {
    salt: Vec<char>,
    min_length: usize,
    alphabet: Vec<char>,
    separators: Vec<char>,
    guards: Vec<char>,
}

impl HashIds {
    /// Create a new `HashIdsBuilder`
    pub fn builder() -> HashIdsBuilder {
        HashIdsBuilder::new()
    }

    /// Completes the building process and adjusts alphabet, separators, and guards based on salt.
    fn finish(mut self) -> Self {
        let min_separators = Self::index_from_ratio(self.alphabet.len(), SEPARATOR_DIV);

        if let Some(num_missing_separators) = min_separators.checked_sub(self.separators.len()) {
            if num_missing_separators > 0 {
                let mut new_alphabet = self.alphabet.split_off(num_missing_separators);
                std::mem::swap(&mut self.alphabet, &mut new_alphabet);
                self.separators.append(&mut new_alphabet);
            }
        }

        self.alphabet = Self::reorder(&self.alphabet, &self.salt);
        self.separators = Self::reorder(&self.separators, &self.salt);

        let num_guards = Self::index_from_ratio(self.alphabet.len(), GUARD_DIV);

        if self.alphabet.len() < 3 {
            self.guards = self.separators.split_off(num_guards);
            std::mem::swap(&mut self.separators, &mut self.guards);
        } else {
            self.guards = self.alphabet.split_off(num_guards);
            std::mem::swap(&mut self.alphabet, &mut self.guards);
        }

        self
    }

    /// Calculates an index for a given ratio.
    fn index_from_ratio(dividend: usize, divisor: f32) -> usize {
        (dividend as f32 / divisor).ceil() as _
    }

    /// Reorders characters in a string based on a salt value.
    fn reorder(string: &[char], salt: &[char]) -> Vec<char> {
        let mut out = string.to_vec();

        if salt.is_empty() {
            return out;
        }

        let mut int_sum = 0;
        let mut index = 0;

        for i in (1..string.len()).rev() {
            let int = u32::from(salt[index]) as usize;
            int_sum += int;
            let j = (int + index + int_sum) % i;
            out.swap(i, j);
            index = (index + 1) % salt.len();
        }

        out
    }

    /// Hashes a number using the given alphabet.
    fn hash(mut number: usize, alphabet: &[char]) -> Vec<char> {
        let mut hashed = VecDeque::new();
        loop {
            hashed.push_front(alphabet[number % alphabet.len()]);
            number /= alphabet.len();
            if number == 0 {
                break;
            }
        }
        hashed.into_iter().collect()
    }

    /// Unhashes characters back into a number using the given alphabet.
    fn unhash<I: Iterator<Item = char>>(hashed: I, alphabet: &[char]) -> Option<u64> {
        let mut number: u64 = 0;

        for c in hashed {
            let pos = alphabet.iter().position(|y| y == &c)? as u64;
            number *= alphabet.len() as u64;
            number += pos;
        }

        Some(number)
    }

    /// Splits a string based on given splitters.
    fn split<I: Iterator<Item = char>>(string: I, splitters: &[char]) -> Vec<Vec<char>> {
        let mut parts = Vec::new();
        let mut buf = Vec::new();
        for c in string {
            if splitters.contains(&c) {
                parts.push(buf);
                buf = Vec::new();
            } else {
                buf.push(c);
            }
        }
        parts.push(buf);
        parts
    }

    /// Encode a slice of numbers into a string
    pub fn encode(&self, vals: &[u64]) -> String {
        if vals.is_empty() {
            return String::new();
        }

        let mut alphabet = self.alphabet.clone();

        let vals_hash = vals
            .iter()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + ((*x as usize) % (i + 100)));

        let lottery = self.alphabet[vals_hash % self.alphabet.len()];
        let mut encoded = vec![lottery];

        for (i, val) in vals.iter().map(|x| *x as usize).enumerate() {
            let alphabet_salt = std::iter::once(lottery)
                .chain(self.salt.iter().copied())
                .chain(alphabet.iter().copied())
                .take(alphabet.len())
                .collect::<Vec<_>>();

            alphabet = Self::reorder(&alphabet, &alphabet_salt);
            let mut last = Self::hash(val, &alphabet);
            let val = val % (u32::from(last[0]) as usize + i);
            encoded.append(&mut last);
            encoded.push(self.separators[val % self.separators.len()]);
        }

        let _ = encoded.pop();

        if encoded.len() >= self.min_length {
            encoded.into_iter().collect::<String>()
        } else {
            let mut encoded = encoded.into_iter().collect::<VecDeque<_>>();

            let mut guard_index = (vals_hash + u32::from(encoded[0]) as usize) % self.guards.len();
            encoded.push_front(self.guards[guard_index]);

            if encoded.len() < self.min_length {
                guard_index = (vals_hash + u32::from(encoded[2]) as usize) % self.guards.len();
                encoded.push_back(self.guards[guard_index]);
            }

            let split_at = alphabet.len() / 2;

            while encoded.len() < self.min_length {
                alphabet = Self::reorder(&alphabet, &alphabet);

                for c in alphabet[split_at..].iter().copied().rev() {
                    encoded.push_front(c);
                }
                for c in alphabet[..split_at].iter().copied() {
                    encoded.push_back(c);
                }
                if let Some(excess) = encoded.len().checked_sub(self.min_length) {
                    if excess > 0 {
                        let from_index = excess / 2;
                        return encoded
                            .drain(from_index..from_index + self.min_length)
                            .collect::<String>();
                    }
                }
            }

            encoded.into_iter().collect::<String>()
        }
    }

    /// Decode a string into a `Vec` of numbers
    pub fn decode(&self, hash_str: &str) -> Result<Vec<u64>, Error> {
        if hash_str.is_empty() {
            return Ok(vec![]);
        }

        let mut alphabet = self.alphabet.clone();

        let mut parts = Self::split(hash_str.chars(), &self.guards);

        let mut hash_str = if parts.len() >= 2 && parts.len() <= 3 {
            parts.remove(1)
        } else {
            parts.remove(0)
        };

        let lottery = hash_str.get(0).ok_or(Error::MissingLotteryChar)?.clone();
        hash_str.remove(0);

        let parts = Self::split(hash_str.iter().copied(), &self.separators);

        let mut out = Vec::with_capacity(parts.len());

        for part in parts {
            let alphabet_salt = std::iter::once(lottery)
                .chain(self.salt.iter().copied())
                .chain(alphabet.iter().copied())
                .take(alphabet.len())
                .collect::<Vec<_>>();
            alphabet = Self::reorder(&alphabet, &alphabet_salt);

            if let Some(number) = Self::unhash(part.iter().copied(), &alphabet) {
                out.push(number)
            } else {
                return Err(Error::InvalidHash);
            }
        }

        Ok(out)
    }
}
