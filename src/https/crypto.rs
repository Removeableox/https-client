use std::time::{SystemTime, UNIX_EPOCH};

type Key = [u8;32];

const RANGE: [u8;32] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31];
const DRANGE: [u8;32] = [31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0];

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

/// input (32 byte array) % modulus (u8) 
/// helper function for mod_keys()
fn key_mod(mut input: Key, modulus: u8) -> u8 {
    let mut carry: u32 = 0u32;
    for i in 0..32 {
        println!("current byte: {}", input[i]);
        println!("carry from prev: {}", carry);

        let value = input[i] as u32 + carry;
        // if the last byte
        if i == 31 {
            return value as u8 % modulus; 
        }
        // if not the last byte
        carry = (((value%modulus as u32) as f32 / modulus as f32) * 255f32) as u32;
    }
    0u8
}

/// a (32 byte array) % b (32 byte array)
fn mod_keys(a: Key, b: Key) -> Key {
    // do something
    
    [0u8; 32] // temp
}

pub fn gen_public_key(mut private_key: Key) -> Key {
    // 2^255 - 19 as a byte array (curve25519 modulus)
    let modulus = [0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED];
    
    // 9 is the curve 25519 base point
    key_mul(&mut private_key, 9u8);

    let public_key = mod_keys(private_key, modulus);

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
    use super::{key_mod, random_key};

    #[test]
    fn modulus() {
        let key = random_key(); 
        println!("{:?}", key);
        let modulus = 5;
        println!("{}", modulus);
        let res = key_mod(key, modulus);
        println!("{}", res);
    }
}
