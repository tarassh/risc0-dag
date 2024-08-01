
pub struct RawPBLink {
    pub hash: Option<Vec<u8>>,
    pub name: Option<String>,
    pub tsize: Option<u64>,
}


pub struct RawPBNode {
    pub data: Option<Vec<u8>>,
    pub links: Vec<RawPBLink>,
}

pub mod protobuf;
