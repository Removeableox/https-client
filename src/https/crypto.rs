use std::{iter::Rev, time::{SystemTime, UNIX_EPOCH}, u128};

fn generate_random_number() -> u8 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_nanos() as u8 % 0xFF
}

type ByteArray = Vec<u8>;
type Range = Vec<usize>;

fn random_byte_array(size: u8) -> ByteArray {
    let mut random = Vec::new();
    for _ in 0..size {
        random.push(generate_random_number());
    }
    random[0] = 0x0;
    random
}


/// more optimized solution to (a..b).rev().collect()
fn range(mut a: usize, b: usize, step: usize) -> Range {
    let mut range: Vec<usize> = Vec::new();
    
    while a != b {
        range.push(a);
        a += step;
    }

    range 
}

fn mul_byte_arrays(a: &ByteArray, b: &ByteArray) -> ByteArray {
    let size = a.len();
    for i in (0..size).rev().collect() {
        
    }
    vec![0]
}

pub fn gen_public_key(private_key: ByteArray) -> ByteArray {
    // 2^255 - 19 as a byte array (curve25519 modulus)
    let modulus_lower = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xED];

    let modulus_higher = vec![0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; 

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
    use super::{gen_public_key, mul_byte_arrays, random_byte_array, ByteArray};

    /// just for testing with small byte_arrays
    fn byte_array_eval(arr: ByteArray) -> u128 {
        let size = arr.len();
        let exponents: ByteArray = (0..size as u8).rev().collect();
        let mut decimal_value: u128 = 0;
        for i in 0..size {
            let byte = arr[i];
            let exponent = exponents[i];
            decimal_value += (byte as u128) * 256u128.pow(exponent as u32);
        }
        decimal_value
    }

    #[test]
    fn mul() {
        assert_eq!(byte_array_eval(mul_byte_arrays(&vec![0,0,100], &vec![0,0,3])), 300); 
    }
}
