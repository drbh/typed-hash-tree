use digest::Digest;

// use blake2::Blake2b;
use sha2::Sha256;

// use base64::{decode, encode};
use hex::{decode, encode};

fn hash<D: Digest>(data: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.input(data.as_bytes());
    output.copy_from_slice(hasher.result().as_slice())
}

fn hash_lr<D: Digest>(left: &[u8; 32], right: &[u8; 32], output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.input(left);
    hasher.input(right);
    output.copy_from_slice(hasher.result().as_slice())
}

fn get_parents(leaves: Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut nodes = Vec::new();
    for i in (0..&leaves.len() - 1).step_by(2) {
        let _left = &leaves[i];
        let _right = &leaves[i + 1];
        let mut buf: [u8; 32] = [0; 32];
        hash_lr::<Sha256>(_left, _right, &mut buf);
        nodes.push(buf.clone())
    }
    if &leaves.len() % 2 != 0 {
        nodes.push(*leaves.last().unwrap());
    }
    nodes
}

fn main() {
    let data = vec![
        "david", "richard", "blyn", "holtz", "tree", "worm", "ness", "dog", "cat", "rat",
    ];

    let mut levels = Vec::new();
    let mut leaves = Vec::new();

    for _data in &data {
        let mut buf: [u8; 32] = [0; 32];
        hash::<Sha256>(_data, &mut buf);
        leaves.push(buf)
    }

    levels.push(leaves.clone());
    while &leaves.len() > &1 {
        leaves = get_parents(leaves);
        levels.push(leaves.clone());
    }

    println!("{:?}", data);
    for lvl in levels {
        println!("");
        for lf in lvl {
            println!("{:?}", encode(lf));
        }
    }
}
