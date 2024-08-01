use dag_core::{Inputs, Outputs, Dag};
use risc0_zkvm::{
    guest::env,
    sha::Digest,
};
use sha2::{Digest as _, Sha256};
use dag_lib::encoders::{RawPBNode, RawPBLink, protobuf::encode_node};


fn main() {
    let inputs: Inputs = env::read();
    let parsed: Dag = serde_json::from_str(&inputs.data).unwrap();

    let node = RawPBNode {
        data: Some(parsed.data),
        links: parsed.links.iter().map(|link| RawPBLink {
            hash: Some(link.hash.clone()),
            name: Some(link.name.clone()),
            tsize: Some(link.tsize),
        }).collect(),
    };

    let encoded = encode_node(&node);
    let digest = Sha256::digest(&encoded);
    let digest = Digest::try_from(digest.as_slice()).unwrap();

    let out = Outputs {
        hash: digest,
    };
    env::commit(&out);
}
