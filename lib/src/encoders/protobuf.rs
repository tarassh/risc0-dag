use crate::encoders::{RawPBLink, RawPBNode};

fn sov(x: i64) -> usize {
    let mut n = 0;
    let mut x = x;

    if x < 0 {
        x = -x;
    }

    while x >= 0x80 {
        x >>= 7;
        n += 1;
    }

    n + 1
}

fn encode_varint(bytes: &mut [u8], offset: usize, mut v: i64) -> usize {
    let mut offset = offset;
    offset -= sov(v);
    let base = offset;
    while v >= u32::MAX as i64 {
        bytes[offset] = (v & 0x7f | 0x80) as u8;
        v >>= 7;
        offset += 1;

        // bytes[offset++] = (v & 0x7f) | 0x80
        // v /= 128
    }

    while v >= 0x80 {
        bytes[offset] = ((v & 0x7f) | 0x80) as u8;
        v >>= 7;
        offset += 1;
    }
    bytes[offset] = (v & 0x7f) as u8;
    base
}

fn encode_link(link: &RawPBLink, bytes: &mut [u8]) -> usize {
    let mut i = bytes.len();

    if let Some(tsize) = link.tsize {
        if tsize > u64::MAX {
            panic!("Tsize too large for encoding");
        }
        i = encode_varint(bytes, i, tsize as i64) - 1;
        bytes[i] = 0x18;
    }

    if let Some(ref name) = link.name {
        let name_bytes = name.as_bytes();
        i -= name_bytes.len();
        bytes[i..(i + name_bytes.len())].copy_from_slice(name_bytes);
        i = encode_varint(bytes, i, name_bytes.len() as i64) - 1;
        bytes[i] = 0x12;
    }

    if let Some(ref hash) = link.hash {
        i -= hash.len();
        bytes[i..(i + hash.len())].copy_from_slice(hash);
        i = encode_varint(bytes, i, hash.len() as i64) - 1;
        bytes[i] = 0xa;
    }

    bytes.len() - i
}

pub fn encode_node(node: &RawPBNode) -> Vec<u8> {
    let size = size_node(node);
    let mut bytes = vec![0; size];
    let mut i = size;

    if let Some(ref data) = node.data {
        i -= data.len();
        bytes[i..(i + data.len())].copy_from_slice(data);
        i = encode_varint(&mut bytes, i, data.len() as i64) - 1;
        bytes[i] = 0xa;
    }

    for link in node.links.iter().rev() {
        let size = encode_link(link, &mut bytes[..i]);
        i -= size;
        i = encode_varint(&mut bytes, i, size as i64) - 1;
        bytes[i] = 0x12;
    }

    bytes
}

fn size_link(link: &RawPBLink) -> usize {
    let mut n = 0;

    if let Some(ref hash) = link.hash {
        let l = hash.len();
        n += 1 + l + sov(l as i64);
    }

    if let Some(ref name) = link.name {
        let l = name.as_bytes().len();
        n += 1 + l + sov(l as i64);
    }

    if let Some(tsize) = link.tsize {
        n += 1 + sov(tsize as i64);
    }

    n
}

fn size_node(node: &RawPBNode) -> usize {
    let mut n = 0;

    if let Some(ref data) = node.data {
        let l = data.len();
        n += 1 + l + sov(l as i64);
    }

    for link in &node.links {
        let l = size_link(link);
        n += 1 + l + sov(l as i64);
    }

    n
}