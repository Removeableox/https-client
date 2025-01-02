use std::time::{SystemTime, UNIX_EPOCH};
use crate::https::{bigint256::BigInt256, rand::SimpleRng};

// Curve25519 constants
const MODULUS: [u8; 32] = [
    0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED
];
const BASEPOINT: [u8; 32] = [9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Default, Clone)]
pub struct Keys {
    pub public: BigInt256,
    pub private: BigInt256,
}

/// helper function for random_key()
fn random_num_from_sys_time() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_nanos() as u64 
}

fn random_key() -> BigInt256 {
    let mut key = BigInt256::new();
    let mut rng = SimpleRng::new(random_num_from_sys_time());

    for byte in key.iter_mut() {
        *byte = rng.next();
    }

    // Clamp the private key for Curve25519
    key[0] &= 0xF8;
    key[31] &= 0x7F;
    key[31] |= 0x40;

    println!("{:?}", key);

    key
}

/// Generate a public key from a private key using Curve25519
pub fn gen_public_key(private_key: BigInt256) -> BigInt256 {
    let basepoint = BigInt256::from(BASEPOINT);
    (private_key * basepoint) % BigInt256::from(MODULUS)
}

pub fn key_pair() -> Keys {
    let private_key = random_key();
    let public_key = gen_public_key(private_key.clone());
    Keys {
        private: private_key,
        public: public_key,
    }
}

#[cfg(test)]
mod tests {
    use super::random_key;

    #[test]
    fn rand() {
        random_key();
    }
}
