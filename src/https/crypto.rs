use std::{iter::Rev, time::{SystemTime, UNIX_EPOCH}, u128};

fn generate_random_number() -> u8 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_nanos() as u8 % 0xFF
}

type ByteArray = Vec<u8>;

fn random_byte_array(size: u8) -> ByteArray {
    let mut random = Vec::new();
    for _ in 0..size {
        random.push(generate_random_number());
    }
    random[0] = 0x0;
    random
}

/// if an "override_exponents" value is given it will overide the standard  
/// decrementing exponent for byte array calc:
///
/// byte0 * 256^{size-1}....lastbyte * 256^{0}
///
/// in this context this is used for evaluating byte arrays in multiple sections
/// because rust cannot store a 32 byte int so we make two 16 byte arrays and eval
/// them (31 through 16) and (15 through 0) respectively 
fn byte_array_eval(arr: ByteArray, override_exponents: Option<Vec<u8>>) -> u128 {
    let size = arr.len();
    let exponents: Vec<u8> = match override_exponents {
        Some(override_exp) => override_exp,
        None => (0..size as u8).rev().collect(),
    };

    let mut decimal_value: u128 = 0;
    for (i, &byte) in arr.iter().enumerate() {
        let exponent = exponents[i];
        decimal_value += (byte as u128) * 256u128.pow(exponent as u32);
    }
    decimal_value
}

pub fn gen_public_key(private_key: ByteArray) -> ByteArray {
    // 2^255 - 19 as a byte array (curve25519 modulus)
    let modulus_lower = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED];
    let modulus_higher = vec![0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; 
    let higher_exponents = (0..32 as u8).rev().collect();
    println!("{}", byte_array_eval(modulus_higher, Some(higher_exponents)));

    modulus_lower
} 

pub fn key_pair() -> Keys {
    let private_key = random_byte_array(32);
    let public_key = gen_public_key(private_key.clone());
    return Keys {
        private: private_key,
        public: public_key,
        ..Default::default()
    }
}

#[derive(Default)]
pub struct Keys {
    public: Vec<u8>,
    private: Vec<u8>,
    server_public: Vec<u8>,
    handshake_secret: Vec<u8>,
    client_handshake_secret: Vec<u8>,
    client_handshake_key: Vec<u8>,
    server_handshake_key: Vec<u8>,
    client_handshake_iv: Vec<u8>,
    server_handshake_iv: Vec<u8>,

    client_app_key: Vec<u8>,
    client_app_iv: Vec<u8>,
    server_app_key: Vec<u8>,
    server_app_iv: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::{gen_public_key, random_byte_array};

    #[test]
    fn calc_modulus() {
        gen_public_key(random_byte_array(32));
    }
}
