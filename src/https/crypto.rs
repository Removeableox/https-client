use std::time::{SystemTime, UNIX_EPOCH};

type Key = [u8;32];

const RANGE: Key = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31];
const DRANGE: Key = [31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

const MODULUS: Key = [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED];
const BASEPOINT: u8 = 9;

#[derive(Default)]
pub struct Keys {
    public: Key,
    private: Key, 
}


/// helper function for random_key()
fn random_number() -> u8 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_nanos() as u8 % 0xFF
}

/// used for generating 32 byte private keys
fn random_key() -> Key {
    let mut random = [0u8; 32]; 
    for i in 0..32 {
        random[i] = random_number();
    }
    random[0] = 0x0;
    random
}

fn key_mul(a: &mut Key, b: u8) {
    let mut carry: u8 = 0;
    for i in DRANGE {
        let value: u128 = (a[i as usize] + carry) as u128 * b as u128;
        a[i as usize] = (value % 255) as u8;
        carry = (value / 255) as u8;
    }
} 


pub fn gen_public_key(mut private_key: Key) -> Key {
    key_mul(&mut private_key, BASEPOINT);
    let public_key = mod_keys(private_key, MODULUS);

    public_key 
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

#[cfg(test)]
mod tests {
    use crate::https::crypto::mod_keys;

    use super::random_key;

    #[test]
    fn modulus() {
        let key = random_key(); 
        println!("{:?}", key);
        let modulus = random_key();
        println!("{:?}", modulus);
        let res = mod_keys(key, modulus);
        println!("{:?}", res);
    }
}
