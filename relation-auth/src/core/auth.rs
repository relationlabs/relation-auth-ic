use libsecp256k1::{Message, Signature};
use tiny_keccak::{Hasher, Keccak};

use crate::config;
use crate::core::{ed25519, sr25519};
use crate::model::auth::{Algorithm, AuthRequest};
use crate::util;

const ETH_PUBLIC_KEY_PREFIX_UNCOMPRESSED: &str = "04";
const ETH_ADDRESS_PREFIX_UNCOMPRESSED: &str = "0x";
const ANONYMOUS_PRINCIPAL: &str = "2vxsx-fae";

pub fn verify_sign(req: &AuthRequest) -> Option<String> {
    if let Algorithm::None = req.algorithm {
        let p = ic_cdk::caller();
        ic_cdk::println!("ic address: {}", p.to_string());
        if ANONYMOUS_PRINCIPAL != p.to_string() {
            return Some(p.to_string());
        }
    }
    if !verify_expiration(req.message.parse().unwrap()) {
        return None;
    }
    match &req.algorithm {
        Algorithm::Sr25519 => {
            let decoded_pk = hex::decode(&req.pk).unwrap();
            let encode_signature = hex::decode(&req.decoded_signature).unwrap();
            let wrapped_message = format!("<Bytes>{}</Bytes>", &req.message);
            let r = sr25519::verify_with_sr25519(
                &decoded_pk,
                &wrapped_message.as_bytes().to_vec(),
                &encode_signature,
            );
            if r {
                return Some(req.pk.clone());
            }
        }
        Algorithm::Ed25519 => {
            let decoded_pk = hex::decode(&req.pk).unwrap();
            let encode_signature = hex::decode(&req.decoded_signature).unwrap();
            let r = ed25519::verify_with_ed25519(
                &decoded_pk,
                &req.message.as_bytes().to_vec(),
                &encode_signature,
            );
            if r {
                return Some(req.pk.clone());
            }
        }
        Algorithm::Secp256k1 => {
            let message = Message::parse_slice(&eth_message(&req.message)).unwrap();
            let signature = hex::decode(&req.decoded_signature).unwrap();
            ic_cdk::println!("signature len:{}", signature.len());
            let signature = Signature::parse_standard_slice(&signature[..64]).unwrap();
            let hx = hex::decode(&format!(
                "{}{}",
                ETH_PUBLIC_KEY_PREFIX_UNCOMPRESSED,
                req.pk.clone()
            ))
            .unwrap();
            ic_cdk::println!("pk len:{}", hx.as_slice().len());
            let public_key =
                libsecp256k1::PublicKey::parse_slice(&hx.as_slice()[1..], None).unwrap();
            let is_valid = libsecp256k1::verify(&message, &signature, &public_key);
            if is_valid {
                // use keccak-256 to calculate the hash of a public key
                let hash = keccak256(&public_key.serialize()[1..]);
                // keep only the last 20 bytes (least significant bytes) as address
                let address = format!(
                    "{}{}",
                    ETH_ADDRESS_PREFIX_UNCOMPRESSED,
                    hex::encode(&hash[12..]).to_lowercase()
                );
                ic_cdk::println!("eth address: {}", address);
                return Some(address);
            }
        }
        _ => {
            return None;
        }
    }
    None
}

fn verify_expiration(timestamp: u64) -> bool {
    let current = util::current_timestamp();
    ic_cdk::println!(
        "messagetime:{} current:{} EXPIRATION_EPOCH_SECS:{}",
        timestamp,
        current,
        config::WALLET_VERIFY_EXPIRATION_EPOCH_SECS_DEFAULT
    );
    current + config::WALLET_VERIFY_EXPIRATION_EPOCH_SECS_DEFAULT > timestamp
        && current - config::WALLET_VERIFY_EXPIRATION_EPOCH_SECS_DEFAULT < timestamp
}

pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut keccak256 = Keccak::v256();
    keccak256.update(bytes);
    keccak256.finalize(&mut output);
    output
}

fn eth_message(message: &String) -> [u8; 32] {
    let m = keccak256(
        format!(
            "{}{}{}",
            "\x19Ethereum Signed Message:\n",
            message.len(),
            message
        )
        .as_bytes(),
    );
    ic_cdk::println!("message: {}", hex::encode(m));
    m
}
