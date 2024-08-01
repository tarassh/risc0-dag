use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};
use dag_lib::decoders::cid::deserialize_cid;
use dag_lib::decoders::base64::deserialize_base64;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Outputs {
    pub hash: Digest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Inputs {
    pub data: String,
    pub root_cid: String
}


#[derive(Deserialize)]
pub struct Link {
    #[serde(rename = "Hash", deserialize_with = "deserialize_cid")]
    pub hash: Vec<u8>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Tsize")]
    pub tsize: u64,
}

#[derive(Deserialize)]
pub struct Dag {
    #[serde(rename = "Data", deserialize_with = "deserialize_base64")]
    pub data: Vec<u8>,
    #[serde(rename = "Links")]
    pub links: Vec<Link>,
}