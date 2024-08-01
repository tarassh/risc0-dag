use serde::Deserialize;
use serde::de::{self, Deserializer};

use super::base32::decode_base32;
use super::base58::decode_base58;


fn cid_decoder(source: &str) -> Result<Vec<u8>, &'static str> {
    let prefix = source.chars().next().ok_or("Invalid CID: Empty input")?;
    match prefix {
        'Q' => decode_base58(source),
        'z' => decode_base58(&source[1..]),
        'b' => decode_base32(&source[1..]),
        _ => Err("Unsupported CID prefix"),
    }
}

pub fn deserialize_cid<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    cid_decoder(&s).map_err(de::Error::custom)
}