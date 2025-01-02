use std::time::{SystemTime, UNIX_EPOCH};
use crate::https::bigint256::BigInt256;

// curve 25519 constants 
const MODULUS: [u8;32] = [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED];
const BASEPOINT: u8 = 9;

#[derive(Default)]
pub struct Keys {
    public: BigInt256,
    private: BigInt256, 
}

/// helper function for random_key()
fn random_number() -> u8 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_nanos() as u8 % 0xFF
}

/// used for generating 32 byte private keys
fn random_key() -> BigInt256 {
    let mut random = BigInt256::new();
    for i in 0..32 {
        random[i] = random_number();
    }
    random[0] = 0x0;
    random
}

/// generate public key from a private key using curve 25519
pub fn gen_public_key(private_key: BigInt256) -> BigInt256 {
    private_key * BigInt256::from_u8(BASEPOINT) % BigInt256::from(MODULUS)
} 

pub fn key_pair() -> Keys {
    let private_key = random_key();
    let public_key = gen_public_key(private_key.clone());
    return Keys {
        private: private_key,
        public: public_key,
        ..Default::default()
    }
}

