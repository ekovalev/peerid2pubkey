#![cfg_attr(not(feature = "std"), no_std)]

use libp2p::identity::{ed25519, PublicKey};
use multihash::Multihash;
use std::{env, fs};

pub mod util;

#[derive(Debug)]
pub struct Ed25519PublicKey(ed25519::PublicKey);

impl core::fmt::Display for Ed25519PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{}", hex::encode(&self.0.encode().to_vec()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Error {
    Base58DecodeError,
    MultihashParseError,
    DecodeFromProtobuf,
    InvalidPublicKey,
    InvalidInput,
}

fn to_public_key(encoded: String) -> Result<ed25519::PublicKey, Error> {
    let as_hex = String::from_utf8(encoded.into_bytes())
        .map_err(|_| Error::InvalidInput)?
        .into_bytes();
    let decoded = bs58::decode(&as_hex)
        .into_vec()
        .map_err(|_| Error::Base58DecodeError)?;
    let multihash = Multihash::from_bytes(&decoded).map_err(|_| Error::MultihashParseError)?;
    let pk = PublicKey::from_protobuf_encoding(multihash.digest())
        .map_err(|_| Error::DecodeFromProtobuf)?;
    match pk {
        PublicKey::Ed25519(k) => Ok(k),
        _ => Err(Error::InvalidPublicKey),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    for v in &args[1..] {
        match fs::read_to_string(v) {
            Ok(contents) => {
                // Try to treat the arg is a filename
                contents.lines().for_each(|s| {
                    if let Ok(pk) = to_public_key(s.to_string()) {
                        println!("{}", Ed25519PublicKey(pk));
                    } else {
                        println!("invalid input");
                    };
                })
            }
            _ => {
                // Treat the argument as individual peer ID
                if let Ok(pk) = to_public_key(v.to_string()) {
                    println!("{}", Ed25519PublicKey(pk));
                } else {
                    println!("invalid input");
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;

    #[test]
    fn manual_key_restoration() {
        let bytes = b"12D3KooWFafgzRvxrqYnyjdcYGSmNpNyDFry4SuCWmMp6XNsfL3X";
        let a = bs58::decode(bytes).into_vec().unwrap();
        let mh = Multihash::from_bytes(&a).unwrap();
        let dg = mh.digest();
        let pk = PublicKey::from_protobuf_encoding(dg).unwrap();
        match pk {
            PublicKey::Ed25519(k) => println!("pk: {}", Ed25519PublicKey(k)),
            _ => println!("Unsupported key type"),
        };
    }

    #[test]
    fn key_conversion_works() {
        for _ in 0..100 {
            let public_key = Keypair::generate_ed25519().public();
            let peer_id = public_key.to_peer_id();
            let encoded = peer_id.to_base58();

            let pk = to_public_key(encoded).unwrap();

            assert_eq!(public_key, PublicKey::Ed25519(pk.clone()));
            
            println!("original_pk: {:?}\npeer_id: {}\nderived_pk: {}\n", public_key, peer_id.to_base58(), Ed25519PublicKey(pk));
        }
    }
}
