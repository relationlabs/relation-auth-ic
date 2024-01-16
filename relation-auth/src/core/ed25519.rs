extern crate ed25519_dalek;

use ed25519_dalek::*;

pub type PK = PublicKey;

pub fn verify_with_ed25519(
    decoded_pk: &Vec<u8>,
    decoded_message: &Vec<u8>,
    decoded_signature: &Vec<u8>,
) -> bool {
    let pk = match PK::from_bytes(&decoded_pk[..]) {
        Ok(pk) => pk,
        Err(e) => {
            ic_cdk::print(format!("{}", e));
            panic!("Provided public key is invalid.");
        }
    };

    let signature = match Signature::from_bytes(&decoded_signature[..]) {
        Ok(signature) => signature,
        Err(_) => return false,
    };
    pk.verify(&decoded_message[..], &signature).is_ok()
}
